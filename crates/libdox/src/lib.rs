pub use dox_derive::Dox;

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
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
pub enum DocType {
    Field(Field),
    Container(Container),
}

pub trait Dox {
    fn dox() -> DocType;
}

impl<T> Dox for Vec<T> {
    fn dox() -> DocType {
        DocType::Field(Field {
            name: "Vec".to_string(),
            typ: "Vec<T>".to_string(),
            doc: "A vector of items".to_string(),
        })
    }
}

macro_rules! impl_dox_for_primitive {
    ($($t:ty),*) => {
        $(
            impl Dox for $t {
                fn dox() -> DocType {
                    DocType::Field(Field {
                        name: stringify!($t).to_string(),
                        typ: stringify!($t).to_string(),
                        doc: format!("A {} value", stringify!($t)),
                    })
                }
            }
        )*
    }
}

impl_dox_for_primitive!(
    i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64, bool, char, String
);

pub trait Renderer {
    fn render(&self, doc_type: DocType) -> String;
}

pub struct Text;

impl Renderer for Text {
    fn render(&self, doc_type: DocType) -> String {
        match doc_type {
            DocType::Field(field) => format!("{} ({}): {}", field.name, field.typ, field.doc),
            DocType::Container(container) => {
                let fields = container
                    .fields
                    .into_iter()
                    .map(|field| format!("{} ({}): {}", field.name, field.typ, field.doc))
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
