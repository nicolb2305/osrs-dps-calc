use std::collections::HashMap;

use osrs_dps_calc::{
    equipment::{Slots, StyleType},
    generics::read_file,
    prayers::Prayer,
    spells::Spell,
    unit::{Enemy, Levels, Player},
};

pub struct PlayerConstructor {
    items: HashMap<String, Slots>,
    prayers: HashMap<String, Prayer>,
    spells: HashMap<String, Spell>,
    player: Player,
}

impl PlayerConstructor {
    fn new() -> Self {
        let items = read_file::<Slots>("./data/equipment.json").unwrap();
        let prayers = read_file::<Prayer>("./data/prayers.json").unwrap();
        let spells = read_file::<Spell>("./data/spells.json").unwrap();
        let player = Player::default().set_levels(Levels {
            hitpoints: 99.into(),
            attack: 99.into(),
            strength: 99.into(),
            defence: 99.into(),
            ranged: 99.into(),
            magic: 99.into(),
            prayer: 99.into(),
        });

        Self {
            items,
            prayers,
            spells,
            player,
        }
    }

    fn equip(mut self, slot: &str) -> Self {
        self.player = self.player.equip(self.items.get(slot).unwrap().clone());
        self
    }

    fn activate_prayer(mut self, prayer: &str) -> Self {
        self.player = self
            .player
            .activate_prayer(self.prayers.get(prayer).unwrap().clone());
        self
    }

    fn select_spell(mut self, spell: &str) -> Self {
        self.player = self
            .player
            .select_spell(self.spells.get(spell).unwrap().clone());
        self
    }

    fn build(self) -> Player {
        self.player
    }
}

fn create_fire_giant() -> Result<Enemy, &'static str> {
    let enemies = read_file::<Enemy>("./data/enemies.json").unwrap();
    if let Some(fire_giant) = enemies.get("Fire giant (level 86)") {
        Ok(fire_giant.clone())
    } else {
        Err("Could not find 'Fire giant (level 86)'")
    }
}

fn create_mithril_dragon() -> Result<Enemy, &'static str> {
    let enemies = read_file::<Enemy>("./data/enemies.json").unwrap();
    if let Some(mithril_dragon) = enemies.get("Mithril dragon") {
        Ok(mithril_dragon.clone())
    } else {
        Err("Could not find 'Mithril dragon'")
    }
}

#[test]
fn test_standard_melee_accuracy() {
    let mut player = PlayerConstructor::new()
        .equip("Abyssal whip")
        .equip("Dragon defender")
        .activate_prayer("Piety")
        .build();
    let enemy = create_fire_giant().unwrap();
    player.change_combat_style(1).unwrap();
    assert_eq!(player.max_accuracy_roll(&enemy), 21590.into());
}

#[test]
fn test_standard_melee_max_hit() {
    let mut player = PlayerConstructor::new()
        .equip("Abyssal whip")
        .equip("Dragon defender")
        .activate_prayer("Piety")
        .build();
    let enemy = create_fire_giant().unwrap();
    player.change_combat_style(1).unwrap();
    assert_eq!(player.max_hit(&enemy), 31.into());
}

#[test]
fn test_enemy_slash_defence() {
    let enemy = create_fire_giant().unwrap();
    assert_eq!(enemy.max_defence_roll(&StyleType::Slash), 4958.into());
}

#[test]
fn test_standard_melee_dps_vs_enemy() {
    let mut player = PlayerConstructor::new()
        .equip("Abyssal whip")
        .equip("Dragon defender")
        .activate_prayer("Piety")
        .build();
    player.change_combat_style(1).unwrap();
    let enemy = create_fire_giant().unwrap();
    assert!((player.dps(&enemy) - 5.716_511_895_388_511_5).abs() < 1e-12);
}

#[test]
fn test_dragon_hunter_crossbow_accuracy() {
    let mut player = PlayerConstructor::new()
        .equip("Dragon hunter crossbow")
        .equip("Dragon bolts")
        .activate_prayer("Rigour")
        .build();
    player.change_combat_style(1).unwrap();
    let enemy = create_mithril_dragon().unwrap();
    assert_eq!(player.max_accuracy_roll(&enemy), 26044.into());
}

#[test]
fn test_dragon_hunter_crossbow_max_hit() {
    let mut player = PlayerConstructor::new()
        .equip("Dragon hunter crossbow")
        .equip("Dragon bolts")
        .activate_prayer("Rigour")
        .build();
    player.change_combat_style(1).unwrap();
    let enemy = create_mithril_dragon().unwrap();
    assert_eq!(player.max_hit(&enemy), 46.into());
}

#[test]
fn test_dragon_hunter_crossbow_dps() {
    let mut player = PlayerConstructor::new()
        .equip("Dragon hunter crossbow")
        .equip("Dragon bolts")
        .activate_prayer("Rigour")
        .build();
    player.change_combat_style(1).unwrap();
    let enemy = create_mithril_dragon().unwrap();
    assert!((player.dps(&enemy) - 2.340_311_149_659_705).abs() < 1e-12);
}

#[test]
fn test_colossal_blade_dps() {
    let player = PlayerConstructor::new()
        .equip("Colossal blade")
        .activate_prayer("Piety")
        .build();
    let enemy = create_fire_giant().unwrap();
    assert!((player.dps(&enemy) - 4.529_077_680_484_447).abs() < 1e-12);
}

#[test]
fn test_wind_bolt_dps() {
    let player = PlayerConstructor::new().select_spell("Wind Bolt").build();
    let enemy = create_fire_giant().unwrap();
    assert!((player.dps(&enemy) - 1.430_348_618_544_771).abs() < 1e-12);
}

#[test]
fn test_trident_of_the_swamp() {
    let player = PlayerConstructor::new()
        .equip("Trident of the swamp")
        .activate_prayer("Mystic Might")
        .build();
    let enemy = create_mithril_dragon().unwrap();
    assert!((player.dps(&enemy) - 2.141_780_355_389_947_5).abs() < 1e-12);
}
