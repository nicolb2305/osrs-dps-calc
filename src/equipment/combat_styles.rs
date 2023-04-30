use crate::generics::{Scalar, Ticks, Tiles};
use anyhow::{anyhow, Result};
use serde::Deserialize;

#[derive(Debug, Clone, Default, Copy)]
pub enum StyleType {
    Slash,
    #[default]
    Crush,
    Stab,
    Ranged,
    Magic,
    None,
}

impl StyleType {
    pub fn is_melee(self) -> bool {
        matches!(self, Self::Crush | Self::Slash | Self::Stab)
    }
    pub fn is_ranged(self) -> bool {
        matches!(self, Self::Ranged)
    }
    pub fn is_magic(self) -> bool {
        matches!(self, Self::Magic)
    }
}

#[derive(Debug, Clone, Default)]
pub enum WeaponStyle {
    #[default]
    Accurate,
    Aggressive,
    Defensive,
    Controlled,
    Rapid,
    Longrange,
    ShortFuse,
    MediumFuse,
    LongFuse,
    Autocast,
    DefensiveAutocast,
    None,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct CombatOptionModifier {
    pub attack: Scalar,
    pub strength: Scalar,
    pub defence: Scalar,
    pub ranged: Scalar,
    pub magic: Scalar,
    pub attack_range: Tiles,
    pub attack_speed: Ticks,
}

#[derive(Debug, Clone)]
pub struct CombatOption {
    pub name: String,
    pub style_type: StyleType,
    pub weapon_style: WeaponStyle,
}

impl Default for CombatOption {
    fn default() -> Self {
        Self {
            name: "Punch".to_owned(),
            style_type: StyleType::Crush,
            weapon_style: WeaponStyle::Accurate,
        }
    }
}

impl CombatOption {
    pub fn new(name: &str, style_type: StyleType, weapon_style: WeaponStyle) -> Self {
        Self {
            name: name.to_owned(),
            style_type,
            weapon_style,
        }
    }

    /// # Errors
    /// Returns an error if `style_type` and `weapon_style` are incompatible, e.g. Slash, Rapid
    pub fn invisible_boost(&self) -> Result<CombatOptionModifier> {
        let mut boost = CombatOptionModifier::default();
        match (&self.style_type, &self.weapon_style) {
            (StyleType::Slash | StyleType::Crush | StyleType::Stab, WeaponStyle::Accurate) => {
                boost.attack += 3.into();
            }
            (StyleType::Slash | StyleType::Crush | StyleType::Stab, WeaponStyle::Aggressive) => {
                boost.strength += 3.into();
            }
            (_, WeaponStyle::Defensive) => boost.defence += 3.into(),
            (_, WeaponStyle::Controlled) => {
                boost.attack += 1.into();
                boost.strength += 1.into();
                boost.defence += 1.into();
            }
            (StyleType::Ranged, WeaponStyle::Accurate | WeaponStyle::ShortFuse) => {
                boost.ranged += 3.into();
            }
            (StyleType::Ranged, WeaponStyle::Rapid | WeaponStyle::MediumFuse) => {
                boost.attack_speed -= 1.into();
            }
            (StyleType::Ranged, WeaponStyle::Longrange) => {
                boost.defence += 3.into();
                boost.attack_range += 2.into();
            }
            (_, WeaponStyle::LongFuse) => boost.attack_range += 1.into(),
            (StyleType::Magic, WeaponStyle::Accurate) => boost.magic += 3.into(),
            (StyleType::Magic, WeaponStyle::Longrange) => {
                boost.magic += 1.into();
                boost.defence += 3.into();
                boost.attack_range += 2.into();
            }
            (StyleType::Magic, WeaponStyle::Autocast | WeaponStyle::DefensiveAutocast)
            | (StyleType::None, WeaponStyle::None) => (),
            _ => return Err(anyhow!("Not interested")),
        };
        Ok(boost)
    }
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum WeaponType {
    TwoHandedSword,
    Axe,
    Banner,
    Blunt,
    Bludgeon,
    Bulwark,
    Claw,
    Partisan,
    Pickaxe,
    Polearm,
    Polestaff,
    Scythe,
    SlashSword,
    Spear,
    Spiked,
    StabSword,
    Unarmed,
    Whip,
    Bow,
    Chinchompa,
    Crossbow,
    Gun,
    Thrown,
    BladedStaff,
    PoweredStaff,
    PoweredWand,
    Staff,
    Salamander,
}

macro_rules! create_combat_options {
    ($(($name:expr, $style_type:ident, $weapon_style:ident)),*) => {
        {
            let mut v = Vec::new();
            $(
                v.push(CombatOption::new($name, StyleType::$style_type, WeaponStyle::$weapon_style));
            )*
            v
        }
    };
}

impl WeaponType {
    #[allow(clippy::too_many_lines)]
    pub fn combat_boost(self) -> Vec<CombatOption> {
        #[allow(clippy::vec_init_then_push)]
        match self {
            Self::TwoHandedSword => create_combat_options!(
                ("Chop", Slash, Accurate),
                ("Slash", Slash, Aggressive),
                ("Smash", Crush, Aggressive),
                ("Block", Slash, Defensive)
            ),
            Self::Axe => create_combat_options!(
                ("Chop", Slash, Accurate),
                ("Hack", Slash, Aggressive),
                ("Smash", Crush, Aggressive),
                ("Block", Slash, Defensive)
            ),
            Self::Banner => create_combat_options!(
                ("Lunge", Stab, Accurate),
                ("Swipe", Slash, Aggressive),
                ("Pound", Crush, Controlled),
                ("Block", Stab, Defensive)
            ),
            Self::Blunt => create_combat_options!(
                ("Pound", Crush, Accurate),
                ("Pummel", Crush, Aggressive),
                ("Block", Crush, Defensive)
            ),
            Self::Bludgeon => create_combat_options!(
                ("Pound", Crush, Aggressive),
                ("Pummel", Crush, Aggressive),
                ("Block", Crush, Aggressive)
            ),
            Self::Bulwark => {
                create_combat_options!(("Pummel", Crush, Accurate), ("Block", None, None))
            }
            Self::Claw | Self::SlashSword => create_combat_options!(
                ("Chop", Slash, Accurate),
                ("Slash", Slash, Aggressive),
                ("Lunge", Stab, Controlled),
                ("Block", Slash, Defensive)
            ),
            Self::Partisan => create_combat_options!(
                ("Stab", Stab, Accurate),
                ("Lunge", Stab, Aggressive),
                ("Pound", Crush, Aggressive),
                ("Block", Stab, Defensive)
            ),
            Self::Pickaxe => create_combat_options!(
                ("Spike", Stab, Accurate),
                ("Impale", Stab, Aggressive),
                ("Smash", Crush, Aggressive),
                ("Block", Stab, Defensive)
            ),
            Self::Polearm => create_combat_options!(
                ("Jab", Stab, Controlled),
                ("Swipe", Slash, Aggressive),
                ("Fend", Stab, Defensive)
            ),
            Self::Polestaff => create_combat_options!(
                ("Bash", Crush, Accurate),
                ("Pound", Crush, Aggressive),
                ("Block", Crush, Defensive)
            ),
            Self::Scythe => create_combat_options!(
                ("Reap", Slash, Accurate),
                ("Chop", Slash, Aggressive),
                ("Jab", Crush, Aggressive),
                ("Block", Slash, Defensive)
            ),
            Self::Spear => create_combat_options!(
                ("Lunge", Stab, Controlled),
                ("Swipe", Slash, Controlled),
                ("Pound", Crush, Controlled),
                ("Block", Stab, Defensive)
            ),
            Self::Spiked => create_combat_options!(
                ("Pound", Crush, Accurate),
                ("Pummel", Crush, Aggressive),
                ("Spike", Stab, Controlled),
                ("Block", Crush, Defensive)
            ),
            Self::StabSword => create_combat_options!(
                ("Stab", Stab, Accurate),
                ("Lunge", Stab, Aggressive),
                ("Slash", Slash, Aggressive),
                ("Block", Stab, Defensive)
            ),
            Self::Unarmed => create_combat_options!(
                ("Punch", Crush, Accurate),
                ("Kick", Crush, Aggressive),
                ("Block", Crush, Defensive)
            ),
            Self::Whip => create_combat_options!(
                ("Flick", Slash, Accurate),
                ("Lash", Slash, Controlled),
                ("Deflect", Slash, Defensive)
            ),
            Self::Bow | Self::Crossbow | Self::Thrown => create_combat_options!(
                ("Accurate", Ranged, Accurate),
                ("Rapid", Ranged, Rapid),
                ("Longrange", Ranged, Longrange)
            ),
            Self::Chinchompa => create_combat_options!(
                ("Short fuse", Ranged, ShortFuse),
                ("Medium fuse", Ranged, MediumFuse),
                ("Long fuse", Ranged, LongFuse)
            ),
            Self::Gun => {
                create_combat_options!(("Aim and Fire", None, None), ("Kick", Crush, Aggressive))
            }
            Self::BladedStaff => create_combat_options!(
                ("Jab", Stab, Accurate),
                ("Swipe", Slash, Aggressive),
                ("Fend", Crush, Defensive),
                ("Spell", Magic, Autocast),
                ("Spell", Magic, DefensiveAutocast)
            ),
            Self::PoweredStaff | Self::PoweredWand => create_combat_options!(
                ("Accurate", Magic, Accurate),
                ("Accurate", Magic, Accurate),
                ("Longrange", Magic, Longrange)
            ),
            Self::Staff => create_combat_options!(
                ("Bash", Crush, Accurate),
                ("Pound", Crush, Aggressive),
                ("Focus", Crush, Defensive),
                ("Spell", Magic, Autocast),
                ("Spell", Magic, DefensiveAutocast)
            ),
            Self::Salamander => create_combat_options!(
                ("Scorch", Slash, Aggressive),
                ("Flare", Ranged, Accurate),
                ("Blaze", Magic, Defensive)
            ),
        }
    }
}
