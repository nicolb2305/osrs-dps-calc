use serde::Deserialize;

use crate::{
    equipment::{
        Ammunition, Body, Cape, CombatOption, Feet, Hands, Head, Legs, Neck, Ring, Shield, Slots,
        Stats, StyleType, WeaponOneHanded, Wielded,
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

    pub fn max_hit_callback(&self, mut value: Scalar, player: &Player, enemy: &Enemy) -> Scalar {
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