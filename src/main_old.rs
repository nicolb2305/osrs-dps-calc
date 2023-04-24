#![allow(dead_code)]

use std::collections::HashMap;

use generics::NamedData;

use crate::{
    items::{Equipment, EquipmentSlot},
    player::{Gear, Player, Stats},
    prayers::Prayer,
};

pub mod generics {
    use serde::Deserialize;

    pub trait NamedData: for<'a> Deserialize<'a> {
        fn get_name(&self) -> &str;
    }

    #[derive(Deserialize, Debug, Clone, Copy)]
    pub struct Fraction {
        dividend: i32,
        divisor: i32,
    }

    impl std::ops::Mul<Scalar> for Fraction {
        type Output = Scalar;

        fn mul(self, rhs: Scalar) -> Self::Output {
            Scalar((self.dividend * rhs.0) / self.divisor)
        }
    }

    #[derive(Deserialize, Debug, Clone, Copy, PartialEq, PartialOrd)]
    pub struct Percentage(i32);

    impl std::ops::Mul<Scalar> for Percentage {
        type Output = Scalar;

        fn mul(self, rhs: Scalar) -> Self::Output {
            Scalar((self.0 * rhs.0) / 100)
        }
    }

    impl From<i32> for Percentage {
        fn from(value: i32) -> Self {
            Percentage(value)
        }
    }

    #[derive(Deserialize, Debug, Clone, Copy)]
    pub struct Scalar(i32);

    impl std::ops::Mul<Scalar> for Scalar {
        type Output = Scalar;

        fn mul(self, rhs: Scalar) -> Self::Output {
            Scalar((self.0 * rhs.0) / 100)
        }
    }

    impl std::ops::Mul<Percentage> for Scalar {
        type Output = Scalar;

        fn mul(self, rhs: Percentage) -> Self::Output {
            Scalar((self.0 * rhs.0) / 100)
        }
    }

    impl std::ops::Add for Scalar {
        type Output = Scalar;

        fn add(self, rhs: Self) -> Self::Output {
            Scalar(self.0 + rhs.0)
        }
    }

    impl std::ops::AddAssign for Scalar {
        fn add_assign(&mut self, rhs: Self) {
            self.0 += rhs.0;
        }
    }

    impl From<i32> for Scalar {
        fn from(value: i32) -> Self {
            Self(value)
        }
    }

    #[derive(Deserialize, Debug, Clone, Copy)]
    pub struct Tiles(i32);

    #[derive(Deserialize, Debug, Clone, Copy)]
    pub struct Ticks(i32);
}

pub mod items {
    use crate::generics::{NamedData, Percentage, Scalar, Ticks, Tiles};
    use serde::Deserialize;

    #[derive(Deserialize, Debug, Clone)]
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

    #[derive(Debug, Clone)]
    pub enum StyleType {
        Slash,
        Crush,
        Stab,
        Ranged,
        Magic,
        None,
    }

    #[derive(Debug, Clone)]
    pub enum WeaponStyle {
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

    impl CombatOption {
        pub fn new(name: &str, style_type: StyleType, weapon_style: WeaponStyle) -> Self {
            Self {
                name: name.to_owned(),
                style_type,
                weapon_style,
            }
        }
    }

    pub struct CombatOptionModifier {
        pub attack: Scalar,
        pub strength: Scalar,
        pub defence: Scalar,
        pub ranged: Scalar,
        pub magic: Scalar,
        pub attack_range: Scalar,
        pub attack_speed: Scalar,
    }

    impl Default for CombatOptionModifier {
        fn default() -> Self {
            CombatOptionModifier {
                attack: 0.into(),
                strength: 0.into(),
                defence: 0.into(),
                ranged: 0.into(),
                magic: 0.into(),
                attack_range: 0.into(),
                attack_speed: 0.into(),
            }
        }
    }

    impl CombatOption {
        #[allow(clippy::missing_panics_doc)]
        pub fn invisible_boost(&self) -> CombatOptionModifier {
            let mut boost = CombatOptionModifier::default();
            match (&self.style_type, &self.weapon_style) {
                (StyleType::Slash | StyleType::Crush | StyleType::Stab, WeaponStyle::Accurate) => {
                    boost.attack += 3.into();
                }
                (
                    StyleType::Slash | StyleType::Crush | StyleType::Stab,
                    WeaponStyle::Aggressive,
                ) => boost.strength += 3.into(),
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
                    boost.attack_speed += 1.into();
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

    #[derive(Deserialize, Debug, Clone)]
    #[serde(tag = "slot")]
    pub enum EquipmentSlot {
        Head(HeadSlot),
        Cape(CapeSlot),
        Neck(NeckSlot),
        Ammunition(AmmunitionSlot),
        Weapon(WeaponSlot),
        Shield(ShieldSlot),
        Body(BodySlot),
        Legs(LegsSlot),
        Hands(HandsSlot),
        Feet(FeetSlot),
        Ring(RingSlot),
    }

    impl NamedData for EquipmentSlot {
        fn get_name(&self) -> &str {
            match self {
                Self::Head(v) => &v.name,
                Self::Cape(v) => &v.name,
                Self::Neck(v) => &v.name,
                Self::Ammunition(v) => &v.name,
                Self::Weapon(v) => &v.name,
                Self::Shield(v) => &v.name,
                Self::Body(v) => &v.name,
                Self::Legs(v) => &v.name,
                Self::Hands(v) => &v.name,
                Self::Feet(v) => &v.name,
                Self::Ring(v) => &v.name,
            }
        }
    }

    #[derive(Deserialize, Debug, Clone)]
    pub struct Equipment {
        #[serde(flatten)]
        pub slot: EquipmentSlot,
    }

    impl NamedData for Equipment {
        fn get_name(&self) -> &str {
            self.slot.get_name()
        }
    }

    #[derive(Deserialize, Debug, Clone)]
    pub struct SharedEquipmentValues {
        pub attack: StatBonuses,
        pub defence: StatBonuses,
        pub damage: DamageBonus,
        pub prayer_bonus: Scalar,
    }

    #[derive(Deserialize, Debug, Clone)]
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

    #[derive(Deserialize, Debug, Clone)]
    pub struct DamageBonus {
        pub strength: Scalar,
        pub ranged: Percentage,
        pub magic: Percentage,
    }

    #[derive(Deserialize, Debug, Clone)]
    pub struct HeadSlot {
        pub name: String,
        #[serde(flatten)]
        pub shared: SharedEquipmentValues,
    }

    #[derive(Deserialize, Debug, Clone)]
    pub struct CapeSlot {
        pub name: String,
        #[serde(flatten)]
        pub shared: SharedEquipmentValues,
    }

    #[derive(Deserialize, Debug, Clone)]
    pub struct NeckSlot {
        pub name: String,
        #[serde(flatten)]
        pub shared: SharedEquipmentValues,
    }

    #[derive(Deserialize, Debug, Clone)]
    pub struct AmmunitionSlot {
        pub name: String,
        #[serde(flatten)]
        pub shared: SharedEquipmentValues,
    }

    #[derive(Deserialize, Debug, Clone)]
    pub struct WeaponSlot {
        pub name: String,
        #[serde(flatten)]
        pub shared: SharedEquipmentValues,
        pub weapon_stats: WeaponStats,
    }

    #[derive(Deserialize, Debug, Clone)]
    pub struct WeaponStats {
        pub name: String,
        pub weapon_type: WeaponType,
        pub attack_speed: Tiles,
        pub range: Ticks,
    }

    #[derive(Deserialize, Debug, Clone)]
    pub struct ShieldSlot {
        pub name: String,
        #[serde(flatten)]
        pub shared: SharedEquipmentValues,
    }

    #[derive(Deserialize, Debug, Clone)]
    pub struct BodySlot {
        pub name: String,
        #[serde(flatten)]
        pub shared: SharedEquipmentValues,
    }

    #[derive(Deserialize, Debug, Clone)]
    pub struct LegsSlot {
        pub name: String,
        #[serde(flatten)]
        pub shared: SharedEquipmentValues,
    }

    #[derive(Deserialize, Debug, Clone)]
    pub struct HandsSlot {
        pub name: String,
        #[serde(flatten)]
        pub shared: SharedEquipmentValues,
    }

    #[derive(Deserialize, Debug, Clone)]
    pub struct FeetSlot {
        pub name: String,
        #[serde(flatten)]
        pub shared: SharedEquipmentValues,
    }

    #[derive(Deserialize, Debug, Clone)]
    pub struct RingSlot {
        pub name: String,
        #[serde(flatten)]
        pub shared: SharedEquipmentValues,
    }
}

pub mod prayers {
    use crate::generics::{NamedData, Percentage};
    use serde::Deserialize;

    #[derive(Deserialize, Debug, Clone)]
    pub struct Prayer {
        pub name: String,
        pub defence: Percentage,
        pub melee_accuracy: Percentage,
        pub melee_damage: Percentage,
        pub ranged_accuracy: Percentage,
        pub ranged_damage: Percentage,
        pub magic_accuracy: Percentage,
        pub magic_defence: Percentage,
    }

    impl NamedData for Prayer {
        fn get_name(&self) -> &str {
            &self.name
        }
    }
}

pub mod player {
    use crate::generics::{Percentage, Scalar};
    use crate::items::{
        AmmunitionSlot, BodySlot, CapeSlot, CombatOption, FeetSlot, HandsSlot, HeadSlot, LegsSlot,
        NeckSlot, RingSlot, ShieldSlot, WeaponSlot,
    };
    use crate::prayers::Prayer;

    #[derive(Debug, Clone)]
    pub struct Gear {
        pub head: Option<HeadSlot>,
        pub cape: Option<CapeSlot>,
        pub neck: Option<NeckSlot>,
        pub ammuntion: Option<AmmunitionSlot>,
        pub weapon: Option<WeaponSlot>,
        pub shield: Option<ShieldSlot>,
        pub body: Option<BodySlot>,
        pub legs: Option<LegsSlot>,
        pub hands: Option<HandsSlot>,
        pub feet: Option<FeetSlot>,
        pub ring: Option<RingSlot>,
    }

    impl Default for Gear {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Gear {
        pub fn new() -> Self {
            Gear {
                head: None,
                cape: None,
                neck: None,
                ammuntion: None,
                weapon: None,
                shield: None,
                body: None,
                legs: None,
                hands: None,
                feet: None,
                ring: None,
            }
        }

        pub fn equip_weapon(&mut self, weapon: WeaponSlot) {
            self.weapon = Some(weapon);
        }
    }

    #[derive(Debug)]
    pub struct Player {
        pub stats: Stats,
        pub active_prayers: Vec<Prayer>,
        pub equipment: Gear,
        pub combat_style: CombatOption,
    }

    impl Player {
        pub fn new(
            stats: Stats,
            active_prayers: Vec<Prayer>,
            equipment: Gear,
            combat_style: CombatOption,
        ) -> Player {
            Player {
                stats,
                active_prayers,
                equipment,
                combat_style,
            }
        }

        pub fn prayer_melee_accuracy(&self) -> Percentage {
            self.active_prayers
                .iter()
                .find(|prayer| prayer.melee_accuracy > 0.into())
                .map_or(0.into(), |prayer| prayer.melee_accuracy)
        }

        pub fn max_melee_accuracy_roll(&self) -> Scalar {
            let mut effective_attack_level = self.stats.attack * self.prayer_melee_accuracy();
            effective_attack_level += self.combat_style.invisible_boost().attack;
            effective_attack_level += 8.into();
            todo!()
        }
    }

    #[derive(Debug)]
    pub struct Stats {
        pub hitpoints: Scalar,
        pub attack: Scalar,
        pub strength: Scalar,
        pub defence: Scalar,
        pub ranged: Scalar,
        pub magic: Scalar,
        pub prayer: Scalar,
    }

    impl Default for Stats {
        fn default() -> Self {
            Stats {
                hitpoints: 0.into(),
                attack: 0.into(),
                strength: 0.into(),
                defence: 0.into(),
                ranged: 0.into(),
                magic: 0.into(),
                prayer: 0.into(),
            }
        }
    }
}

fn read_file<T>(path: &str) -> Result<HashMap<String, T>, Box<dyn std::error::Error>>
where
    T: NamedData,
{
    let data = std::fs::read_to_string(path)?;
    Ok(serde_json::from_str::<Vec<T>>(&data)?
        .into_iter()
        .map(|x| (x.get_name().to_owned(), x))
        .collect::<HashMap<_, _>>())
}

fn main() {
    let items = read_file::<Equipment>("./data/weapons.json").unwrap();
    let prayers = read_file::<Prayer>("./data/prayers.json").unwrap();

    dbg!(&items);
    // dbg!(prayers);
    // dbg!(Percentage(25) * items.get("Abyssal Whip").unwrap().stats.attack.slash);
    // dbg!(Scalar(-3) * Percentage(50));

    let player_stats = Stats {
        attack: 99.into(),
        hitpoints: 99.into(),
        strength: 99.into(),
        defence: 99.into(),
        ranged: 99.into(),
        magic: 99.into(),
        prayer: 99.into(),
    };
    let active_prayers = vec![prayers.get("Piety").unwrap().clone()];
    let mut player_equipment = Gear::new();
    if let Some(equipment) = items.get("Abyssal Whip") {
        if let EquipmentSlot::Weapon(weapon) = &equipment.slot {
            player_equipment.equip_weapon(weapon.clone());
        }
    }
    let combat_style = player_equipment
        .weapon
        .clone()
        .unwrap()
        .weapon_stats
        .weapon_type
        .combat_boost()
        .get(0)
        .unwrap()
        .clone();
    let player = Player::new(player_stats, active_prayers, player_equipment, combat_style);
    dbg!(player);
}
