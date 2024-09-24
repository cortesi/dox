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

#[cfg(test)]
mod tests {
    use super::*;
    use libdox::{Container, Field, Primitive, Typ};
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
}
