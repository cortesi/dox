#![allow(dead_code)]

use libdox::Dox;

#[derive(Dox)]
struct InnerStruct {
    /// This is an inner field
    inner_field: i32,
}

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
    use libdox::{render, DocType, Text};

    #[test]
    fn test_simple_render() {
        let result = render::<TestStruct, _>(&Text);
        assert_eq!(result, "test_field (String): This is a test field\nnested (InnerStruct): This is a nested struct\nvec_field (Vec < i32 >): This is a vector of integers");
    }

    #[test]
    fn test_dox() {
        if let DocType::Container(fields) = TestStruct::dox() {
            assert_eq!(fields.len(), 3);
            assert_eq!(fields[0].name, "test_field");
            assert_eq!(fields[0].typ, "String");
            assert_eq!(fields[0].doc, "This is a test field");
            assert_eq!(fields[1].name, "nested");
            assert_eq!(fields[1].typ, "InnerStruct");
            assert_eq!(fields[1].doc, "This is a nested struct");
            assert_eq!(fields[2].name, "vec_field");
            assert_eq!(fields[2].typ, "Vec < i32 >");
            assert_eq!(fields[2].doc, "This is a vector of integers");
        } else {
            panic!("Expected DocType::Container");
        }
    }
}
