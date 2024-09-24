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

macro_rules! impl_dox_for_primitive {
    ($($t:ty, $name:expr, $doc:expr);*) => {
        $(
            impl Dox for $t {
                fn dox() -> Field {
                    Field::Primitive(Primitive {
                        name: $name.to_string(),
                        typ: stringify!($t).to_string(),
                        doc: $doc.to_string(),
                    })
                }
            }
        )*
    }
}

impl_dox_for_primitive! {
    i8, "i8", "8-bit signed integer";
    i16, "i16", "16-bit signed integer";
    i32, "i32", "32-bit signed integer";
    i64, "i64", "64-bit signed integer";
    i128, "i128", "128-bit signed integer";
    isize, "isize", "Pointer-sized signed integer";
    u8, "u8", "8-bit unsigned integer";
    u16, "u16", "16-bit unsigned integer";
    u32, "u32", "32-bit unsigned integer";
    u64, "u64", "64-bit unsigned integer";
    u128, "u128", "128-bit unsigned integer";
    usize, "usize", "Pointer-sized unsigned integer";
    f32, "f32", "32-bit floating point";
    f64, "f64", "64-bit floating point";
    bool, "bool", "Boolean type";
    char, "char", "Unicode scalar value";
    String, "String", "UTF-8 encoded, growable string"
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
