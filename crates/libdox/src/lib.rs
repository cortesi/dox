pub use dox_derive::Dox;

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub typ: String,
    pub doc: String,
}

#[derive(Debug, Clone)]
pub enum DocType {
    Field(Field),
    Container(Vec<Field>),
}

pub trait Dox {
    fn dox() -> DocType;
}

pub trait Renderer {
    fn render(&self, doc_type: DocType) -> String;
}

pub struct Text;

impl Renderer for Text {
    fn render(&self, doc_type: DocType) -> String {
        match doc_type {
            DocType::Field(field) => format!("{} ({}): {}", field.name, field.typ, field.doc),
            DocType::Container(fields) => fields
                .into_iter()
                .map(|field| format!("{} ({}): {}", field.name, field.typ, field.doc))
                .collect::<Vec<_>>()
                .join("\n"),
        }
    }
}

pub fn render<T: Dox, R: Renderer>(renderer: &R) -> String {
    renderer.render(T::dox())
}
