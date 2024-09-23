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

pub trait Renderer {
    fn render(&self, doc_type: Field) -> String;
}

pub struct Text;

impl Renderer for Text {
    fn render(&self, doc_type: Field) -> String {
        match doc_type {
            Field::Primitive(field) => format!("{} ({}): {}", field.name, field.typ, field.doc),
            Field::Container(container) => {
                let fields = container
                    .fields
                    .iter()
                    .map(|field| self.render(field.clone()))
                    .collect::<Vec<_>>()
                    .join("\n");
                format!("{}\n{}", container.doc, fields)
            }
        }
    }
}

pub fn render<T: Dox, R: Renderer>(renderer: &R) -> String {
    renderer.render(T::dox())
}
