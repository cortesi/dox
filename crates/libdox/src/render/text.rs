use crate::{Container, Enum, Field, Renderer};

pub struct Text;

impl Renderer for Text {
    fn render(&self, doc_type: Field) -> String {
        match doc_type {
            Field::Container(container) => render_container(&container),
            Field::Primitive(_) => String::new(),
            Field::Enum(enum_type) => render_enum(&enum_type),
        }
    }
}

fn render_enum(enum_type: &Enum) -> String {
    let mut result = String::new();
    result.push_str(&format!("{} (enum)\n", enum_type.name));
    result.push_str(&format!("Doc: {}\n", enum_type.doc));
    result.push_str("Variants:\n");
    for variant in &enum_type.variants {
        result.push_str(&format!("- {}\n", variant));
    }
    result
}

fn render_container(container: &Container) -> String {
    let mut result = String::new();
    result.push_str(&format!(
        "{}\n{}\n",
        container.type_name,
        "=".repeat(container.type_name.len())
    ));

    for field in &container.fields {
        match field {
            Field::Primitive(prim) => {
                result.push_str(&format!("{} ({}): {}\n", prim.name, prim.typ, prim.doc));
            }
            Field::Container(nested) => {
                result.push_str(&format!(
                    "{} ({}): {}\n",
                    nested.name, nested.type_name, nested.doc
                ));
                result.push('\n');
                result.push_str(&render_container(nested));
            }
            Field::Enum(enum_type) => {
                result.push_str(&format!("{} (enum): {}\n", enum_type.name, enum_type.doc));
                result.push_str(&render_enum(enum_type));
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Container, Field, Primitive, Renderer, Typ};
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_text_renderer() {
        let doc = Field::Container(Container {
            name: "TestStruct".to_string(),
            type_name: "TestStruct".to_string(),
            doc: "This is a test struct".to_string(),
            fields: vec![
                Field::Primitive(Primitive {
                    name: "field1".to_string(),
                    typ: Typ::String,
                    doc: "A string field".to_string(),
                }),
                Field::Primitive(Primitive {
                    name: "field2".to_string(),
                    typ: Typ::I32,
                    doc: "An integer field".to_string(),
                }),
                Field::Container(Container {
                    name: "nested".to_string(),
                    type_name: "NestedStruct".to_string(),
                    doc: "A nested struct".to_string(),
                    fields: vec![Field::Primitive(Primitive {
                        name: "inner_field".to_string(),
                        typ: Typ::Bool,
                        doc: "A boolean field".to_string(),
                    })],
                }),
            ],
        });

        let renderer = Text;
        let result = renderer.render(doc);

        let expected = indoc! {"
            TestStruct
            ==========
            field1 (String): A string field
            field2 (i32): An integer field
            nested (NestedStruct): A nested struct

            NestedStruct
            ============
            inner_field (bool): A boolean field
        "};

        assert_eq!(result, expected);
    }
}
