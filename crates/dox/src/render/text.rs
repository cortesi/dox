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
    result.push_str(&format!("{}\n", "=".repeat(enum_type.name.len() + 7)));
    result.push_str(&format!("{}\n\n", enum_type.doc));
    result.push_str("Variants:\n");
    for variant in &enum_type.variants {
        result.push_str(&format!("- {}: {}\n", variant.name, variant.doc));
    }
    result
}

fn render_container(container: &Container) -> String {
    let mut result = String::new();
    result.push_str(&format!("{}\n", container.type_name));
    result.push_str(&format!("{}\n", "=".repeat(container.type_name.len())));
    result.push_str(&format!("{}\n\n", container.doc));

    for field in &container.fields {
        match field {
            Field::Primitive(prim) => {
                result.push_str(&format!("- {} ({}): {}\n", prim.name, prim.typ, prim.doc));
            }
            Field::Container(nested) => {
                result.push_str(&format!(
                    "- {} ({}): {}\n",
                    nested.name, nested.type_name, nested.doc
                ));
            }
            Field::Enum(enum_type) => {
                result.push_str(&format!("- {} (enum): {}\n", enum_type.name, enum_type.doc));
            }
        }
    }

    let nested_fields: Vec<_> = container
        .fields
        .iter()
        .filter(|f| matches!(f, Field::Container(_) | Field::Enum(_)))
        .collect();

    if !nested_fields.is_empty() {
        result.push('\n');
        for (i, field) in nested_fields.iter().enumerate() {
            result.push_str(&match field {
                Field::Container(nested) => render_container(nested),
                Field::Enum(enum_type) => render_enum(enum_type),
                _ => unreachable!(),
            });
            if i < nested_fields.len() - 1 {
                result.push('\n');
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Container, Field, Primitive, Renderer, Typ, Variant};
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    fn strip_and_compare(a: &str, b: &str) {
        let a_stripped = a
            .lines()
            .map(|line| line.trim())
            .collect::<Vec<&str>>()
            .join("\n");
        let b_stripped = b
            .lines()
            .map(|line| line.trim())
            .collect::<Vec<&str>>()
            .join("\n");
        assert_eq!(a_stripped, b_stripped);
    }

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
                        name: "nested_field".to_string(),
                        typ: Typ::String,
                        doc: "A field in the nested struct".to_string(),
                    })],
                }),
                Field::Enum(Enum {
                    name: "enum_field".to_string(),
                    doc: "An enum field".to_string(),
                    variants: vec![
                        Variant {
                            name: "Variant1".to_string(),
                            doc: "First variant".to_string(),
                        },
                        Variant {
                            name: "Variant2".to_string(),
                            doc: "Second variant".to_string(),
                        },
                    ],
                }),
            ],
        });

        let renderer = Text;
        let result = renderer.render(doc);

        let expected = indoc! {"
            TestStruct
            ==========
            This is a test struct

            - field1 (String): A string field
            - field2 (i32): An integer field
            - nested (NestedStruct): A nested struct
            - enum_field (enum): An enum field

            NestedStruct
            ============
            A nested struct

            - nested_field (String): A field in the nested struct

            enum_field (enum)
            =================
            An enum field

            Variants:
            - Variant1: First variant
            - Variant2: Second variant
        "};

        strip_and_compare(&result, expected);
    }
}
