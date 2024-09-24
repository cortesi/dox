pub mod render;

pub use dox_derive::Dox;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Typ {
    I8,
    I16,
    I32,
    I64,
    I128,
    Isize,
    U8,
    U16,
    U32,
    U64,
    U128,
    Usize,
    F32,
    F64,
    Bool,
    Char,
    String,
    Vec(Box<Typ>),
}

impl fmt::Display for Typ {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Typ::I8 => write!(f, "i8"),
            Typ::I16 => write!(f, "i16"),
            Typ::I32 => write!(f, "i32"),
            Typ::I64 => write!(f, "i64"),
            Typ::I128 => write!(f, "i128"),
            Typ::Isize => write!(f, "isize"),
            Typ::U8 => write!(f, "u8"),
            Typ::U16 => write!(f, "u16"),
            Typ::U32 => write!(f, "u32"),
            Typ::U64 => write!(f, "u64"),
            Typ::U128 => write!(f, "u128"),
            Typ::Usize => write!(f, "usize"),
            Typ::F32 => write!(f, "f32"),
            Typ::F64 => write!(f, "f64"),
            Typ::Bool => write!(f, "bool"),
            Typ::Char => write!(f, "char"),
            Typ::String => write!(f, "String"),
            Typ::Vec(inner) => write!(f, "Vec<{}>", inner),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Primitive {
    pub name: String,
    pub typ: Typ,
    pub doc: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Container {
    pub name: String,
    pub fields: Vec<Field>,
    pub doc: String,
    pub original_name: String,
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
            Field::Container(_) => Typ::String, // Use String as a placeholder for custom types
        };
        Field::Primitive(Primitive {
            name: String::new(),
            typ: Typ::Vec(Box::new(inner_type)),
            doc: String::new(),
        })
    }
}

macro_rules! impl_dox_for_primitive {
    ($($t:ty => $variant:ident),*) => {
        $(
            impl Dox for $t {
                fn dox() -> Field {
                    Field::Primitive(Primitive {
                        name: String::new(),
                        typ: Typ::$variant,
                        doc: String::new(),
                    })
                }
            }
        )*
    }
}

impl_dox_for_primitive! {
    i8 => I8, i16 => I16, i32 => I32, i64 => I64, i128 => I128, isize => Isize,
    u8 => U8, u16 => U16, u32 => U32, u64 => U64, u128 => U128, usize => Usize,
    f32 => F32, f64 => F64,
    bool => Bool, char => Char
}

impl Dox for String {
    fn dox() -> Field {
        Field::Primitive(Primitive {
            name: String::new(),
            typ: Typ::String,
            doc: String::new(),
        })
    }
}

pub trait Renderer {
    fn render(&self, doc_type: Field) -> String;
}

pub fn render<T: Dox, R: Renderer>(renderer: &R) -> String {
    renderer.render(T::dox())
}
