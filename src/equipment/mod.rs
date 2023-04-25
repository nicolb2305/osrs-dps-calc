mod weapon_callbacks;

use std::ops::Add;

use crate::{
    equipment::weapon_callbacks::{
        colossal_blade, dragon_hunter_crossbow_accuracy, dragon_hunter_crossbow_max_hit, identity,
        salve_amulet,
    },
    generics::{NamedData, Percentage, Scalar, Ticks, Tiles},
    unit::{Enemy, Player},
};
use serde::Deserialize;

pub trait HasStats: for<'a> Deserialize<'a> {
    fn name(&self) -> &str;
    fn attack(&self) -> StatBonuses;
    fn defence(&self) -> StatBonuses;
    fn damage(&self) -> DamageBonus;
    fn prayer_bonus(&self) -> Scalar;
}

pub trait IsWeapon {
    fn weapon_stats(&self) -> WeaponStats;
}

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
}

impl Attribute {
    pub fn accuracy_roll_callback(&self) -> fn(Scalar, &Player, &Enemy) -> Scalar {
        match self {
            Self::DragonHunterCrossbow => dragon_hunter_crossbow_accuracy,
            Self::SalveAmulet => salve_amulet,
            _ => identity,
        }
    }

    pub fn max_hit_callback(&self) -> fn(Scalar, &Player, &Enemy) -> Scalar {
        match self {
            Self::DragonHunterCrossbow => dragon_hunter_crossbow_max_hit,
            Self::SalveAmulet => salve_amulet,
            Self::ColossalBlade => colossal_blade,
            _ => identity,
        }
    }
}

macro_rules! equipment_struct {
    ($($struct_name:tt)*) => {
        $(
            #[derive(Deserialize, Debug, Clone)]
            pub struct $struct_name {
                pub name: String,
                #[serde(flatten)]
                pub stats: Stats,
                pub attributes: Vec<Attribute>,
            }

            impl HasStats for $struct_name {
                fn name(&self) -> &str {
                    &self.name
                }

                fn attack(&self) -> StatBonuses {
                    self.stats.attack
                }

                fn defence(&self) -> StatBonuses {
                    self.stats.defence
                }

                fn damage(&self) -> DamageBonus {
                    self.stats.damage
                }

                fn prayer_bonus(&self) -> Scalar {
                    self.stats.prayer_bonus
                }
            }

            impl Default for $struct_name {
                fn default() -> Self {
                    Self {
                        name: "Empty".to_owned(),
                        stats: Stats::default(),
                        attributes: Vec::default()
                    }
                }
            }
        )*
    };
}

macro_rules! weapon_struct {
    ($($struct_name:tt)*) => {
        $(
            #[derive(Deserialize, Debug, Clone)]
            pub struct $struct_name {
                pub name: String,
                #[serde(flatten)]
                pub stats: Stats,
                pub attributes: Vec<Attribute>,
                pub weapon_stats: WeaponStats,
            }

            impl HasStats for $struct_name {
                fn name(&self) -> &str {
                    &self.name
                }

                fn attack(&self) -> StatBonuses {
                    self.stats.attack
                }

                fn defence(&self) -> StatBonuses {
                    self.stats.defence
                }

                fn damage(&self) -> DamageBonus {
                    self.stats.damage
                }

                fn prayer_bonus(&self) -> Scalar {
                    self.stats.prayer_bonus
                }
            }

            impl IsWeapon for $struct_name {
                fn weapon_stats(&self) -> WeaponStats {
                    self.weapon_stats
                }
            }

            impl Default for $struct_name {
                fn default() -> Self {
                    Self {
                        name: "Empty".to_owned(),
                        stats: Stats::default(),
                        attributes: Vec::default(),
                        weapon_stats: WeaponStats::default(),
                    }
                }
            }
        )*
    };
}

equipment_struct!(Head Cape Neck Ammunition Shield Body Legs Hands Feet Ring);
weapon_struct!(WeaponOneHanded WeaponTwoHanded);

#[derive(Debug, Deserialize, Default, Clone, Copy)]
pub struct Stats {
    pub attack: StatBonuses,
    pub defence: StatBonuses,
    pub damage: DamageBonus,
    pub prayer_bonus: Scalar,
}

impl Add for Stats {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            attack: self.attack + rhs.attack,
            defence: self.defence + rhs.defence,
            damage: self.damage + rhs.damage,
            prayer_bonus: self.prayer_bonus + rhs.prayer_bonus,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub enum Wielded {
    OneHanded {
        weapon: WeaponOneHanded,
        shield: Shield,
    },
    TwoHanded {
        weapon: WeaponTwoHanded,
    },
}

impl Default for Wielded {
    fn default() -> Self {
        Self::OneHanded {
            weapon: WeaponOneHanded::default(),
            shield: Shield::default(),
        }
    }
}

impl Wielded {
    pub fn equip_one_handed(weapon: Option<WeaponOneHanded>, shield: Option<Shield>) -> Wielded {
        Self::OneHanded {
            weapon: weapon.unwrap_or_default(),
            shield: shield.unwrap_or_default(),
        }
    }

    pub fn equip_two_handed(weapon: Option<WeaponTwoHanded>) -> Wielded {
        Self::TwoHanded {
            weapon: weapon.unwrap_or_default(),
        }
    }

    pub fn combat_boost(&self) -> Vec<CombatOption> {
        match self {
            Self::OneHanded { weapon, shield: _ } => weapon.weapon_stats.weapon_type.combat_boost(),
            Self::TwoHanded { weapon } => weapon.weapon_stats.weapon_type.combat_boost(),
        }
    }

    pub fn stats(&self) -> Stats {
        match self {
            Self::OneHanded { weapon, shield } => weapon.stats + shield.stats,
            Self::TwoHanded { weapon } => weapon.stats,
        }
    }

    pub fn weapon_stats(&self) -> WeaponStats {
        match self {
            Self::OneHanded { weapon, shield: _ } => weapon.weapon_stats,
            Self::TwoHanded { weapon } => weapon.weapon_stats,
        }
    }

    pub fn attack_speed(&self, combat_style: &CombatOption) -> Ticks {
        let tick_offset = combat_style.invisible_boost().attack_speed;

        let weapon_attack_speed = match self {
            Self::OneHanded { weapon, shield: _ } => weapon.weapon_stats.attack_speed,
            Self::TwoHanded { weapon } => weapon.weapon_stats.attack_speed,
        };

        weapon_attack_speed + tick_offset
    }

    pub fn attributes(&self) -> &Vec<Attribute> {
        match self {
            Self::OneHanded { weapon, shield: _ } => &weapon.attributes,
            Self::TwoHanded { weapon } => &weapon.attributes,
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

impl HasStats for Slots {
    fn name(&self) -> &str {
        match self {
            Self::Head(v) => v.name(),
            Self::Cape(v) => v.name(),
            Self::Neck(v) => v.name(),
            Self::Ammunition(v) => v.name(),
            Self::WeaponOneHanded(v) => v.name(),
            Self::WeaponTwoHanded(v) => v.name(),
            Self::Shield(v) => v.name(),
            Self::Body(v) => v.name(),
            Self::Legs(v) => v.name(),
            Self::Hands(v) => v.name(),
            Self::Feet(v) => v.name(),
            Self::Ring(v) => v.name(),
        }
    }

    fn attack(&self) -> StatBonuses {
        match self {
            Self::Head(v) => v.attack(),
            Self::Cape(v) => v.attack(),
            Self::Neck(v) => v.attack(),
            Self::Ammunition(v) => v.attack(),
            Self::WeaponOneHanded(v) => v.attack(),
            Self::WeaponTwoHanded(v) => v.attack(),
            Self::Shield(v) => v.attack(),
            Self::Body(v) => v.attack(),
            Self::Legs(v) => v.attack(),
            Self::Hands(v) => v.attack(),
            Self::Feet(v) => v.attack(),
            Self::Ring(v) => v.attack(),
        }
    }

    fn defence(&self) -> StatBonuses {
        match self {
            Self::Head(v) => v.defence(),
            Self::Cape(v) => v.defence(),
            Self::Neck(v) => v.defence(),
            Self::Ammunition(v) => v.defence(),
            Self::WeaponOneHanded(v) => v.defence(),
            Self::WeaponTwoHanded(v) => v.defence(),
            Self::Shield(v) => v.defence(),
            Self::Body(v) => v.defence(),
            Self::Legs(v) => v.defence(),
            Self::Hands(v) => v.defence(),
            Self::Feet(v) => v.defence(),
            Self::Ring(v) => v.defence(),
        }
    }

    fn damage(&self) -> DamageBonus {
        match self {
            Self::Head(v) => v.damage(),
            Self::Cape(v) => v.damage(),
            Self::Neck(v) => v.damage(),
            Self::Ammunition(v) => v.damage(),
            Self::WeaponOneHanded(v) => v.damage(),
            Self::WeaponTwoHanded(v) => v.damage(),
            Self::Shield(v) => v.damage(),
            Self::Body(v) => v.damage(),
            Self::Legs(v) => v.damage(),
            Self::Hands(v) => v.damage(),
            Self::Feet(v) => v.damage(),
            Self::Ring(v) => v.damage(),
        }
    }

    fn prayer_bonus(&self) -> Scalar {
        match self {
            Self::Head(v) => v.prayer_bonus(),
            Self::Cape(v) => v.prayer_bonus(),
            Self::Neck(v) => v.prayer_bonus(),
            Self::Ammunition(v) => v.prayer_bonus(),
            Self::WeaponOneHanded(v) => v.prayer_bonus(),
            Self::WeaponTwoHanded(v) => v.prayer_bonus(),
            Self::Shield(v) => v.prayer_bonus(),
            Self::Body(v) => v.prayer_bonus(),
            Self::Legs(v) => v.prayer_bonus(),
            Self::Hands(v) => v.prayer_bonus(),
            Self::Feet(v) => v.prayer_bonus(),
            Self::Ring(v) => v.prayer_bonus(),
        }
    }
}

impl NamedData for Slots {
    fn get_name(&self) -> &str {
        self.name()
    }
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct StatBonuses {
    pub stab: Scalar,
    pub slash: Scalar,
    pub crush: Scalar,
    pub ranged: Scalar,
    pub magic: Scalar,
}

impl Add for StatBonuses {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            stab: self.stab + rhs.stab,
            slash: self.slash + rhs.slash,
            crush: self.crush + rhs.crush,
            ranged: self.ranged + rhs.ranged,
            magic: self.magic + rhs.magic,
        }
    }
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

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct DamageBonus {
    pub strength: Scalar,
    pub ranged: Scalar,
    pub magic: Percentage,
}

impl Add for DamageBonus {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            strength: self.strength + rhs.strength,
            ranged: self.ranged + rhs.ranged,
            magic: self.magic + rhs.magic,
        }
    }
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

#[derive(Debug, Clone, Default)]
pub enum StyleType {
    Slash,
    #[default]
    Crush,
    Stab,
    Ranged,
    Magic,
    None,
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

impl CombatOption {
    #[allow(clippy::missing_panics_doc)]
    pub fn invisible_boost(&self) -> CombatOptionModifier {
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
            _ => panic!("Not a valid weapon!"),
        };
        boost
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

impl WeaponType {
    #[allow(clippy::too_many_lines)]
    pub fn combat_boost(&self) -> Vec<CombatOption> {
        match self {
            Self::TwoHandedSword => vec![
                CombatOption::new("Chop", StyleType::Slash, WeaponStyle::Accurate),
                CombatOption::new("Slash", StyleType::Slash, WeaponStyle::Aggressive),
                CombatOption::new("Smash", StyleType::Crush, WeaponStyle::Aggressive),
                CombatOption::new("Block", StyleType::Slash, WeaponStyle::Defensive),
            ],
            Self::Axe => vec![
                CombatOption::new("Chop", StyleType::Slash, WeaponStyle::Accurate),
                CombatOption::new("Hack", StyleType::Slash, WeaponStyle::Aggressive),
                CombatOption::new("Smash", StyleType::Crush, WeaponStyle::Aggressive),
                CombatOption::new("Block", StyleType::Slash, WeaponStyle::Defensive),
            ],
            Self::Banner => vec![
                CombatOption::new("Lunge", StyleType::Stab, WeaponStyle::Accurate),
                CombatOption::new("Swipe", StyleType::Slash, WeaponStyle::Aggressive),
                CombatOption::new("Pound", StyleType::Crush, WeaponStyle::Controlled),
                CombatOption::new("Block", StyleType::Stab, WeaponStyle::Defensive),
            ],
            Self::Blunt => vec![
                CombatOption::new("Pound", StyleType::Crush, WeaponStyle::Accurate),
                CombatOption::new("Pummel", StyleType::Crush, WeaponStyle::Aggressive),
                CombatOption::new("Block", StyleType::Crush, WeaponStyle::Defensive),
            ],
            Self::Bludgeon => vec![
                CombatOption::new("Pound", StyleType::Crush, WeaponStyle::Aggressive),
                CombatOption::new("Pummel", StyleType::Crush, WeaponStyle::Aggressive),
                CombatOption::new("Block", StyleType::Crush, WeaponStyle::Aggressive),
            ],
            Self::Bulwark => vec![
                CombatOption::new("Pummel", StyleType::Crush, WeaponStyle::Accurate),
                CombatOption::new("Block", StyleType::None, WeaponStyle::None),
            ],
            Self::Claw | Self::SlashSword => vec![
                CombatOption::new("Chop", StyleType::Slash, WeaponStyle::Accurate),
                CombatOption::new("Slash", StyleType::Slash, WeaponStyle::Aggressive),
                CombatOption::new("Lunge", StyleType::Stab, WeaponStyle::Controlled),
                CombatOption::new("Block", StyleType::Slash, WeaponStyle::Defensive),
            ],
            Self::Partisan => vec![
                CombatOption::new("Stab", StyleType::Stab, WeaponStyle::Accurate),
                CombatOption::new("Lunge", StyleType::Stab, WeaponStyle::Aggressive),
                CombatOption::new("Pound", StyleType::Crush, WeaponStyle::Aggressive),
                CombatOption::new("Block", StyleType::Stab, WeaponStyle::Defensive),
            ],
            Self::Pickaxe => vec![
                CombatOption::new("Spike", StyleType::Stab, WeaponStyle::Accurate),
                CombatOption::new("Impale", StyleType::Stab, WeaponStyle::Aggressive),
                CombatOption::new("Smash", StyleType::Crush, WeaponStyle::Aggressive),
                CombatOption::new("Block", StyleType::Stab, WeaponStyle::Defensive),
            ],
            Self::Polearm => vec![
                CombatOption::new("Jab", StyleType::Stab, WeaponStyle::Controlled),
                CombatOption::new("Swipe", StyleType::Slash, WeaponStyle::Aggressive),
                CombatOption::new("Fend", StyleType::Stab, WeaponStyle::Defensive),
            ],
            Self::Polestaff => vec![
                CombatOption::new("Bash", StyleType::Crush, WeaponStyle::Accurate),
                CombatOption::new("Pound", StyleType::Crush, WeaponStyle::Aggressive),
                CombatOption::new("Block", StyleType::Crush, WeaponStyle::Defensive),
            ],
            Self::Scythe => vec![
                CombatOption::new("Reap", StyleType::Slash, WeaponStyle::Accurate),
                CombatOption::new("Chop", StyleType::Slash, WeaponStyle::Aggressive),
                CombatOption::new("Jab", StyleType::Crush, WeaponStyle::Aggressive),
                CombatOption::new("Block", StyleType::Slash, WeaponStyle::Defensive),
            ],
            Self::Spear => vec![
                CombatOption::new("Lunge", StyleType::Stab, WeaponStyle::Controlled),
                CombatOption::new("Swipe", StyleType::Slash, WeaponStyle::Controlled),
                CombatOption::new("Pound", StyleType::Crush, WeaponStyle::Controlled),
                CombatOption::new("Block", StyleType::Stab, WeaponStyle::Defensive),
            ],
            Self::Spiked => vec![
                CombatOption::new("Pound", StyleType::Crush, WeaponStyle::Accurate),
                CombatOption::new("Pummel", StyleType::Crush, WeaponStyle::Aggressive),
                CombatOption::new("Spike", StyleType::Stab, WeaponStyle::Controlled),
                CombatOption::new("Block", StyleType::Crush, WeaponStyle::Defensive),
            ],
            Self::StabSword => vec![
                CombatOption::new("Stab", StyleType::Stab, WeaponStyle::Accurate),
                CombatOption::new("Lunge", StyleType::Stab, WeaponStyle::Aggressive),
                CombatOption::new("Slash", StyleType::Slash, WeaponStyle::Aggressive),
                CombatOption::new("Block", StyleType::Stab, WeaponStyle::Defensive),
            ],
            Self::Unarmed => vec![
                CombatOption::new("Punch", StyleType::Crush, WeaponStyle::Accurate),
                CombatOption::new("Kick", StyleType::Crush, WeaponStyle::Aggressive),
                CombatOption::new("Block", StyleType::Crush, WeaponStyle::Defensive),
            ],
            Self::Whip => vec![
                CombatOption::new("Flick", StyleType::Slash, WeaponStyle::Accurate),
                CombatOption::new("Lash", StyleType::Slash, WeaponStyle::Controlled),
                CombatOption::new("Deflect", StyleType::Slash, WeaponStyle::Defensive),
            ],
            Self::Bow | Self::Crossbow | Self::Thrown => vec![
                CombatOption::new("Accurate", StyleType::Ranged, WeaponStyle::Accurate),
                CombatOption::new("Rapid", StyleType::Ranged, WeaponStyle::Rapid),
                CombatOption::new("Longrange", StyleType::Ranged, WeaponStyle::Longrange),
            ],
            Self::Chinchompa => vec![
                CombatOption::new("Short fuse", StyleType::Ranged, WeaponStyle::ShortFuse),
                CombatOption::new("Medium fuse", StyleType::Ranged, WeaponStyle::MediumFuse),
                CombatOption::new("Long fuse", StyleType::Ranged, WeaponStyle::LongFuse),
            ],
            Self::Gun => vec![
                CombatOption::new("Aim and Fire", StyleType::None, WeaponStyle::None),
                CombatOption::new("Kick", StyleType::Crush, WeaponStyle::Aggressive),
            ],
            Self::BladedStaff => vec![
                CombatOption::new("Jab", StyleType::Stab, WeaponStyle::Accurate),
                CombatOption::new("Swipe", StyleType::Slash, WeaponStyle::Aggressive),
                CombatOption::new("Fend", StyleType::Crush, WeaponStyle::Defensive),
                CombatOption::new("Spell", StyleType::Magic, WeaponStyle::Autocast),
                CombatOption::new("Spell", StyleType::Magic, WeaponStyle::DefensiveAutocast),
            ],
            Self::PoweredStaff | Self::PoweredWand => vec![
                CombatOption::new("Accurate", StyleType::Magic, WeaponStyle::Accurate),
                CombatOption::new("Accurate", StyleType::Magic, WeaponStyle::Accurate),
                CombatOption::new("Longrange", StyleType::Magic, WeaponStyle::Longrange),
            ],
            Self::Staff => vec![
                CombatOption::new("Bash", StyleType::Crush, WeaponStyle::Accurate),
                CombatOption::new("Pound", StyleType::Crush, WeaponStyle::Aggressive),
                CombatOption::new("Focus", StyleType::Crush, WeaponStyle::Defensive),
                CombatOption::new("Spell", StyleType::Magic, WeaponStyle::Autocast),
                CombatOption::new("Spell", StyleType::Magic, WeaponStyle::DefensiveAutocast),
            ],
            Self::Salamander => vec![
                CombatOption::new("Scorch", StyleType::Slash, WeaponStyle::Aggressive),
                CombatOption::new("Flare", StyleType::Ranged, WeaponStyle::Accurate),
                CombatOption::new("Blaze", StyleType::Magic, WeaponStyle::Defensive),
            ],
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
