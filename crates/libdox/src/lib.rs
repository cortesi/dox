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

impl<T: Dox> Dox for Vec<T> {
    fn dox() -> Field {
        let inner = T::dox();
        let inner_type = match inner {
            Field::Primitive(p) => p.typ,
            Field::Container(c) => c.name,
        };
        Field::Primitive(Primitive {
            name: String::new(),
            typ: format!("Vec < {} >", inner_type),
            doc: String::new(),
        })
    }
}

macro_rules! impl_dox_for_primitive {
    ($($t:ty),*) => {
        $(
            impl Dox for $t {
                fn dox() -> Field {
                    Field::Primitive(Primitive {
                        name: String::new(),
                        typ: stringify!($t).to_lowercase(),
                        doc: String::new(),
                    })
                }
            }
        )*
    }
}

impl_dox_for_primitive! {
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
    f32, f64,
    bool, char
}

impl Dox for String {
    fn dox() -> Field {
        Field::Primitive(Primitive {
            name: String::new(),
            typ: "String".to_string(),
            doc: String::new(),
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
