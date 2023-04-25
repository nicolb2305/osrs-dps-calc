use crate::generics::{NamedData, Scalar};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Spell {
    pub name: String,
    pub max_hit: Scalar,
    pub spellbook: Spellbook,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub enum Attribute {
    Bolt,
    Barrage,
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub enum Spellbook {
    Standard,
    Ancient,
    Lunar,
    Arceuus,
}

impl NamedData for Spell {
    fn get_name(&self) -> &str {
        &self.name
    }
}
