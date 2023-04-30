use self::callbacks::{
    arclight, black_mask, black_mask_imbued, blisterwood_accuracy, blisterwood_flail_max_hit,
    blisterwood_sickle_max_hit, colossal_blade, dragon_hunter_crossbow_accuracy,
    dragon_hunter_crossbow_max_hit, harmonised_nightmare_staff_attack_speed, identity,
    salve_amulet, salve_amulet_enchanted, salve_amulet_enchanted_imbued, salve_amulet_imbued,
    wilderness_weapon_magic, wilderness_weapon_melee, wilderness_weapon_ranged,
};
use crate::{
    generics::{Scalar, Ticks},
    unit::{Enemy, Player},
};
use serde::Deserialize;

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
    BlisterwoodFlail,
    BlisterwoodSickle,
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
    WildernessWeaponMelee,
    WildernessWeaponRanged,
    WildernessWeaponMagic,
}

impl Attribute {
    pub fn accuracy_roll_callback(self) -> fn(Scalar, &Player, &Enemy) -> Scalar {
        match self {
            Self::DragonHunterCrossbow => dragon_hunter_crossbow_accuracy,
            Self::SalveAmulet => salve_amulet,
            Self::SalveAmuletImbued => salve_amulet_imbued,
            Self::SalveAmuletEnchanted => salve_amulet_enchanted,
            Self::SalveAmuletEnchantedImbued => salve_amulet_enchanted_imbued,
            Self::BlackMask => black_mask,
            Self::BlackMaskImbued => black_mask_imbued,
            Self::WildernessWeaponMelee => wilderness_weapon_melee,
            Self::WildernessWeaponRanged => wilderness_weapon_ranged,
            Self::WildernessWeaponMagic => wilderness_weapon_magic,
            Self::Arclight => arclight,
            Self::BlisterwoodFlail | Self::BlisterwoodSickle => blisterwood_accuracy,
            _ => identity,
        }
    }

    pub fn max_hit_callback(self) -> fn(Scalar, &Player, &Enemy) -> Scalar {
        match self {
            Self::DragonHunterCrossbow => dragon_hunter_crossbow_max_hit,
            Self::SalveAmulet => salve_amulet,
            Self::SalveAmuletImbued => salve_amulet_imbued,
            Self::SalveAmuletEnchanted => salve_amulet_enchanted,
            Self::SalveAmuletEnchantedImbued => salve_amulet_enchanted_imbued,
            Self::BlackMask => black_mask,
            Self::BlackMaskImbued => black_mask_imbued,
            Self::ColossalBlade => colossal_blade,
            Self::WildernessWeaponMelee => wilderness_weapon_melee,
            Self::WildernessWeaponRanged => wilderness_weapon_ranged,
            Self::WildernessWeaponMagic => wilderness_weapon_magic,
            Self::Arclight => arclight,
            Self::BlisterwoodFlail => blisterwood_flail_max_hit,
            Self::BlisterwoodSickle => blisterwood_sickle_max_hit,
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

mod callbacks {
    use super::Attribute;
    use crate::{
        equipment::combat_styles::StyleType,
        generics::{Fraction, Scalar, Ticks},
        unit::{Enemy, EnemyAttribute, Player},
    };
    use std::cmp::min;

    pub(crate) fn identity<T>(value: T, _player: &Player, _enemy: &Enemy) -> T {
        value
    }

    pub(crate) fn dragon_hunter_crossbow_accuracy(
        value: Scalar,
        player: &Player,
        enemy: &Enemy,
    ) -> Scalar {
        if enemy.has_attribute(&EnemyAttribute::Dragon)
            && player.combat_option().style_type.is_melee()
        {
            value * Fraction::new(13, 10)
        } else {
            value
        }
    }

    pub(crate) fn dragon_hunter_crossbow_max_hit(
        value: Scalar,
        player: &Player,
        enemy: &Enemy,
    ) -> Scalar {
        if enemy.has_attribute(&EnemyAttribute::Dragon)
            && player.combat_option().style_type.is_ranged()
        {
            value * Fraction::new(5, 4)
        } else {
            value
        }
    }

    pub(crate) fn salve_amulet(value: Scalar, player: &Player, enemy: &Enemy) -> Scalar {
        if enemy.has_attribute(&EnemyAttribute::Undead)
            && player.combat_option().style_type.is_melee()
        {
            value * Fraction::new(7, 6)
        } else {
            value
        }
    }

    pub(crate) fn salve_amulet_enchanted(value: Scalar, player: &Player, enemy: &Enemy) -> Scalar {
        if enemy.has_attribute(&EnemyAttribute::Undead)
            && player.combat_option().style_type.is_melee()
        {
            value * Fraction::new(6, 5)
        } else {
            value
        }
    }

    pub(crate) fn salve_amulet_imbued(value: Scalar, _player: &Player, enemy: &Enemy) -> Scalar {
        if enemy.has_attribute(&EnemyAttribute::Undead) {
            value * Fraction::new(7, 6)
        } else {
            value
        }
    }

    pub(crate) fn salve_amulet_enchanted_imbued(
        value: Scalar,
        _player: &Player,
        enemy: &Enemy,
    ) -> Scalar {
        if enemy.has_attribute(&EnemyAttribute::Undead) {
            value * Fraction::new(6, 5)
        } else {
            value
        }
    }

    pub(crate) fn black_mask(value: Scalar, player: &Player, _enemy: &Enemy) -> Scalar {
        let attrs = &player.equipped().head.unwrap_or_default().inner.attributes;
        if player.extra.on_slayer_task
            && player.combat_option().style_type.is_melee()
            && !attrs.contains(&Attribute::SalveAmulet)
            && !attrs.contains(&Attribute::SalveAmuletEnchanted)
            && !attrs.contains(&Attribute::SalveAmuletImbued)
            && !attrs.contains(&Attribute::SalveAmuletEnchantedImbued)
        {
            value * Fraction::new(7, 6)
        } else {
            value
        }
    }

    pub(crate) fn black_mask_imbued(value: Scalar, player: &Player, _enemy: &Enemy) -> Scalar {
        if player.extra.on_slayer_task {
            let attrs = &player.equipped().head.unwrap_or_default().inner.attributes;
            match player.combat_option().style_type {
                StyleType::Stab | StyleType::Slash | StyleType::Crush
                    if !attrs.contains(&Attribute::SalveAmulet)
                        && !attrs.contains(&Attribute::SalveAmuletEnchanted)
                        && !attrs.contains(&Attribute::SalveAmuletImbued)
                        && !attrs.contains(&Attribute::SalveAmuletEnchantedImbued) =>
                {
                    value * Fraction::new(7, 6)
                }
                StyleType::Ranged | StyleType::Magic
                    if !attrs.contains(&Attribute::SalveAmuletImbued)
                        && !attrs.contains(&Attribute::SalveAmuletEnchantedImbued) =>
                {
                    value * Fraction::new(23, 20)
                }
                _ => value,
            }
        } else {
            value
        }
    }

    pub(crate) fn wilderness_weapon_melee(
        value: Scalar,
        player: &Player,
        _enemy: &Enemy,
    ) -> Scalar {
        if player.extra.in_wilderness && player.combat_option().style_type.is_melee() {
            value * Fraction::new(3, 2)
        } else {
            value
        }
    }

    pub(crate) fn wilderness_weapon_ranged(
        value: Scalar,
        player: &Player,
        _enemy: &Enemy,
    ) -> Scalar {
        if player.extra.in_wilderness && player.combat_option().style_type.is_ranged() {
            value * Fraction::new(3, 2)
        } else {
            value
        }
    }

    pub(crate) fn wilderness_weapon_magic(
        value: Scalar,
        player: &Player,
        _enemy: &Enemy,
    ) -> Scalar {
        if player.extra.in_wilderness && player.combat_option().style_type.is_magic() {
            value * Fraction::new(3, 2)
        } else {
            value
        }
    }

    pub(crate) fn arclight(value: Scalar, player: &Player, enemy: &Enemy) -> Scalar {
        if enemy.has_attribute(&EnemyAttribute::Demon)
            && player.combat_option().style_type.is_melee()
        {
            value * Fraction::new(17, 10)
        } else {
            value
        }
    }

    pub(crate) fn blisterwood_accuracy(value: Scalar, player: &Player, enemy: &Enemy) -> Scalar {
        if enemy.has_attribute(&EnemyAttribute::Vampyre)
            && player.combat_option().style_type.is_melee()
        {
            value * Fraction::new(21, 20)
        } else {
            value
        }
    }

    pub(crate) fn blisterwood_flail_max_hit(
        value: Scalar,
        player: &Player,
        enemy: &Enemy,
    ) -> Scalar {
        if enemy.has_attribute(&EnemyAttribute::Vampyre)
            && player.combat_option().style_type.is_melee()
        {
            value * Fraction::new(5, 4)
        } else {
            value
        }
    }

    pub(crate) fn blisterwood_sickle_max_hit(
        value: Scalar,
        player: &Player,
        enemy: &Enemy,
    ) -> Scalar {
        if enemy.has_attribute(&EnemyAttribute::Vampyre)
            && player.combat_option().style_type.is_melee()
        {
            value * Fraction::new(23, 20)
        } else {
            value
        }
    }

    pub(crate) fn colossal_blade(value: Scalar, player: &Player, enemy: &Enemy) -> Scalar {
        if player.combat_option().style_type.is_melee() {
            let size: Scalar = min(enemy.size, 5.into()).into();
            value + (Scalar::new(2) * size)
        } else {
            value
        }
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

    // pub(crate) fn general_multiplier(
    //     enemy_attribute: &EnemyAttribute,
    //     fraction: Fraction,
    // ) -> impl Fn(Scalar, &Player, &Enemy) -> Scalar + '_ {
    //     move |value, _player, enemy| {
    //         if enemy.has_attribute(enemy_attribute) {
    //             value * fraction
    //         } else {
    //             value
    //         }
    //     }
    // }
}
