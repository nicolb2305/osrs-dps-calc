use std::collections::HashMap;

use generics::NamedData;

pub(crate) mod weapon_callbacks {
    use std::cmp::min;

    use crate::{
        equipment::StyleType,
        generics::{Fraction, Scalar},
        unit::{Enemy, Player},
    };

    pub(crate) fn identity(value: Scalar, _player: &Player, _enemy: &Enemy) -> Scalar {
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
            StyleType::Stab | StyleType::Slash | StyleType::Crush
                if enemy.has_attribute(&Undead) =>
            {
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
}

pub mod generics {
    use serde::Deserialize;

    pub const SECONDS_PER_TICK: f64 = 0.6;

    pub trait NamedData: for<'a> Deserialize<'a> {
        fn get_name(&self) -> &str;
    }

    #[derive(Deserialize, Debug, Clone, Copy, Default)]
    pub struct Fraction {
        pub dividend: i32,
        pub divisor: i32,
    }

    impl std::ops::Mul<Scalar> for Fraction {
        type Output = Scalar;

        fn mul(self, rhs: Scalar) -> Self::Output {
            Scalar((self.dividend * rhs.0) / self.divisor)
        }
    }

    #[derive(Deserialize, Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
    pub struct Percentage(i32);

    impl std::ops::Mul<Scalar> for Percentage {
        type Output = Scalar;

        fn mul(self, rhs: Scalar) -> Self::Output {
            Scalar(((100 + self.0) * rhs.0) / 100)
        }
    }

    impl From<i32> for Percentage {
        fn from(value: i32) -> Self {
            Self(value)
        }
    }

    impl std::ops::Add for Percentage {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Self(self.0 + rhs.0)
        }
    }

    #[derive(Deserialize, Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Scalar(i32);

    impl Scalar {
        pub fn new(value: i32) -> Self {
            Self(value)
        }
    }

    impl std::ops::Mul for Scalar {
        type Output = Self;

        fn mul(self, rhs: Scalar) -> Self::Output {
            Self(self.0 * rhs.0)
        }
    }

    impl std::ops::Mul<Fraction> for Scalar {
        type Output = Self;

        fn mul(self, rhs: Fraction) -> Self::Output {
            Self((self.0 * rhs.dividend) / rhs.divisor)
        }
    }

    impl std::ops::Mul<Percentage> for Scalar {
        type Output = Self;

        fn mul(self, rhs: Percentage) -> Self::Output {
            Self((self.0 * (100 + rhs.0)) / 100)
        }
    }

    impl std::ops::Div for Scalar {
        type Output = Self;

        fn div(self, rhs: Self) -> Self::Output {
            Self(self.0 / rhs.0)
        }
    }

    impl std::ops::Add for Scalar {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Self(self.0 + rhs.0)
        }
    }

    impl std::ops::AddAssign for Scalar {
        fn add_assign(&mut self, rhs: Self) {
            self.0 += rhs.0;
        }
    }

    impl std::ops::SubAssign for Scalar {
        fn sub_assign(&mut self, rhs: Self) {
            self.0 -= rhs.0;
        }
    }

    impl From<i32> for Scalar {
        fn from(value: i32) -> Self {
            Self(value)
        }
    }

    impl From<Scalar> for i32 {
        fn from(value: Scalar) -> Self {
            value.0
        }
    }

    impl From<Tiles> for Scalar {
        fn from(value: Tiles) -> Self {
            Self(value.0)
        }
    }

    #[derive(Deserialize, Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Tiles(i32);

    impl From<i32> for Tiles {
        fn from(value: i32) -> Self {
            Self(value)
        }
    }

    impl std::ops::AddAssign for Tiles {
        fn add_assign(&mut self, rhs: Self) {
            self.0 += rhs.0;
        }
    }

    impl From<Scalar> for Tiles {
        fn from(value: Scalar) -> Self {
            Self(value.0)
        }
    }

    #[derive(Deserialize, Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Ticks(i32);

    impl From<i32> for Ticks {
        fn from(value: i32) -> Self {
            Self(value)
        }
    }

    impl std::ops::Add for Ticks {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Self(self.0 + rhs.0)
        }
    }

    impl std::ops::SubAssign for Ticks {
        fn sub_assign(&mut self, rhs: Self) {
            self.0 -= rhs.0;
        }
    }

    impl From<Ticks> for i32 {
        fn from(value: Ticks) -> Self {
            value.0
        }
    }
}

pub mod equipment {
    use std::ops::Add;

    use crate::{
        generics::{NamedData, Percentage, Scalar, Ticks, Tiles},
        unit::{Enemy, Player},
        weapon_callbacks::{
            colossal_blade, dragon_hunter_crossbow_accuracy, dragon_hunter_crossbow_max_hit,
            identity, salve_amulet,
        },
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
        pub fn equip_one_handed(
            weapon: Option<WeaponOneHanded>,
            shield: Option<Shield>,
        ) -> Wielded {
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
                Self::OneHanded { weapon, shield: _ } => {
                    weapon.weapon_stats.weapon_type.combat_boost()
                }
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
}

pub mod prayers {
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
}

pub mod unit {
    use serde::Deserialize;

    use crate::{
        equipment::{
            Ammunition, Body, Cape, CombatOption, Feet, Hands, Head, Legs, Neck, Ring, Shield,
            Slots, Stats, StyleType, WeaponOneHanded, Wielded,
        },
        generics::{NamedData, Scalar, Tiles, SECONDS_PER_TICK},
        prayers::Prayer,
    };

    #[derive(Debug, Deserialize, Clone)]
    pub struct Enemy {
        pub name: String,
        pub levels: Levels,
        pub stats: Stats,
        pub attributes: Vec<EnemyAttribute>,
        pub size: Tiles,
    }

    #[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
    pub enum EnemyAttribute {
        Demon,
        Raid,
        Dragon,
        Golem,
        Vampyre,
        Leafy,
        Undead,
    }

    impl NamedData for Enemy {
        fn get_name(&self) -> &str {
            &self.name
        }
    }

    impl Enemy {
        pub fn max_defence_roll(&self, style_type: &StyleType) -> Scalar {
            let style_defence = match style_type {
                StyleType::Stab => self.stats.defence.stab,
                StyleType::Slash => self.stats.defence.slash,
                StyleType::Crush => self.stats.defence.crush,
                StyleType::Ranged => self.stats.defence.ranged,
                StyleType::Magic => self.stats.defence.magic,
                StyleType::None => unimplemented!(),
            };

            let effective_defence_level = self.levels.defence + 9.into();

            effective_defence_level * (style_defence + 64.into())
        }

        pub fn has_attribute(&self, attribute: &EnemyAttribute) -> bool {
            self.attributes.contains(attribute)
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Extra {
        pub on_slayer_task: bool,
        pub mining_level: Scalar,
        pub in_wilderness: bool,
    }

    impl Default for Extra {
        fn default() -> Self {
            Self {
                on_slayer_task: true,
                mining_level: 99.into(),
                in_wilderness: true,
            }
        }
    }

    #[derive(Debug)]
    pub struct Player {
        pub levels: Levels,
        equipped: Equipped,
        pub active_prayers: Vec<Prayer>,
        combat_option: CombatOption,
        pub extra: Extra,
    }

    impl Player {
        #[must_use]
        pub fn equip_full(mut self, equipped: Equipped) -> Self {
            self.equipped = equipped;
            self.update_combat_option();
            self
        }

        fn update_combat_option(&mut self) {
            self.combat_option = self
                .equipped
                .wielded
                .combat_boost()
                .get(0)
                .expect("Should contain at least 3 options")
                .clone();
        }

        #[must_use]
        pub fn set_levels(mut self, levels: Levels) -> Self {
            self.levels = levels;
            self
        }

        #[must_use]
        pub fn activate_prayer(mut self, prayer: Prayer) -> Self {
            self.active_prayers.push(prayer);
            self
        }

        #[must_use]
        pub fn equip(mut self, slot: Slots) -> Self {
            match slot {
                Slots::Head(head) => self.equipped.head = head,
                Slots::Cape(cape) => self.equipped.cape = cape,
                Slots::Neck(neck) => self.equipped.neck = neck,
                Slots::Ammunition(ammunition) => self.equipped.ammunition = ammunition,
                Slots::WeaponOneHanded(weapon_one_handed) => {
                    match self.equipped.wielded {
                        Wielded::OneHanded {
                            ref mut weapon,
                            shield: _,
                        } => *weapon = weapon_one_handed,
                        Wielded::TwoHanded { weapon: _ } => {
                            self.equipped.wielded = Wielded::OneHanded {
                                weapon: weapon_one_handed,
                                shield: Shield::default(),
                            }
                        }
                    };
                    self.update_combat_option();
                }
                Slots::WeaponTwoHanded(weapon_two_handed) => {
                    match self.equipped.wielded {
                        Wielded::OneHanded {
                            weapon: _,
                            shield: _,
                        } => {
                            self.equipped.wielded = Wielded::TwoHanded {
                                weapon: weapon_two_handed,
                            }
                        }
                        Wielded::TwoHanded { ref mut weapon } => *weapon = weapon_two_handed,
                    };
                    self.update_combat_option();
                }
                Slots::Body(body) => self.equipped.body = body,
                Slots::Shield(new_shield) => {
                    match self.equipped.wielded {
                        Wielded::OneHanded {
                            weapon: _,
                            ref mut shield,
                        } => *shield = new_shield,
                        Wielded::TwoHanded { weapon: _ } => {
                            self.equipped.wielded = Wielded::OneHanded {
                                weapon: WeaponOneHanded::default(),
                                shield: new_shield,
                            }
                        }
                    };
                    self.update_combat_option();
                }
                Slots::Legs(legs) => self.equipped.legs = legs,
                Slots::Hands(hands) => self.equipped.hands = hands,
                Slots::Feet(feet) => self.equipped.feet = feet,
                Slots::Ring(ring) => self.equipped.ring = ring,
            };

            self
        }

        pub fn equipped(&self) -> &Equipped {
            &self.equipped
        }

        pub fn combat_option(&self) -> &CombatOption {
            &self.combat_option
        }

        /// # Errors
        /// Returns an error if the index is invalid for the currently wielded weapon
        pub fn change_combat_style(&mut self, index: usize) -> Result<(), &str> {
            let combat_options = self.equipped.wielded.combat_boost();
            self.combat_option = combat_options.get(index).ok_or("Invalid index")?.clone();
            Ok(())
        }

        pub fn prayer_stats(&self) -> crate::prayers::Stats {
            self.active_prayers
                .iter()
                .fold(crate::prayers::Stats::default(), |acc, p| acc + p.stats)
        }

        pub fn max_melee_accuracy_roll(&self, enemy: &Enemy) -> Scalar {
            let mut effective_attack_level =
                self.levels.attack * self.prayer_stats().melee_accuracy;
            effective_attack_level += self.combat_option.invisible_boost().attack;
            effective_attack_level += 8.into();

            let style_bonus = match self.combat_option.style_type {
                StyleType::Stab => self.equipped.total_stats().attack.stab,
                StyleType::Slash => self.equipped.total_stats().attack.slash,
                StyleType::Crush => self.equipped.total_stats().attack.crush,
                _ => unreachable!(),
            };

            let mut attack_roll = effective_attack_level * (style_bonus + 64.into());

            attack_roll = self
                .equipped
                .accuracy_roll_callback(attack_roll, self, enemy);

            attack_roll
        }

        pub fn max_melee_hit(&self, enemy: &Enemy) -> Scalar {
            let mut effective_strength_level =
                self.levels.strength * self.prayer_stats().melee_damage;
            effective_strength_level += self.combat_option.invisible_boost().strength;
            effective_strength_level += 8.into();

            let mut max_hit = (effective_strength_level
                * (self.equipped.total_stats().damage.strength + 64.into())
                + 320.into())
                / 640.into();

            max_hit = self.equipped.max_hit_callback(max_hit, self, enemy);

            max_hit
        }

        pub fn max_ranged_accuracy_roll(&self, enemy: &Enemy) -> Scalar {
            let mut effective_ranged_level =
                self.levels.ranged * self.prayer_stats().ranged_accuracy;
            effective_ranged_level += self.combat_option.invisible_boost().ranged;
            effective_ranged_level += 8.into();

            let style_bonus = match self.combat_option.style_type {
                StyleType::Ranged => self.equipped.total_stats().attack.ranged,
                _ => unreachable!(),
            };

            let mut attack_roll = effective_ranged_level * (style_bonus + 64.into());

            attack_roll = self
                .equipped
                .accuracy_roll_callback(attack_roll, self, enemy);

            attack_roll
        }

        pub fn max_ranged_hit(&self, enemy: &Enemy) -> Scalar {
            let mut effective_ranged_level = self.levels.ranged * self.prayer_stats().ranged_damage;
            effective_ranged_level += self.combat_option.invisible_boost().ranged;
            effective_ranged_level += 8.into();

            let mut max_hit = (effective_ranged_level
                * (self.equipped.total_stats().damage.ranged + 64.into())
                + 320.into())
                / 640.into();

            max_hit = self.equipped.max_hit_callback(max_hit, self, enemy);

            max_hit
        }

        pub fn max_accuracy_roll(&self, enemy: &Enemy) -> Scalar {
            match self.combat_option.style_type {
                StyleType::Stab | StyleType::Slash | StyleType::Crush => {
                    self.max_melee_accuracy_roll(enemy)
                }
                StyleType::Ranged => self.max_ranged_accuracy_roll(enemy),
                StyleType::Magic => todo!(),
                StyleType::None => todo!(),
            }
        }

        pub fn max_hit(&self, enemy: &Enemy) -> Scalar {
            match self.combat_option.style_type {
                StyleType::Stab | StyleType::Slash | StyleType::Crush => self.max_melee_hit(enemy),
                StyleType::Ranged => self.max_ranged_hit(enemy),
                StyleType::Magic => todo!(),
                StyleType::None => todo!(),
            }
        }

        pub fn dps(&self, enemy: &Enemy) -> f64 {
            let max_enemy_defence_roll: i32 = enemy
                .max_defence_roll(&self.combat_option.style_type)
                .into();
            let max_accuracy_roll: i32 = self.max_accuracy_roll(enemy).into();
            let max_hit: i32 = self.max_hit(enemy).into();
            let attack_speed: i32 = self
                .equipped
                .wielded
                .attack_speed(&self.combat_option)
                .into();

            let max_accuracy_roll: f64 = max_accuracy_roll.into();
            let max_enemy_defence_roll: f64 = max_enemy_defence_roll.into();
            let max_hit: f64 = max_hit.into();
            let attack_speed: f64 = attack_speed.into();

            let hit_rate = if max_enemy_defence_roll > max_accuracy_roll {
                0.5 * max_accuracy_roll / max_enemy_defence_roll
            } else {
                1f64 - (0.5 * max_enemy_defence_roll / max_accuracy_roll)
            };

            ((hit_rate * max_hit / 2.0) / attack_speed) / SECONDS_PER_TICK
        }
    }

    impl Default for Player {
        fn default() -> Self {
            Self {
                levels: Levels::default(),
                equipped: Equipped::default(),
                active_prayers: Vec::default(),
                combat_option: Equipped::default()
                    .wielded
                    .combat_boost()
                    .get(0)
                    .expect("Should contain at least 3 options")
                    .clone(),
                extra: Extra::default(),
            }
        }
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct Levels {
        pub hitpoints: Scalar,
        pub attack: Scalar,
        pub strength: Scalar,
        pub defence: Scalar,
        pub ranged: Scalar,
        pub magic: Scalar,
        pub prayer: Scalar,
    }

    impl Default for Levels {
        fn default() -> Self {
            Self {
                hitpoints: 10.into(),
                attack: 1.into(),
                strength: 1.into(),
                defence: 1.into(),
                ranged: 1.into(),
                magic: 1.into(),
                prayer: 1.into(),
            }
        }
    }

    #[derive(Debug, Default, Clone)]
    pub struct Equipped {
        pub head: Head,
        pub cape: Cape,
        pub neck: Neck,
        pub ammunition: Ammunition,
        pub wielded: Wielded,
        pub body: Body,
        pub legs: Legs,
        pub hands: Hands,
        pub feet: Feet,
        pub ring: Ring,
    }

    impl Equipped {
        pub fn total_stats(&self) -> Stats {
            self.head.stats
                + self.cape.stats
                + self.neck.stats
                + self.ammunition.stats
                + self.wielded.stats()
                + self.body.stats
                + self.legs.stats
                + self.hands.stats
                + self.feet.stats
                + self.ring.stats
        }

        pub fn accuracy_roll_callback(
            &self,
            mut value: Scalar,
            player: &Player,
            enemy: &Enemy,
        ) -> Scalar {
            value = self.head.attributes.iter().fold(value, |value, attribute| {
                (attribute.accuracy_roll_callback())(value, player, enemy)
            });
            value = self.cape.attributes.iter().fold(value, |value, attribute| {
                (attribute.accuracy_roll_callback())(value, player, enemy)
            });
            value = self.neck.attributes.iter().fold(value, |value, attribute| {
                (attribute.accuracy_roll_callback())(value, player, enemy)
            });
            value = self
                .ammunition
                .attributes
                .iter()
                .fold(value, |value, attribute| {
                    (attribute.accuracy_roll_callback())(value, player, enemy)
                });
            value = self
                .wielded
                .attributes()
                .iter()
                .fold(value, |value, attribute| {
                    (attribute.accuracy_roll_callback())(value, player, enemy)
                });
            value = self.body.attributes.iter().fold(value, |value, attribute| {
                (attribute.accuracy_roll_callback())(value, player, enemy)
            });
            value = self.legs.attributes.iter().fold(value, |value, attribute| {
                (attribute.accuracy_roll_callback())(value, player, enemy)
            });
            value = self
                .hands
                .attributes
                .iter()
                .fold(value, |value, attribute| {
                    (attribute.accuracy_roll_callback())(value, player, enemy)
                });
            value = self.feet.attributes.iter().fold(value, |value, attribute| {
                (attribute.accuracy_roll_callback())(value, player, enemy)
            });
            value = self.ring.attributes.iter().fold(value, |value, attribute| {
                (attribute.accuracy_roll_callback())(value, player, enemy)
            });

            value
        }

        pub fn max_hit_callback(
            &self,
            mut value: Scalar,
            player: &Player,
            enemy: &Enemy,
        ) -> Scalar {
            value = self.head.attributes.iter().fold(value, |value, attribute| {
                (attribute.max_hit_callback())(value, player, enemy)
            });
            value = self.cape.attributes.iter().fold(value, |value, attribute| {
                (attribute.max_hit_callback())(value, player, enemy)
            });
            value = self.neck.attributes.iter().fold(value, |value, attribute| {
                (attribute.max_hit_callback())(value, player, enemy)
            });
            value = self
                .ammunition
                .attributes
                .iter()
                .fold(value, |value, attribute| {
                    (attribute.max_hit_callback())(value, player, enemy)
                });
            value = self
                .wielded
                .attributes()
                .iter()
                .fold(value, |value, attribute| {
                    (attribute.max_hit_callback())(value, player, enemy)
                });
            value = self.body.attributes.iter().fold(value, |value, attribute| {
                (attribute.max_hit_callback())(value, player, enemy)
            });
            value = self.legs.attributes.iter().fold(value, |value, attribute| {
                (attribute.max_hit_callback())(value, player, enemy)
            });
            value = self
                .hands
                .attributes
                .iter()
                .fold(value, |value, attribute| {
                    (attribute.max_hit_callback())(value, player, enemy)
                });
            value = self.feet.attributes.iter().fold(value, |value, attribute| {
                (attribute.max_hit_callback())(value, player, enemy)
            });
            value = self.ring.attributes.iter().fold(value, |value, attribute| {
                (attribute.max_hit_callback())(value, player, enemy)
            });

            value
        }
    }
}

/// # Errors
/// Returns an error if the given file cannot be found
pub fn read_file<T>(path: &str) -> Result<HashMap<String, T>, Box<dyn std::error::Error>>
where
    T: NamedData,
{
    let data = std::fs::read_to_string(path)?;
    Ok(serde_json::from_str::<Vec<T>>(&data)?
        .into_iter()
        .map(|x| (x.get_name().to_owned(), x))
        .collect::<HashMap<_, _>>())
}
