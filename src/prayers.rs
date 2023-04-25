use std::ops::Add;

use crate::generics::{NamedData, Percentage};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Prayer {
    pub name: String,
    #[serde(flatten)]
    pub stats: Stats,
}

#[derive(Deserialize, Debug, Clone, Copy, Default)]
pub struct Stats {
    pub defence: Percentage,
    pub melee_accuracy: Percentage,
    pub melee_damage: Percentage,
    pub ranged_accuracy: Percentage,
    pub ranged_damage: Percentage,
    pub magic_accuracy: Percentage,
    pub magic_defence: Percentage,
}

impl Add for Stats {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            defence: self.defence + rhs.defence,
            melee_accuracy: self.melee_accuracy + rhs.melee_accuracy,
            melee_damage: self.melee_damage + rhs.melee_damage,
            ranged_accuracy: self.ranged_accuracy + rhs.ranged_accuracy,
            ranged_damage: self.ranged_damage + rhs.ranged_damage,
            magic_accuracy: self.magic_accuracy + rhs.magic_accuracy,
            magic_defence: self.magic_defence + rhs.magic_defence,
        }
    }
}

impl NamedData for Prayer {
    fn get_name(&self) -> &str {
        &self.name
    }
}
