#![allow(dead_code)]

use libdox::Dox;

#[derive(Dox)]
struct TestStruct {
    /// This is a test field
    test_field: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use libdox::render;

    #[test]
    fn test_simple_render() {
        let result = render::<TestStruct>();
        assert_eq!(result, "test_field (String): This is a test field");
    }

    #[test]
    fn test_dox_fields() {
        let fields = TestStruct::dox_fields();
        assert_eq!(fields.len(), 1);
        assert_eq!(fields[0].name, "test_field");
        assert_eq!(fields[0].typ, "String");
        assert_eq!(fields[0].doc, "This is a test field");
    }
}
