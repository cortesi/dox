#![allow(dead_code)]

use libdox::Dox;
use serde::Serialize;

#[derive(Dox, Serialize)]
struct InnerStruct {
    /// This is an inner field
    #[serde(rename = "inner")]
    inner: i32,
}

/// This is a test struct
#[derive(Dox, Serialize)]
struct TestStruct {
    /// This is a test field
    #[serde(rename = "test")]
    test: String,
    /// This is a nested struct
    nested: InnerStruct,
    /// This is a vector of integers
    #[serde(rename = "vector")]
    vector: Vec<i32>,
}

/// This is a struct with snake_case rename
#[derive(Dox, Serialize)]
#[serde(rename_all = "snake_case")]
struct SnakeCaseStruct {
    /// This is a camel case field
    camel_case_field: String,
}

/// This is a struct with camelCase rename
#[derive(Dox, Serialize)]
#[serde(rename_all = "camelCase")]
struct CamelCaseStruct {
    /// This is a snake case field
    snake_case_field: String,
}

/// This is a test enum
#[derive(Dox, Serialize)]
enum TestEnum {
    Variant1,
    Variant2,
    Variant3,
}

#[cfg(test)]
mod tests {
    use super::*;
    use libdox::{Container, Enum, Field, Primitive, Typ};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_dox() {
        let expected = Field::Container(Container {
            name: "TestStruct".to_string(),
            type_name: "TestStruct".to_string(),
            fields: vec![
                Field::Primitive(Primitive {
                    name: "test".to_string(),
                    typ: Typ::String,
                    doc: "This is a test field".to_string(),
                }),
                Field::Container(Container {
                    name: "nested".to_string(),
                    type_name: "InnerStruct".to_string(),
                    fields: vec![Field::Primitive(Primitive {
                        name: "inner".to_string(),
                        typ: Typ::I32,
                        doc: "This is an inner field".to_string(),
                    })],
                    doc: "This is a nested struct".to_string(),
                }),
                Field::Primitive(Primitive {
                    name: "vector".to_string(),
                    typ: Typ::Vec(Box::new(Typ::I32)),
                    doc: "This is a vector of integers".to_string(),
                }),
            ],
            doc: "This is a test struct".to_string(),
        });

        assert_eq!(TestStruct::dox(), expected);
    }

    #[test]
    fn test_snake_case_rename() {
        let expected = Field::Container(Container {
            name: "SnakeCaseStruct".to_string(),
            type_name: "SnakeCaseStruct".to_string(),
            fields: vec![Field::Primitive(Primitive {
                name: "camel_case_field".to_string(),
                typ: Typ::String,
                doc: "This is a camel case field".to_string(),
            })],
            doc: "This is a struct with snake_case rename".to_string(),
        });

        assert_eq!(SnakeCaseStruct::dox(), expected);
    }

    #[test]
    fn test_camel_case_rename() {
        let expected = Field::Container(Container {
            name: "CamelCaseStruct".to_string(),
            type_name: "CamelCaseStruct".to_string(),
            fields: vec![Field::Primitive(Primitive {
                name: "snakeCaseField".to_string(),
                typ: Typ::String,
                doc: "This is a snake case field".to_string(),
            })],
            doc: "This is a struct with camelCase rename".to_string(),
        });

        assert_eq!(CamelCaseStruct::dox(), expected);
    }

    #[test]
    fn test_enum() {
        let expected = Field::Enum(Enum {
            name: "TestEnum".to_string(),
            doc: "This is a test enum".to_string(),
            variants: vec![
                "Variant1".to_string(),
                "Variant2".to_string(),
                "Variant3".to_string(),
            ],
        });

        assert_eq!(TestEnum::dox(), expected);
    }
}
