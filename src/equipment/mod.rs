mod weapon_callbacks;

use crate::{
    equipment::weapon_callbacks::{
        colossal_blade, dragon_hunter_crossbow_accuracy, dragon_hunter_crossbow_max_hit,
        harmonised_nightmare_staff_attack_speed, identity, salve_amulet,
    },
    generics::{NamedData, Percentage, Scalar, Ticks, Tiles},
    unit::{Enemy, Player},
};
use lazy_static::lazy_static;
use serde::Deserialize;

pub trait HasStats: for<'a> Deserialize<'a> {
    fn inner(&self) -> &Equipment;
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
    HarmonisedNightmareStaff,
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

    pub fn attack_speed_callback(&self) -> fn(Ticks, &Player, &Enemy) -> Ticks {
        match self {
            Self::HarmonisedNightmareStaff => harmonised_nightmare_staff_attack_speed,
            _ => identity,
        }
    }
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct Equipment {
    pub name: String,
    #[serde(flatten)]
    pub stats: Stats,
    pub attributes: Vec<Attribute>,
}

macro_rules! equipment_struct {
    ($($struct_name:ident)*) => {
        $(
            #[derive(Deserialize, Debug, Clone)]
            pub struct $struct_name {
                #[serde(flatten)]
                pub inner: Equipment,
            }

            impl HasStats for $struct_name {
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
    ($($struct_name:ident)*) => {
        $(
            #[derive(Deserialize, Debug, Clone)]
            pub struct $struct_name {
                #[serde(flatten)]
                pub inner: Equipment,
                pub weapon_stats: WeaponStats,
                pub powered_staff_type: Option<PoweredStaff>
            }

            impl HasStats for $struct_name {
                fn inner(&self) -> &Equipment {
                    &self.inner
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

equipment_struct!(Head Cape Neck Ammunition Shield Body Legs Hands Feet Ring);
weapon_struct!(WeaponOneHanded WeaponTwoHanded);

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
                weapon.unwrap_or_default().inner.stats
                    + shield.unwrap_or(&DEFAULT_ITEM_SHIELD).inner.stats
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
        let tick_offset = combat_style.invisible_boost().attack_speed;

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

impl HasStats for Slots {
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

lazy_static! {
    static ref DEFAULT_ITEM_EQUIPMENT: Equipment = Equipment {
        name: "Empty".to_string(),
        ..Default::default()
    };
    static ref DEFAULT_ITEM_WEAPON_ONE_HANDED: WeaponOneHanded = WeaponOneHanded {
        inner: DEFAULT_ITEM_EQUIPMENT.clone(),
        ..Default::default()
    };
    static ref DEFAULT_ITEM_WEAPON_TWO_HANDED: WeaponTwoHanded = WeaponTwoHanded {
        inner: DEFAULT_ITEM_EQUIPMENT.clone(),
        ..Default::default()
    };
    static ref DEFAULT_ITEM_SHIELD: Shield = Shield {
        inner: DEFAULT_ITEM_EQUIPMENT.clone()
    };
    static ref DEFAULT_ITEM_HEAD: Head = Head {
        inner: DEFAULT_ITEM_EQUIPMENT.clone()
    };
    static ref DEFAULT_ITEM_CAPE: Cape = Cape {
        inner: DEFAULT_ITEM_EQUIPMENT.clone()
    };
    static ref DEFAULT_ITEM_NECK: Neck = Neck {
        inner: DEFAULT_ITEM_EQUIPMENT.clone()
    };
    static ref DEFAULT_ITEM_AMMUNITION: Ammunition = Ammunition {
        inner: DEFAULT_ITEM_EQUIPMENT.clone()
    };
    static ref DEFAULT_ITEM_BODY: Body = Body {
        inner: DEFAULT_ITEM_EQUIPMENT.clone()
    };
    static ref DEFAULT_ITEM_LEGS: Legs = Legs {
        inner: DEFAULT_ITEM_EQUIPMENT.clone()
    };
    static ref DEFAULT_ITEM_HANDS: Hands = Hands {
        inner: DEFAULT_ITEM_EQUIPMENT.clone()
    };
    static ref DEFAULT_ITEM_FEET: Feet = Feet {
        inner: DEFAULT_ITEM_EQUIPMENT.clone()
    };
    static ref DEFAULT_ITEM_RING: Ring = Ring {
        inner: DEFAULT_ITEM_EQUIPMENT.clone()
    };
}

impl Default for &WeaponOneHanded {
    fn default() -> Self {
        &DEFAULT_ITEM_WEAPON_ONE_HANDED
    }
}

impl Default for &WeaponTwoHanded {
    fn default() -> Self {
        &DEFAULT_ITEM_WEAPON_TWO_HANDED
    }
}

impl Default for &Shield {
    fn default() -> Self {
        &DEFAULT_ITEM_SHIELD
    }
}

impl Default for &Head {
    fn default() -> Self {
        &DEFAULT_ITEM_HEAD
    }
}

impl Default for &Cape {
    fn default() -> Self {
        &DEFAULT_ITEM_CAPE
    }
}

impl Default for &Neck {
    fn default() -> Self {
        &DEFAULT_ITEM_NECK
    }
}

impl Default for &Ammunition {
    fn default() -> Self {
        &DEFAULT_ITEM_AMMUNITION
    }
}

impl Default for &Body {
    fn default() -> Self {
        &DEFAULT_ITEM_BODY
    }
}

impl Default for &Legs {
    fn default() -> Self {
        &DEFAULT_ITEM_LEGS
    }
}

impl Default for &Hands {
    fn default() -> Self {
        &DEFAULT_ITEM_HANDS
    }
}

impl Default for &Feet {
    fn default() -> Self {
        &DEFAULT_ITEM_FEET
    }
}

impl Default for &Ring {
    fn default() -> Self {
        &DEFAULT_ITEM_RING
    }
}
