#![allow(clippy::needless_update)]
pub mod combat_styles;
mod default_items;
pub(crate) mod weapon_callbacks;

use self::{
    combat_styles::{CombatOption, WeaponType},
    weapon_callbacks::Attribute,
};
use crate::generics::{NamedData, Percentage, Scalar, Ticks, Tiles};
use serde::Deserialize;

#[allow(clippy::module_name_repetitions)]
pub trait ContainsEquipment: for<'a> Deserialize<'a> {
    fn inner(&self) -> &Equipment;
}

pub trait ContainsWeaponStats {
    fn weapon_stats(&self) -> WeaponStats;
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct Equipment {
    pub name: String,
    #[serde(flatten)]
    pub stats: Stats,
    pub attributes: Vec<Attribute>,
}

macro_rules! equipment_struct {
    ($($struct_name:ident),*) => {
        $(
            #[derive(Deserialize, Debug, Clone)]
            pub struct $struct_name {
                #[serde(flatten)]
                pub inner: Equipment,
            }

            impl ContainsEquipment for $struct_name {
                fn inner(&self) -> &Equipment {
                    &self.inner
                }
            }

            impl Default for $struct_name {
                fn default() -> Self {
                    Self {
                        inner: Equipment {
                            name: "Empty".to_owned(),
                            stats: Stats::default(),
                            attributes: Vec::default()
                        }
                    }
                }
            }
        )*
    };
}

macro_rules! weapon_struct {
    ($($struct_name:ident),*) => {
        $(
            #[derive(Deserialize, Debug, Clone)]
            pub struct $struct_name {
                #[serde(flatten)]
                pub inner: Equipment,
                pub weapon_stats: WeaponStats,
                pub powered_staff_type: Option<PoweredStaff>
            }

            impl ContainsEquipment for $struct_name {
                fn inner(&self) -> &Equipment {
                    &self.inner
                }
            }

            impl ContainsWeaponStats for $struct_name {
                fn weapon_stats(&self) -> WeaponStats {
                    self.weapon_stats
                }
            }

            impl Default for $struct_name {
                fn default() -> Self {
                    Self {
                        inner: Equipment {
                            name: "Empty".to_owned(),
                            stats: Stats::default(),
                            attributes: Vec::default(),
                        },
                        weapon_stats: WeaponStats::default(),
                        powered_staff_type: None,
                    }
                }
            }
        )*
    };
}

equipment_struct!(Head, Cape, Neck, Ammunition, Shield, Body, Legs, Hands, Feet, Ring);
weapon_struct!(WeaponOneHanded, WeaponTwoHanded);

#[derive(Debug, Deserialize, Clone, Copy)]
pub enum PoweredStaff {
    StarterStaff,
    TridentOfTheSeas,
    ThammaronsSceptre,
    AccursedSceptre,
    TridentOfTheSwamp,
    SanguinestiStaff,
    Dawnbringer,
    TumekensShadow,
    CrystalStaffBasic,
    CrystalStaffAttuned,
    CrystallStaffPerfected,
    SwampLizard,
    OrangeSalamander,
    RedSalamander,
    BlackSalamander,
}

#[derive(Debug, Deserialize, Default, Clone, Copy, derive_more::Sum, derive_more::Add)]
pub struct Stats {
    pub attack: StatBonuses,
    pub defence: StatBonuses,
    pub damage: DamageBonus,
    pub prayer_bonus: Scalar,
}

#[derive(Debug, Clone, Copy)]
pub enum Wielded<'a> {
    OneHanded {
        weapon: Option<&'a WeaponOneHanded>,
        shield: Option<&'a Shield>,
    },
    TwoHanded {
        weapon: Option<&'a WeaponTwoHanded>,
    },
}

impl Default for Wielded<'_> {
    fn default() -> Self {
        Self::OneHanded {
            weapon: None,
            shield: None,
        }
    }
}

impl<'a> Wielded<'a> {
    pub fn equip_one_handed(
        weapon: Option<&'a WeaponOneHanded>,
        shield: Option<&'a Shield>,
    ) -> Self {
        Self::OneHanded { weapon, shield }
    }

    pub fn equip_two_handed(weapon: Option<&'a WeaponTwoHanded>) -> Self {
        Self::TwoHanded { weapon }
    }

    pub fn combat_boost(&self) -> Vec<CombatOption> {
        match self {
            Self::OneHanded { weapon, shield: _ } => weapon
                .unwrap_or_default()
                .weapon_stats
                .weapon_type
                .combat_boost(),
            Self::TwoHanded { weapon } => weapon
                .unwrap_or_default()
                .weapon_stats
                .weapon_type
                .combat_boost(),
        }
    }

    pub fn stats(&self) -> Stats {
        match self {
            Self::OneHanded { weapon, shield } => {
                weapon.unwrap_or_default().inner.stats + shield.unwrap_or_default().inner.stats
            }
            Self::TwoHanded { weapon } => weapon.unwrap_or_default().inner.stats,
        }
    }

    pub fn weapon_stats(&self) -> WeaponStats {
        match self {
            Self::OneHanded { weapon, shield: _ } => weapon.unwrap_or_default().weapon_stats,
            Self::TwoHanded { weapon } => weapon.unwrap_or_default().weapon_stats,
        }
    }

    pub fn attack_speed(&self, combat_style: &CombatOption) -> Ticks {
        let tick_offset = combat_style
            .invisible_boost()
            .expect("Valid combat style")
            .attack_speed;

        let weapon_attack_speed = match self {
            Self::OneHanded { weapon, shield: _ } => {
                weapon.unwrap_or_default().weapon_stats.attack_speed
            }
            Self::TwoHanded { weapon } => weapon.unwrap_or_default().weapon_stats.attack_speed,
        };

        weapon_attack_speed + tick_offset
    }

    pub fn attributes(&self) -> &Vec<Attribute> {
        match self {
            Self::OneHanded { weapon, shield: _ } => &weapon.unwrap_or_default().inner.attributes,
            Self::TwoHanded { weapon } => &weapon.unwrap_or_default().inner.attributes,
        }
    }

    pub fn weapon_has_attribute(&self, attribute: &Attribute) -> bool {
        self.attributes().contains(attribute)
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "slot")]
pub enum Slots {
    Head(Head),
    Cape(Cape),
    Neck(Neck),
    Ammunition(Ammunition),
    WeaponOneHanded(WeaponOneHanded),
    WeaponTwoHanded(WeaponTwoHanded),
    Shield(Shield),
    Body(Body),
    Legs(Legs),
    Hands(Hands),
    Feet(Feet),
    Ring(Ring),
}

impl ContainsEquipment for Slots {
    fn inner(&self) -> &Equipment {
        match self {
            Self::Head(v) => v.inner(),
            Self::Cape(v) => v.inner(),
            Self::Neck(v) => v.inner(),
            Self::Ammunition(v) => v.inner(),
            Self::WeaponOneHanded(v) => v.inner(),
            Self::WeaponTwoHanded(v) => v.inner(),
            Self::Shield(v) => v.inner(),
            Self::Body(v) => v.inner(),
            Self::Legs(v) => v.inner(),
            Self::Hands(v) => v.inner(),
            Self::Feet(v) => v.inner(),
            Self::Ring(v) => v.inner(),
        }
    }
}

impl NamedData for Slots {
    fn get_name(&self) -> &str {
        &self.inner().name
    }
}

#[derive(Deserialize, Debug, Clone, Copy, derive_more::Sum, derive_more::Add)]
pub struct StatBonuses {
    pub stab: Scalar,
    pub slash: Scalar,
    pub crush: Scalar,
    pub ranged: Scalar,
    pub magic: Scalar,
}

impl Default for StatBonuses {
    fn default() -> Self {
        Self {
            stab: 0.into(),
            slash: 0.into(),
            crush: 0.into(),
            ranged: 0.into(),
            magic: 0.into(),
        }
    }
}

#[derive(Deserialize, Debug, Clone, Copy, derive_more::Sum, derive_more::Add)]
pub struct DamageBonus {
    pub strength: Scalar,
    pub ranged: Scalar,
    pub magic: Percentage,
}

impl Default for DamageBonus {
    fn default() -> Self {
        Self {
            strength: 0.into(),
            ranged: 0.into(),
            magic: 0.into(),
        }
    }
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct WeaponStats {
    pub weapon_type: WeaponType,
    pub attack_speed: Ticks,
    pub range: Tiles,
}

impl Default for WeaponStats {
    fn default() -> Self {
        Self {
            weapon_type: WeaponType::Unarmed,
            attack_speed: 4.into(),
            range: 1.into(),
        }
    }
}
