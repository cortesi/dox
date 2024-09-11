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
}
