use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Fields, FieldsNamed};

fn extract_doc_comments(attrs: &[Attribute]) -> String {
    attrs
        .iter()
        .filter(|attr| attr.path().is_ident("doc"))
        .filter_map(|attr| {
            attr.meta.require_name_value().ok().and_then(|meta| {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(lit_str),
                    ..
                }) = &meta.value
                {
                    Some(lit_str.value().trim().to_string())
                } else {
                    None
                }
            })
        })
        .collect::<Vec<String>>()
        .join("\n")
}

#[proc_macro_derive(Dox)]
pub fn dox_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(FieldsNamed { named, .. }) => named,
            _ => return TokenStream::new(),
        },
        _ => return TokenStream::new(),
    };

    let field_docs: Vec<_> = fields
        .iter()
        .map(|field| {
            let name = field.ident.as_ref().unwrap();
            let docs = extract_doc_comments(&field.attrs);
            let ty = &field.ty;
            let name_str = name.to_string();
            let type_str = quote!(#ty).to_string();
            quote! {
                libdox::FieldDoc {
                    name: #name_str.to_string(),
                    typ: #type_str.to_string(),
                    doc: #docs.to_string(),
                }
            }
        })
        .collect();

    let expanded = quote! {
        impl libdox::Dox for #name {
            fn dox_fields() -> Vec<libdox::FieldDoc> {
                vec![
                    #(#field_docs),*
                ]
            }
        }
    };
    TokenStream::from(expanded)
}
