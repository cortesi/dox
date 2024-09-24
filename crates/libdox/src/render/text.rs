use crate::{Field, Renderer};

pub struct Text;

impl Renderer for Text {
    fn render(&self, doc_type: Field) -> String {
        render_field(&doc_type, 0)
    }
}

fn render_field(field: &Field, indent: usize) -> String {
    match field {
        Field::Primitive(prim) => format!(
            "{}{} ({}): {}",
            "  ".repeat(indent),
            prim.name,
            prim.typ,
            prim.doc
        ),
        Field::Container(container) => {
            let mut result = format!("{}{}", "  ".repeat(indent), container.doc);
            for field in &container.fields {
                result.push_str(&format!("\n{}", render_field(field, indent + 1)));
            }
            result
        }
    }
}
