use crate::{
    equipment::combat_styles::StyleType,
    generics::{Fraction, Scalar, Ticks},
    unit::{Enemy, Player},
};
use serde::Deserialize;
use std::cmp::min;

#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
pub enum Attribute {
    CrystalArmour,
    CrystalBow,
    SalveAmulet,
    SalveAmuletEnchanted,
    SalveAmuletImbued,
    SalveAmuletEnchantedImbued,
    BlackMask,
    BlackMaskImbued,
    VoidArmour,
    VoidHelmMelee,
    VoidHelmRanged,
    VoidHelmMagic,
    RevenantWeapon,
    DragonHunterLance,
    Arclight,
    KerisPartisan,
    Blisterwood,
    TzhaarMeleeWeapon,
    InquisitorArmour,
    BarroniteMace,
    Silverlight,
    IvandisFlail,
    LeadBladedBattleaxe,
    ColossalBlade,
    TwistedBow,
    DragonHunterCrossbow,
    SmokeStaff,
    HarmonisedNightmareStaff,
}

impl Attribute {
    pub fn accuracy_roll_callback(self) -> fn(Scalar, &Player, &Enemy) -> Scalar {
        match self {
            Self::DragonHunterCrossbow => dragon_hunter_crossbow_accuracy,
            Self::SalveAmulet => salve_amulet,
            _ => identity,
        }
    }

    pub fn max_hit_callback(self) -> fn(Scalar, &Player, &Enemy) -> Scalar {
        match self {
            Self::DragonHunterCrossbow => dragon_hunter_crossbow_max_hit,
            Self::SalveAmulet => salve_amulet,
            Self::ColossalBlade => colossal_blade,
            _ => identity,
        }
    }

    pub fn attack_speed_callback(self) -> fn(Ticks, &Player, &Enemy) -> Ticks {
        match self {
            Self::HarmonisedNightmareStaff => harmonised_nightmare_staff_attack_speed,
            _ => identity,
        }
    }
}

pub trait Callbacks {
    fn accuracy_roll_callback(&self, value: Scalar, player: &Player, enemy: &Enemy) -> Scalar;
    fn max_hit_callback(&self, value: Scalar, player: &Player, enemy: &Enemy) -> Scalar;
}

impl Callbacks for Vec<Attribute> {
    fn accuracy_roll_callback(&self, value: Scalar, player: &Player, enemy: &Enemy) -> Scalar {
        self.iter().fold(value, |value, attribute| {
            (attribute.accuracy_roll_callback())(value, player, enemy)
        })
    }

    fn max_hit_callback(&self, value: Scalar, player: &Player, enemy: &Enemy) -> Scalar {
        self.iter().fold(value, |value, attribute| {
            (attribute.max_hit_callback())(value, player, enemy)
        })
    }
}

pub(crate) fn identity<T>(value: T, _player: &Player, _enemy: &Enemy) -> T {
    value
}

pub(crate) fn dragon_hunter_crossbow_accuracy(
    accuracy_roll: Scalar,
    _player: &Player,
    enemy: &Enemy,
) -> Scalar {
    use crate::unit::EnemyAttribute::Dragon;

    if enemy.has_attribute(&Dragon) {
        accuracy_roll
            * Fraction {
                dividend: 13,
                divisor: 10,
            }
    } else {
        accuracy_roll
    }
}

pub(crate) fn dragon_hunter_crossbow_max_hit(
    max_hit: Scalar,
    _player: &Player,
    enemy: &Enemy,
) -> Scalar {
    use crate::unit::EnemyAttribute::Dragon;

    if enemy.has_attribute(&Dragon) {
        max_hit
            * Fraction {
                dividend: 5,
                divisor: 4,
            }
    } else {
        max_hit
    }
}

pub(crate) fn salve_amulet(value: Scalar, player: &Player, enemy: &Enemy) -> Scalar {
    use crate::unit::EnemyAttribute::Undead;

    match player.combat_option().style_type {
        StyleType::Stab | StyleType::Slash | StyleType::Crush if enemy.has_attribute(&Undead) => {
            value
                * Fraction {
                    dividend: 7,
                    divisor: 6,
                }
        }
        _ => value,
    }
}

pub(crate) fn colossal_blade(max_hit: Scalar, _player: &Player, enemy: &Enemy) -> Scalar {
    let size: Scalar = min(enemy.size, 5.into()).into();
    max_hit + (Scalar::new(2) * size)
}

pub(crate) fn harmonised_nightmare_staff_attack_speed(
    attack_speed: Ticks,
    player: &Player,
    _enemy: &Enemy,
) -> Ticks {
    if player.spell.is_some() {
        4.into()
    } else {
        attack_speed
    }
}
