use crate::{Container, Field, Renderer};

pub struct Text;

impl Renderer for Text {
    fn render(&self, doc_type: Field) -> String {
        match doc_type {
            Field::Container(container) => render_container(&container),
            Field::Primitive(_) => String::new(),
        }
    }
}

fn render_container(container: &Container) -> String {
    let mut result = String::new();
    result.push_str(&format!(
        "{}\n{}\n",
        container.name,
        "=".repeat(container.name.len())
    ));

    for field in &container.fields {
        match field {
            Field::Primitive(prim) => {
                result.push_str(&format!("{} ({}): {}\n", prim.name, prim.typ, prim.doc));
            }
            Field::Container(nested) => {
                result.push_str(&format!(
                    "{} ({}): {}\n",
                    nested.name, nested.name, nested.doc
                ));
                result.push('\n');
                result.push_str(&render_container(nested));
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
                    name: "NestedStruct".to_string(),
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
            NestedStruct (NestedStruct): A nested struct

            NestedStruct
            ============
            inner_field (bool): A boolean field
        "};

        assert_eq!(result, expected);
    }
}
