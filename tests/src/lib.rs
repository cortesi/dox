#![allow(dead_code)]

use libdox::Dox;

#[derive(Dox)]
struct InnerStruct {
    /// This is an inner field
    inner_field: i32,
}

/// This is a test struct
#[derive(Dox)]
struct TestStruct {
    /// This is a test field
    test_field: String,
    /// This is a nested struct
    nested: InnerStruct,
    /// This is a vector of integers
    vec_field: Vec<i32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use libdox::{render, render::Text, Container, Field, Primitive, Typ};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_simple_render() {
        let result = render::<TestStruct, _>(&Text);
        assert_eq!(
            result,
            indoc! {"
                This is a test struct
                  test_field (String): This is a test field
                This is a nested struct
                    inner_field (i32): This is an inner field
                  vec_field (Vec<i32>): This is a vector of integers
            "}
            .trim()
        );
    }

    #[test]
    fn test_dox() {
        let expected = Field::Container(Container {
            name: "TestStruct".to_string(),
            fields: vec![
                Field::Primitive(Primitive {
                    name: "test_field".to_string(),
                    typ: Typ::String,
                    doc: "This is a test field".to_string(),
                }),
                Field::Container(Container {
                    name: "nested".to_string(),
                    fields: vec![Field::Primitive(Primitive {
                        name: "inner_field".to_string(),
                        typ: Typ::I32,
                        doc: "This is an inner field".to_string(),
                    })],
                    doc: "This is a nested struct".to_string(),
                }),
                Field::Primitive(Primitive {
                    name: "vec_field".to_string(),
                    typ: Typ::Vec(Box::new(Typ::I32)),
                    doc: "This is a vector of integers".to_string(),
                }),
            ],
            doc: "This is a test struct".to_string(),
        });

        assert_eq!(TestStruct::dox(), expected);
    }
}
