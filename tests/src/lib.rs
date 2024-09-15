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
    use libdox::{render, Container, DocType, Field, Text};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_simple_render() {
        let result = render::<TestStruct, _>(&Text);
        assert_eq!(
            result,
            indoc! {"
                This is a test struct
                test_field (String): This is a test field
                nested (InnerStruct): This is a nested struct
                vec_field (Vec < i32 >): This is a vector of integers
            "}
            .trim()
        );
    }

    #[test]
    fn test_dox() {
        let expected = DocType::Container(Container {
            name: "TestStruct".to_string(),
            fields: vec![
                Field {
                    name: "test_field".to_string(),
                    typ: "String".to_string(),
                    doc: "This is a test field".to_string(),
                },
                Field {
                    name: "nested".to_string(),
                    typ: "InnerStruct".to_string(),
                    doc: "This is a nested struct".to_string(),
                },
                Field {
                    name: "vec_field".to_string(),
                    typ: "Vec < i32 >".to_string(),
                    doc: "This is a vector of integers".to_string(),
                },
            ],
            doc: "This is a test struct".to_string(),
        });

        assert_eq!(TestStruct::dox(), expected);
    }
}
