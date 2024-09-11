pub use dox_derive::Dox;

pub trait Dox {
    fn dox_fields() -> Vec<(String, String, String)>;
}

pub fn render<T: Dox>() -> String {
    T::dox_fields()
        .into_iter()
        .map(|(name, ty, doc)| format!("{} ({}): {}", name, ty, doc))
        .collect::<Vec<_>>()
        .join("\n")
}
