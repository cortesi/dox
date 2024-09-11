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

    println!("Number of fields: {}", fields.len());

    let field_docs: Vec<_> = fields
        .iter()
        .filter_map(|field| {
            let name = field.ident.as_ref()?;
            let docs = extract_doc_comments(&field.attrs);
            let ty = &field.ty;
            println!(
                "Field: {}, Type: {:?}, Docs: {:?}",
                name,
                quote!(#ty).to_string(),
                docs
            );
            let name_str = name.to_string();
            let type_str = quote!(#ty).to_string();
            Some(quote! {
                (#name_str.to_string(), #type_str.to_string(), #docs.to_string())
            })
        })
        .collect();

    println!("Number of fields: {}", field_docs.len());

    let expanded = quote! {
        impl libdox::Dox for #name {
            fn dox_fields() -> Vec<(String, String, String)> {
                vec![
                    #(#field_docs),*
                ]
            }
        }
    };

    println!("Expanded macro: {}", expanded);

    TokenStream::from(expanded)
}
