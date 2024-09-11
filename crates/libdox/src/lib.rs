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

pub fn render<T: Dox>() -> String {
    T::dox_fields()
        .into_iter()
        .map(|field| format!("{} ({}): {}", field.name, field.typ, field.doc))
        .collect::<Vec<_>>()
        .join("\n")
}
