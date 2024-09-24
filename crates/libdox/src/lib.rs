pub use dox_derive::Dox;

#[derive(Debug, Clone, PartialEq)]
pub struct Primitive {
    pub name: String,
    pub typ: String,
    pub doc: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Container {
    pub name: String,
    pub fields: Vec<Field>,
    pub doc: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Field {
    Primitive(Primitive),
    Container(Container),
}

pub trait Dox {
    fn dox() -> Field;
}

impl<T> Dox for Vec<T> {
    fn dox() -> Field {
        Field::Primitive(Primitive {
            name: "Vec".to_string(),
            typ: "Vec<T>".to_string(),
            doc: "A vector of items".to_string(),
        })
    }
}

impl Dox for i32 {
    fn dox() -> Field {
        Field::Primitive(Primitive {
            name: "i32".to_string(),
            typ: "i32".to_string(),
            doc: "32-bit signed integer".to_string(),
        })
    }
}

impl Dox for String {
    fn dox() -> Field {
        Field::Primitive(Primitive {
            name: "String".to_string(),
            typ: "String".to_string(),
            doc: "UTF-8 encoded, growable string".to_string(),
        })
    }
}

pub trait Renderer {
    fn render(&self, doc_type: Field) -> String;
}

pub struct Text;

impl Renderer for Text {
    fn render(&self, doc_type: Field) -> String {
        self.render_field(&doc_type, 0)
    }
}

impl Text {
    fn render_field(&self, field: &Field, indent: usize) -> String {
        match field {
            Field::Primitive(prim) => format!(
                "{}{} ({}): {}",
                "  ".repeat(indent),
                prim.name,
                prim.typ,
                prim.doc
            ),
            Field::Container(container) => {
                let mut result = container.doc.to_string();
                for field in &container.fields {
                    result.push_str(&format!("\n{}", self.render_field(field, indent + 1)));
                }
                result
            }
        }
    }
}

pub fn render<T: Dox, R: Renderer>(renderer: &R) -> String {
    renderer.render(T::dox())
}
