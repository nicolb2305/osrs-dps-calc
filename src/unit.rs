use serde::Deserialize;

use crate::{
    equipment::{
        combat_styles::{CombatOption, StyleType},
        weapon_callbacks::Callbacks,
        Ammunition, Body, Cape, ContainsEquipment, Equipment, Feet, Hands, Head, Legs, Neck,
        PoweredStaff, Ring, Slots, Stats, Wielded,
    },
    generics::{NamedData, Scalar, Ticks, Tiles, SECONDS_PER_TICK},
    prayers::Prayer,
    spells::Spell,
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

        let effective_defence_level = if let StyleType::Magic = style_type {
            self.levels.magic
        } else {
            self.levels.defence
        } + 9.into();

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
    pub charge_active: bool,
}

impl Default for Extra {
    fn default() -> Self {
        Self {
            on_slayer_task: true,
            mining_level: 99.into(),
            in_wilderness: true,
            charge_active: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Player<'a> {
    pub levels: Levels,
    equipped: Equipped<'a>,
    pub active_prayers: Vec<&'a Prayer>,
    combat_option: CombatOption,
    pub spell: Option<&'a Spell>,
    pub extra: Extra,
}

impl<'a> Player<'a> {
    #[must_use]
    pub fn equip_full(mut self, equipped: Equipped<'a>) -> Self {
        self.equipped = equipped;
        self.update_combat_option();
        self
    }

    fn update_combat_option(&mut self) {
        // Should not panic as there should always be at least 3 combat options
        self.combat_option = self.equipped.wielded.combat_boost().remove(0);
    }

    #[must_use]
    pub fn set_levels(mut self, levels: Levels) -> Self {
        self.levels = levels;
        self
    }

    #[must_use]
    pub fn activate_prayer(mut self, prayer: &'a Prayer) -> Self {
        self.active_prayers.push(prayer);
        self
    }

    #[must_use]
    pub fn select_spell(mut self, spell: &'a Spell) -> Self {
        self.spell = Some(spell);
        self
    }

    #[must_use]
    pub fn equip(mut self, slot: &'a Slots) -> Self {
        match slot {
            Slots::Head(head) => self.equipped.head = Some(head),
            Slots::Cape(cape) => self.equipped.cape = Some(cape),
            Slots::Neck(neck) => self.equipped.neck = Some(neck),
            Slots::Ammunition(ammunition) => self.equipped.ammunition = Some(ammunition),
            Slots::WeaponOneHanded(weapon_one_handed) => {
                match self.equipped.wielded {
                    Wielded::OneHanded { weapon: _, shield } => {
                        self.equipped.wielded =
                            Wielded::equip_one_handed(Some(weapon_one_handed), shield);
                    }
                    Wielded::TwoHanded { weapon: _ } => {
                        self.equipped.wielded =
                            Wielded::equip_one_handed(Some(weapon_one_handed), None);
                    }
                };
                self.update_combat_option();
            }
            Slots::WeaponTwoHanded(weapon_two_handed) => {
                self.equipped.wielded = Wielded::equip_two_handed(Some(weapon_two_handed));
                self.update_combat_option();
            }
            Slots::Body(body) => self.equipped.body = Some(body),
            Slots::Shield(new_shield) => {
                match self.equipped.wielded {
                    Wielded::OneHanded { weapon, shield: _ } => {
                        self.equipped.wielded = Wielded::equip_one_handed(weapon, Some(new_shield));
                    }
                    Wielded::TwoHanded { weapon: _ } => {
                        self.equipped.wielded = Wielded::equip_one_handed(None, Some(new_shield));
                    }
                };
                self.update_combat_option();
            }
            Slots::Legs(legs) => self.equipped.legs = Some(legs),
            Slots::Hands(hands) => self.equipped.hands = Some(hands),
            Slots::Feet(feet) => self.equipped.feet = Some(feet),
            Slots::Ring(ring) => self.equipped.ring = Some(ring),
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
        let mut combat_options = self.equipped.wielded.combat_boost();
        if index < combat_options.len() {
            self.combat_option = combat_options.remove(index);
            Ok(())
        } else {
            Err("Invalid index")
        }
    }

    pub fn prayer_stats(&self) -> crate::prayers::Stats {
        self.active_prayers
            .iter()
            .fold(crate::prayers::Stats::default(), |acc, p| acc + p.stats)
    }

    pub fn max_melee_accuracy_roll(&self, enemy: &Enemy) -> Scalar {
        let mut effective_attack_level = self.levels.attack * self.prayer_stats().melee_accuracy;
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
        let mut effective_strength_level = self.levels.strength * self.prayer_stats().melee_damage;
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
        let mut effective_ranged_level = self.levels.ranged * self.prayer_stats().ranged_accuracy;
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

    pub fn max_magic_accuracy_roll(&self, enemy: &Enemy) -> Scalar {
        let mut effective_magic_level = self.levels.magic * self.prayer_stats().magic_accuracy;
        effective_magic_level += self.combat_option.invisible_boost().magic;
        effective_magic_level += 8.into();
        if self.spell.is_some() {
            effective_magic_level += 1.into();
        }

        let magic_bonus = self.equipped.total_stats().attack.magic;

        let mut attack_roll = effective_magic_level * (magic_bonus + 64.into());

        attack_roll = self
            .equipped
            .accuracy_roll_callback(attack_roll, self, enemy);

        attack_roll
    }

    pub fn max_magic_hit(&self, _enemy: &Enemy) -> Scalar {
        let mut max_hit = if let Some(max_hit) = self.equipped.powered_staff_max_hit(self) {
            max_hit
        } else if let Some(spell) = &self.spell {
            spell.max_hit
        } else {
            unimplemented!()
        };

        let magic_damage_bonus = self.equipped.total_stats().damage.magic;

        max_hit = max_hit * magic_damage_bonus;

        max_hit
    }

    pub fn max_accuracy_roll(&self, enemy: &Enemy) -> Scalar {
        if let Some(_spell) = &self.spell {
            self.max_magic_accuracy_roll(enemy)
        } else {
            match self.combat_option.style_type {
                StyleType::Stab | StyleType::Slash | StyleType::Crush => {
                    self.max_melee_accuracy_roll(enemy)
                }
                StyleType::Ranged => self.max_ranged_accuracy_roll(enemy),
                StyleType::Magic => self.max_magic_accuracy_roll(enemy),
                StyleType::None => unimplemented!(),
            }
        }
    }

    pub fn max_hit(&self, enemy: &Enemy) -> Scalar {
        if let Some(_spell) = &self.spell {
            self.max_magic_hit(enemy)
        } else {
            match self.combat_option.style_type {
                StyleType::Stab | StyleType::Slash | StyleType::Crush => self.max_melee_hit(enemy),
                StyleType::Ranged => self.max_ranged_hit(enemy),
                StyleType::Magic => self.max_magic_hit(enemy),
                StyleType::None => unimplemented!(),
            }
        }
    }

    pub fn dps(&self, enemy: &Enemy) -> f64 {
        let style_type = if self.spell.is_some() {
            &StyleType::Magic
        } else {
            &self.combat_option.style_type
        };
        let max_enemy_defence_roll: i32 = enemy.max_defence_roll(style_type).into();
        let max_accuracy_roll: i32 = self.max_accuracy_roll(enemy).into();
        let max_hit: i32 = self.max_hit(enemy).into();
        let attack_speed: i32 = if let Some(_spell) = &self.spell {
            self.equipped
                .attack_speed_callback(5.into(), self, enemy)
                .into()
        } else {
            self.equipped
                .wielded
                .attack_speed(&self.combat_option)
                .into()
        };

        let max_accuracy_roll: f64 = max_accuracy_roll.into();
        let max_enemy_defence_roll: f64 = max_enemy_defence_roll.into();
        let max_hit: f64 = max_hit.into();
        let attack_speed: f64 = attack_speed.into();

        let hit_rate = if max_enemy_defence_roll > max_accuracy_roll {
            0.5 * max_accuracy_roll / (max_enemy_defence_roll + 1.0)
        } else {
            1f64 - (0.5 * (max_enemy_defence_roll + 2.0) / (max_accuracy_roll + 1.0))
        };

        ((hit_rate * max_hit / 2.0) / attack_speed) / SECONDS_PER_TICK
    }
}

impl Default for Player<'_> {
    fn default() -> Self {
        Self {
            levels: Levels::default(),
            equipped: Equipped::default(),
            active_prayers: Vec::default(),
            combat_option: Equipped::default().wielded.combat_boost().remove(0),
            spell: None,
            extra: Extra::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
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
            hitpoints: 99.into(),
            attack: 99.into(),
            strength: 99.into(),
            defence: 99.into(),
            ranged: 99.into(),
            magic: 99.into(),
            prayer: 99.into(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Equipped<'a> {
    pub head: Option<&'a Head>,
    pub cape: Option<&'a Cape>,
    pub neck: Option<&'a Neck>,
    pub ammunition: Option<&'a Ammunition>,
    pub wielded: Wielded<'a>,
    pub body: Option<&'a Body>,
    pub legs: Option<&'a Legs>,
    pub hands: Option<&'a Hands>,
    pub feet: Option<&'a Feet>,
    pub ring: Option<&'a Ring>,
}

pub struct EquippedIter<'a> {
    inner: &'a Equipped<'a>,
    index: u8,
}

impl<'a> Iterator for EquippedIter<'a> {
    type Item = &'a Equipment;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = match self.index {
            0 => self.inner.head.unwrap_or_default().inner(),
            1 => self.inner.cape.unwrap_or_default().inner(),
            2 => self.inner.neck.unwrap_or_default().inner(),
            3 => self.inner.ammunition.unwrap_or_default().inner(),
            4 => self.inner.body.unwrap_or_default().inner(),
            5 => self.inner.legs.unwrap_or_default().inner(),
            6 => self.inner.hands.unwrap_or_default().inner(),
            7 => self.inner.feet.unwrap_or_default().inner(),
            8 => self.inner.ring.unwrap_or_default().inner(),
            _ => return None,
        };
        self.index += 1;
        Some(ret)
    }
}

impl Equipped<'_> {
    pub fn iter(&self) -> EquippedIter {
        EquippedIter {
            inner: self,
            index: 0,
        }
    }

    pub fn total_stats(&self) -> Stats {
        let armour_stats: Stats = self.iter().map(|equipment| equipment.stats).sum();
        armour_stats + self.wielded.stats()
    }

    pub fn accuracy_roll_callback(
        &self,
        mut value: Scalar,
        player: &Player,
        enemy: &Enemy,
    ) -> Scalar {
        value = self.iter().fold(value, |value, equipent| {
            equipent
                .attributes
                .accuracy_roll_callback(value, player, enemy)
        });
        value = self
            .wielded
            .attributes()
            .accuracy_roll_callback(value, player, enemy);

        value
    }

    pub fn max_hit_callback(&self, mut value: Scalar, player: &Player, enemy: &Enemy) -> Scalar {
        value = self.iter().fold(value, |value, equipent| {
            equipent.attributes.max_hit_callback(value, player, enemy)
        });
        value = self
            .wielded
            .attributes()
            .max_hit_callback(value, player, enemy);

        value
    }

    pub fn attack_speed_callback(&self, value: Ticks, player: &Player, enemy: &Enemy) -> Ticks {
        self.wielded
            .attributes()
            .iter()
            .fold(value, |value, attribute| {
                (attribute.attack_speed_callback())(value, player, enemy)
            })
    }

    pub fn powered_staff_max_hit(&self, player: &Player) -> Option<Scalar> {
        fn standard_formula(player: &Player, i: i32, j: i32) -> Scalar {
            player.levels.magic / Scalar::new(i) - Scalar::new(j)
        }

        match self.wielded {
            Wielded::OneHanded { weapon, shield: _ } => {
                weapon.unwrap_or_default().powered_staff_type
            }
            Wielded::TwoHanded { weapon } => weapon.unwrap_or_default().powered_staff_type,
        }
        .map(|powered_staff| match powered_staff {
            PoweredStaff::StarterStaff => 8.into(),
            PoweredStaff::TridentOfTheSeas => standard_formula(player, 3, 5),
            PoweredStaff::ThammaronsSceptre => standard_formula(player, 3, 8),
            PoweredStaff::AccursedSceptre => standard_formula(player, 3, 6),
            PoweredStaff::TridentOfTheSwamp => standard_formula(player, 3, 2),
            PoweredStaff::SanguinestiStaff => standard_formula(player, 3, 1),
            PoweredStaff::Dawnbringer => standard_formula(player, 6, 1),
            PoweredStaff::TumekensShadow => standard_formula(player, 3, -1),
            PoweredStaff::CrystalStaffBasic => 25.into(),
            PoweredStaff::CrystalStaffAttuned => 31.into(),
            PoweredStaff::CrystallStaffPerfected => 39.into(),
            PoweredStaff::SwampLizard => {
                ((player.levels.magic * Scalar::new(56 + 64)) + Scalar::new(320)) / Scalar::new(640)
            }
            PoweredStaff::OrangeSalamander => {
                ((player.levels.magic * Scalar::new(59 + 64)) + Scalar::new(320)) / Scalar::new(640)
            }
            PoweredStaff::RedSalamander => {
                ((player.levels.magic * Scalar::new(77 + 64)) + Scalar::new(320)) / Scalar::new(640)
            }
            PoweredStaff::BlackSalamander => {
                ((player.levels.magic * Scalar::new(92 + 64)) + Scalar::new(320)) / Scalar::new(640)
            }
        })
    }
}
