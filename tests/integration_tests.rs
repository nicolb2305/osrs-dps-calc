use std::collections::HashMap;

use osrs_dps_calc::{
    equipment::{Slots, StyleType},
    generics::read_file,
    prayers::Prayer,
    spells::Spell,
    unit::{Enemy, Levels, Player},
};

type TResult<T> = Result<T, Box<dyn std::error::Error>>;

struct PlayerConstructor {
    items: HashMap<String, Slots>,
    prayers: HashMap<String, Prayer>,
    spells: HashMap<String, Spell>,
    player: Player,
}

impl PlayerConstructor {
    fn new() -> TResult<Self> {
        let items = read_file::<Slots>("./data/equipment.json")?;
        let prayers = read_file::<Prayer>("./data/prayers.json")?;
        let spells = read_file::<Spell>("./data/spells.json")?;
        let player = Player::default().set_levels(Levels {
            hitpoints: 99.into(),
            attack: 99.into(),
            strength: 99.into(),
            defence: 99.into(),
            ranged: 99.into(),
            magic: 99.into(),
            prayer: 99.into(),
        });

        Ok(Self {
            items,
            prayers,
            spells,
            player,
        })
    }

    fn equip(mut self, slot: &str) -> TResult<Self> {
        self.player = self
            .player
            .equip(self.items.get(slot).ok_or("Could not find item")?.clone());
        Ok(self)
    }

    fn activate_prayer(mut self, prayer: &str) -> TResult<Self> {
        self.player = self.player.activate_prayer(
            self.prayers
                .get(prayer)
                .ok_or("Could not find prayer")?
                .clone(),
        );
        Ok(self)
    }

    fn select_spell(mut self, spell: &str) -> TResult<Self> {
        self.player = self.player.select_spell(
            self.spells
                .get(spell)
                .ok_or("Could not find spell")?
                .clone(),
        );
        Ok(self)
    }

    fn build(self) -> Player {
        self.player
    }
}

fn create_enemy(enemy: &str) -> TResult<Enemy> {
    let enemies = read_file::<Enemy>("./data/enemies.json")?;
    Ok(enemies.get(enemy).ok_or("Could not find enemy")?.clone())
}

#[test]
fn test_standard_melee_accuracy() -> TResult<()> {
    let mut player = PlayerConstructor::new()?
        .equip("Abyssal whip")?
        .equip("Dragon defender")?
        .activate_prayer("Piety")?
        .build();
    let enemy = create_enemy("Fire giant (level 86)")?;
    player.change_combat_style(1)?;
    assert_eq!(player.max_accuracy_roll(&enemy), 21590.into());
    Ok(())
}

#[test]
fn test_standard_melee_max_hit() -> TResult<()> {
    let mut player = PlayerConstructor::new()?
        .equip("Abyssal whip")?
        .equip("Dragon defender")?
        .activate_prayer("Piety")?
        .build();
    let enemy = create_enemy("Fire giant (level 86)")?;
    player.change_combat_style(1)?;
    assert_eq!(player.max_hit(&enemy), 31.into());
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
    let mut player = PlayerConstructor::new()?
        .equip("Abyssal whip")?
        .equip("Dragon defender")?
        .activate_prayer("Piety")?
        .build();
    player.change_combat_style(1)?;
    let enemy = create_enemy("Fire giant (level 86)")?;
    assert!((player.dps(&enemy) - 5.716_511_895_388_511_5).abs() < 1e-12);
    Ok(())
}

#[test]
fn test_dragon_hunter_crossbow_accuracy() -> TResult<()> {
    let mut player = PlayerConstructor::new()?
        .equip("Dragon hunter crossbow")?
        .equip("Dragon bolts")?
        .activate_prayer("Rigour")?
        .build();
    player.change_combat_style(1)?;
    let enemy = create_enemy("Mithril dragon")?;
    assert_eq!(player.max_accuracy_roll(&enemy), 26044.into());
    Ok(())
}

#[test]
fn test_dragon_hunter_crossbow_max_hit() -> TResult<()> {
    let mut player = PlayerConstructor::new()?
        .equip("Dragon hunter crossbow")?
        .equip("Dragon bolts")?
        .activate_prayer("Rigour")?
        .build();
    player.change_combat_style(1)?;
    let enemy = create_enemy("Mithril dragon")?;
    assert_eq!(player.max_hit(&enemy), 46.into());
    Ok(())
}

#[test]
fn test_dragon_hunter_crossbow_dps() -> TResult<()> {
    let mut player = PlayerConstructor::new()?
        .equip("Dragon hunter crossbow")?
        .equip("Dragon bolts")?
        .activate_prayer("Rigour")?
        .build();
    player.change_combat_style(1)?;
    let enemy = create_enemy("Mithril dragon")?;
    assert!((player.dps(&enemy) - 2.340_311_149_659_705).abs() < 1e-12);
    Ok(())
}

#[test]
fn test_colossal_blade_dps() -> TResult<()> {
    let player = PlayerConstructor::new()?
        .equip("Colossal blade")?
        .activate_prayer("Piety")?
        .build();
    let enemy = create_enemy("Fire giant (level 86)")?;
    assert!((player.dps(&enemy) - 4.529_077_680_484_447).abs() < 1e-12);
    Ok(())
}

#[test]
fn test_wind_bolt_dps() -> TResult<()> {
    let player = PlayerConstructor::new()?.select_spell("Wind Bolt")?.build();
    let enemy = create_enemy("Fire giant (level 86)").unwrap();
    assert!((player.dps(&enemy) - 1.430_348_618_544_771).abs() < 1e-12);
    Ok(())
}

#[test]
fn test_trident_of_the_swamp() -> TResult<()> {
    let player = PlayerConstructor::new()?
        .equip("Trident of the swamp")?
        .activate_prayer("Mystic Might")?
        .build();
    let enemy = create_enemy("Mithril dragon")?;
    assert!((player.dps(&enemy) - 2.141_780_355_389_947_5).abs() < 1e-12);
    Ok(())
}
