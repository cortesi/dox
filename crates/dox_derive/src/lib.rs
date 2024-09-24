use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, Attribute, Data, DeriveInput, Expr, ExprLit, Fields, FieldsNamed, Lit, Meta,
};

fn extract_doc_comments(attrs: &[Attribute]) -> String {
    attrs
        .iter()
        .filter(|attr| attr.path().is_ident("doc"))
        .filter_map(|attr| {
            if let Meta::NameValue(meta) = attr.meta.clone() {
                if let Expr::Lit(ExprLit {
                    lit: Lit::Str(s), ..
                }) = meta.value
                {
                    Some(s.value().trim().to_string())
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
            attr.parse_args_with(|input: syn::parse::ParseStream| {
                let mut rename = None;
                while !input.is_empty() {
                    let meta: syn::Meta = input.parse()?;
                    if let syn::Meta::NameValue(name_value) = meta {
                        if name_value.path.is_ident("rename") {
                            if let Expr::Lit(ExprLit {
                                lit: Lit::Str(lit_str),
                                ..
                            }) = name_value.value
                            {
                                rename = Some(lit_str.value());
                            }
                        }
                    }
                    if !input.is_empty() {
                        input.parse::<syn::Token![,]>()?;
                    }
                }
                Ok(rename)
            })
            .ok()
            .flatten()
        } else {
            None
        }
    })
}

fn extract_serde_rename_all(attrs: &[Attribute]) -> Option<String> {
    attrs.iter().find_map(|attr| {
        if attr.path().is_ident("serde") {
            attr.parse_args_with(|input: syn::parse::ParseStream| {
                let mut rename_all = None;
                while !input.is_empty() {
                    let meta: syn::Meta = input.parse()?;
                    if let syn::Meta::NameValue(name_value) = meta {
                        if name_value.path.is_ident("rename_all") {
                            if let Expr::Lit(ExprLit {
                                lit: Lit::Str(lit_str),
                                ..
                            }) = name_value.value
                            {
                                rename_all = Some(lit_str.value());
                            }
                        }
                    }
                    if !input.is_empty() {
                        input.parse::<syn::Token![,]>()?;
                    }
                }
                Ok(rename_all)
            })
            .ok()
            .flatten()
        } else {
            None
        }
    })
}

fn rename_field(name: &str, rename_rule: &str) -> String {
    match rename_rule {
        "lowercase" => name.to_lowercase(),
        "UPPERCASE" => name.to_uppercase(),
        "PascalCase" => name
            .split('_')
            .map(|word| {
                let mut c = word.chars();
                match c.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                }
            })
            .collect(),
        "camelCase" => {
            let pascal = rename_field(name, "PascalCase");
            pascal[..1].to_lowercase() + &pascal[1..]
        }
        "snake_case" => name.to_lowercase().replace(' ', "_"),
        "SCREAMING_SNAKE_CASE" => name.to_uppercase().replace(' ', "_"),
        "kebab-case" => name.to_lowercase().replace('_', "-"),
        "SCREAMING-KEBAB-CASE" => name.to_uppercase().replace('_', "-"),
        _ => name.to_string(),
    }
}

fn process_field(field: &syn::Field, rename_all: &Option<String>) -> proc_macro2::TokenStream {
    let name = field.ident.as_ref().unwrap();
    let docs = extract_doc_comments(&field.attrs);
    let ty = &field.ty;
    let name_str = extract_serde_rename(&field.attrs)
        .or_else(|| {
            rename_all
                .as_ref()
                .map(|rule| rename_field(&name.to_string(), rule))
        })
        .unwrap_or_else(|| name.to_string());

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

    let rename_all = extract_serde_rename_all(&input.attrs);
    let field_docs: Vec<_> = fields
        .iter()
        .map(|f| process_field(f, &rename_all))
        .collect();

    let struct_docs = extract_doc_comments(&input.attrs);
    let name_str = name.to_string();

    let expanded = quote! {
        impl libdox::Dox for #name {
            fn dox() -> libdox::Field {
                libdox::Field::Container(libdox::Container {
                    name: #name_str.to_string(),
                    type_name: stringify!(#name).to_string(),
                    fields: vec![
                        #(#field_docs),*
                    ],
                    doc: #struct_docs.to_string(),
                })
            }
        }
    };
    TokenStream::from(expanded)
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_extract_serde_rename() {
        let item: syn::ItemStruct = parse_quote! {
            #[serde(rename = "new_name")]
            struct Test;
        };
        assert_eq!(
            extract_serde_rename(&item.attrs),
            Some("new_name".to_string())
        );

        let item: syn::ItemStruct = parse_quote! {
            #[derive(Debug)]
            struct Test;
        };
        assert_eq!(extract_serde_rename(&item.attrs), None);
    }

    #[test]
    fn test_extract_serde_rename_all() {
        let item: syn::ItemStruct = parse_quote! {
            #[serde(rename_all = "camelCase")]
            struct Test;
        };
        assert_eq!(
            extract_serde_rename_all(&item.attrs),
            Some("camelCase".to_string())
        );

        let item: syn::ItemStruct = parse_quote! {
            #[serde(rename_all = "snake_case")]
            struct Test;
        };
        assert_eq!(
            extract_serde_rename_all(&item.attrs),
            Some("snake_case".to_string())
        );

        let item: syn::ItemStruct = parse_quote! {
            #[derive(Debug)]
            struct Test;
        };
        assert_eq!(extract_serde_rename_all(&item.attrs), None);
    }

    #[test]
    fn test_extract_doc_comments() {
        let item: syn::ItemStruct = parse_quote! {
            /// This is a doc comment
            /// It spans multiple lines
            #[derive(Debug)]
            struct Test;
        };
        assert_eq!(
            extract_doc_comments(&item.attrs),
            "This is a doc comment\nIt spans multiple lines"
        );

        let item: syn::ItemStruct = parse_quote! {
            #[derive(Debug)]
            struct Test;
        };
        assert_eq!(extract_doc_comments(&item.attrs), "");
    }

    #[test]
    fn test_rename_field() {
        assert_eq!(rename_field("test_field", "lowercase"), "test_field");
        assert_eq!(rename_field("test_field", "UPPERCASE"), "TEST_FIELD");
        assert_eq!(rename_field("test_field", "PascalCase"), "TestField");
        assert_eq!(rename_field("test_field", "camelCase"), "testField");
        assert_eq!(rename_field("test_field", "snake_case"), "test_field");
        assert_eq!(
            rename_field("test_field", "SCREAMING_SNAKE_CASE"),
            "TEST_FIELD"
        );
        assert_eq!(rename_field("test_field", "kebab-case"), "test-field");
        assert_eq!(
            rename_field("test_field", "SCREAMING-KEBAB-CASE"),
            "TEST-FIELD"
        );
    }
}
