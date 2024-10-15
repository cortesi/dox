//! A command-line tool that renders a demo struct that exercises all supported variants.

use dox::Dox;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, Dox)]
#[serde(rename_all = "snake_case")]
pub enum EnumVariants {
    #[default]
    Plain,
    // Compound(Vec<String>),
}

#[derive(Debug, Clone, Deserialize, Serialize, Dox)]
pub struct Demo {
    /// A plain string
    #[serde(default)]
    pub string: String,

    /// The default dialect.
    #[serde(default)]
    pub plain_variants: EnumVariants,
}

fn main() {
    let renderer = dox::render::Text;
    let rendered = dox::render::<Demo, _>(&renderer);
    println!("{}", rendered);
}
