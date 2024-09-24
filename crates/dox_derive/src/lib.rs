use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Fields, FieldsNamed, Meta};

fn extract_doc_comments(attrs: &[Attribute]) -> String {
    attrs
        .iter()
        .filter(|attr| attr.path().is_ident("doc"))
        .filter_map(|attr| {
            if let Meta::NameValue(meta) = &attr.meta {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(lit_str),
                    ..
                }) = &meta.value
                {
                    Some(lit_str.value().trim().to_string())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn extract_serde_rename(attrs: &[Attribute]) -> Option<String> {
    attrs.iter().find_map(|attr| {
        if attr.path().is_ident("serde") {
            if let Meta::List(meta_list) = &attr.meta {
                for nested_meta in meta_list.tokens.clone().into_iter() {
                    let token_stream = proc_macro2::TokenStream::from(nested_meta);
                    if let Ok(Meta::NameValue(name_value)) = syn::parse2::<Meta>(token_stream) {
                        if name_value.path.is_ident("rename") {
                            if let syn::Expr::Lit(syn::ExprLit {
                                lit: syn::Lit::Str(lit_str),
                                ..
                            }) = name_value.value
                            {
                                return Some(lit_str.value());
                            }
                        }
                    }
                }
            }
        }
        None
    })
}

fn process_field(field: &syn::Field) -> proc_macro2::TokenStream {
    let name = field.ident.as_ref().unwrap();
    let docs = extract_doc_comments(&field.attrs);
    let ty = &field.ty;
    let name_str = extract_serde_rename(&field.attrs).unwrap_or_else(|| name.to_string());

    quote! {
        {
            let mut field = <#ty as libdox::Dox>::dox();
            match &mut field {
                libdox::Field::Container(container) => {
                    container.name = #name_str.to_string();
                    container.doc = #docs.to_string();
                },
                libdox::Field::Primitive(primitive) => {
                    primitive.name = #name_str.to_string();
                    primitive.doc = #docs.to_string();
                },
            }
            field
        }
    }
}

#[proc_macro_derive(Dox)]
pub fn dox_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(FieldsNamed { named, .. }) => named,
            _ => {
                return syn::Error::new_spanned(&input, "Only named fields are supported")
                    .to_compile_error()
                    .into()
            }
        },
        _ => {
            return syn::Error::new_spanned(&input, "Only structs are supported")
                .to_compile_error()
                .into()
        }
    };

    let field_docs: Vec<_> = fields.iter().map(process_field).collect();

    let struct_docs = extract_doc_comments(&input.attrs);
    let name_str = name.to_string();

    let expanded = quote! {
        impl libdox::Dox for #name {
            fn dox() -> libdox::Field {
                libdox::Field::Container(libdox::Container {
                    name: #name_str.to_string(),
                    fields: vec![
                        #(#field_docs),*
                    ],
                    doc: #struct_docs.to_string(),
                    original_name: stringify!(#name).to_string(),
                })
            }
        }
    };
    TokenStream::from(expanded)
}
