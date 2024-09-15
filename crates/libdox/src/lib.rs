pub use dox_derive::Dox;

#[derive(Debug, Clone)]
pub struct FieldDoc {
    pub name: String,
    pub typ: String,
    pub doc: String,
}

pub trait Dox {
    fn dox_fields() -> Vec<FieldDoc>;
}

pub trait Renderer {
    fn render(&self, fields: Vec<FieldDoc>) -> String;
}

pub struct Text;

impl Renderer for Text {
    fn render(&self, fields: Vec<FieldDoc>) -> String {
        fields
            .into_iter()
            .map(|field| format!("{} ({}): {}", field.name, field.typ, field.doc))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

pub fn render<T: Dox, R: Renderer>(renderer: &R) -> String {
    renderer.render(T::dox_fields())
}
