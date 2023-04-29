use std::collections::HashMap;

use lazy_static::lazy_static;
use osrs_dps_calc::{
    equipment::{Slots, StyleType},
    generics::read_file,
    prayers::Prayer,
    spells::Spell,
    unit::{Enemy, Player},
};

type TResult<T> = Result<T, Box<dyn std::error::Error>>;

lazy_static! {
    static ref ITEMS: HashMap<String, Slots> = read_file("./data/equipment.json").unwrap();
    static ref PRAYERS: HashMap<String, Prayer> = read_file("./data/prayers.json").unwrap();
    static ref SPELLS: HashMap<String, Spell> = read_file("./data/spells.json").unwrap();
    static ref ENEMIES: HashMap<String, Enemy> = read_file("./data/enemies.json").unwrap();
}

struct PlayerConstructor<'a> {
    player: Player<'a>,
}

impl<'a> PlayerConstructor<'a> {
    fn new() -> PlayerConstructor<'a> {
        Self {
            player: Player::default(),
        }
    }

    fn equip(mut self, slot: &str) -> TResult<Self> {
        self.player = self
            .player
            .equip(ITEMS.get(slot).ok_or("Could not find item")?);
        Ok(self)
    }

    fn activate_prayer(mut self, prayer: &str) -> TResult<Self> {
        self.player = self
            .player
            .activate_prayer(PRAYERS.get(prayer).ok_or("Could not find prayer")?);
        Ok(self)
    }

    fn select_spell(mut self, spell: &str) -> TResult<Self> {
        self.player = self
            .player
            .select_spell(SPELLS.get(spell).ok_or("Could not find spell")?);
        Ok(self)
    }

    fn build(self) -> Player<'a> {
        self.player
    }
}

fn create_enemy(enemy: &str) -> TResult<&Enemy> {
    Ok(ENEMIES.get(enemy).ok_or("Could not find enemy")?)
}

fn assert_float_eq(lhs: f64, rhs: f64) {
    assert!(
        (lhs - rhs).abs() < 1e-12,
        "{lhs} did not match expected value {rhs}"
    );
}

#[test]
fn test_standard_melee_accuracy() -> TResult<()> {
    let mut player = PlayerConstructor::new()
        .equip("Abyssal whip")?
        .equip("Dragon defender")?
        .activate_prayer("Piety")?
        .build();
    let enemy = create_enemy("Fire giant (level 86)")?;
    player.change_combat_style(1)?;
    assert_eq!(player.max_accuracy_roll(enemy), 21590.into());
    Ok(())
}

#[test]
fn test_standard_melee_max_hit() -> TResult<()> {
    let mut player = PlayerConstructor::new()
        .equip("Abyssal whip")?
        .equip("Dragon defender")?
        .activate_prayer("Piety")?
        .build();
    let enemy = create_enemy("Fire giant (level 86)")?;
    player.change_combat_style(1)?;
    assert_eq!(player.max_hit(enemy), 31.into());
    Ok(())
}

#[test]
fn test_enemy_slash_defence() -> TResult<()> {
    let enemy = create_enemy("Fire giant (level 86)")?;
    assert_eq!(enemy.max_defence_roll(&StyleType::Slash), 4958.into());
    Ok(())
}

#[test]
fn test_standard_melee_dps_vs_enemy() -> TResult<()> {
    let mut player = PlayerConstructor::new()
        .equip("Abyssal whip")?
        .equip("Dragon defender")?
        .activate_prayer("Piety")?
        .build();
    player.change_combat_style(1)?;
    let enemy = create_enemy("Fire giant (level 86)")?;
    assert_float_eq(player.dps(enemy), 5.716_511_895_388_511_5);
    Ok(())
}

#[test]
fn test_dragon_hunter_crossbow_accuracy() -> TResult<()> {
    let mut player = PlayerConstructor::new()
        .equip("Dragon hunter crossbow")?
        .equip("Dragon bolts")?
        .activate_prayer("Rigour")?
        .build();
    player.change_combat_style(1)?;
    let enemy = create_enemy("Mithril dragon")?;
    assert_eq!(player.max_accuracy_roll(enemy), 26044.into());
    Ok(())
}

#[test]
fn test_dragon_hunter_crossbow_max_hit() -> TResult<()> {
    let mut player = PlayerConstructor::new()
        .equip("Dragon hunter crossbow")?
        .equip("Dragon bolts")?
        .activate_prayer("Rigour")?
        .build();
    player.change_combat_style(1)?;
    let enemy = create_enemy("Mithril dragon")?;
    assert_eq!(player.max_hit(enemy), 46.into());
    Ok(())
}

#[test]
fn test_dragon_hunter_crossbow_dps() -> TResult<()> {
    let mut player = PlayerConstructor::new()
        .equip("Dragon hunter crossbow")?
        .equip("Dragon bolts")?
        .activate_prayer("Rigour")?
        .build();
    player.change_combat_style(1)?;
    let enemy = create_enemy("Mithril dragon")?;
    assert_float_eq(player.dps(enemy), 2.340_311_149_659_705);
    Ok(())
}

#[test]
fn test_colossal_blade_dps() -> TResult<()> {
    let player = PlayerConstructor::new()
        .equip("Colossal blade")?
        .activate_prayer("Piety")?
        .build();
    let enemy = create_enemy("Fire giant (level 86)")?;
    assert_float_eq(player.dps(enemy), 4.529_077_680_484_447);
    Ok(())
}

#[test]
fn test_wind_bolt_dps() -> TResult<()> {
    let player = PlayerConstructor::new().select_spell("Wind Bolt")?.build();
    let enemy = create_enemy("Fire giant (level 86)").unwrap();
    assert_float_eq(player.dps(enemy), 1.430_348_618_544_771);
    Ok(())
}

#[test]
fn test_trident_of_the_swamp() -> TResult<()> {
    let player = PlayerConstructor::new()
        .equip("Trident of the swamp")?
        .activate_prayer("Mystic Might")?
        .build();
    let enemy = create_enemy("Mithril dragon")?;
    assert_float_eq(player.dps(enemy), 2.141_780_355_389_947_5);
    Ok(())
}
