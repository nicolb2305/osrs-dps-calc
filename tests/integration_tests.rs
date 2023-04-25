use osrs_dps_calc::{
    equipment::{self, Slots, Wielded},
    prayers::Prayer,
    read_file,
    unit::{Enemy, Equipped, Levels, Player},
};

fn create_melee_player_standard_gear() -> Result<Player, &'static str> {
    let items = read_file::<Slots>("./data/equipment.json").unwrap();
    let prayers = read_file::<Prayer>("./data/prayers.json").unwrap();

    if let (
        Some(Slots::WeaponOneHanded(abyssal_whip)),
        Some(Slots::Shield(dragon_defender)),
        Some(piety),
    ) = (
        items.get("Abyssal whip"),
        items.get("Dragon defender"),
        prayers.get("Piety"),
    ) {
        let wielded =
            Wielded::equip_one_handed(Some(abyssal_whip.clone()), Some(dragon_defender.clone()));
        let equipped = Equipped {
            wielded,
            ..Default::default()
        };
        let levels = Levels {
            attack: 99.into(),
            strength: 99.into(),
            ..Default::default()
        };
        let player = Player::default()
            .equip(equipped)
            .set_levels(levels)
            .activate_prayer(piety.clone());

        Ok(player)
    } else {
        Err("Could not find 'Abyssal whip', 'Dragon defender' and 'Piety'.")
    }
}

fn create_player_dragon_hunter_crossbow() -> Result<Player, &'static str> {
    let items = read_file::<Slots>("./data/equipment.json").unwrap();
    let prayers = read_file::<Prayer>("./data/prayers.json").unwrap();

    if let (
        Some(Slots::WeaponOneHanded(dragon_hunter_crossbow)),
        Some(Slots::Ammunition(dragon_bolts)),
        Some(rigour),
    ) = (
        items.get("Dragon hunter crossbow"),
        items.get("Dragon bolts"),
        prayers.get("Rigour"),
    ) {
        let wielded = Wielded::equip_one_handed(Some(dragon_hunter_crossbow.clone()), None);
        let equipped = Equipped {
            wielded,
            ammunition: dragon_bolts.clone(),
            ..Default::default()
        };
        let levels = Levels {
            ranged: 99.into(),
            ..Default::default()
        };
        let player = Player::default()
            .equip(equipped)
            .set_levels(levels)
            .activate_prayer(rigour.clone());

        Ok(player)
    } else {
        Err("Could not find 'Dragon hunter crossbow', 'Dragon bolts' and 'Rigour'.")
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
    let mut player = create_melee_player_standard_gear().unwrap();
    let fire_giant = create_fire_giant().unwrap();
    player.change_combat_style(1).unwrap();
    assert_eq!(player.max_accuracy_roll(&fire_giant), 21590.into());
}

#[test]
fn test_standard_melee_max_hit() {
    let mut player = create_melee_player_standard_gear().unwrap();
    let fire_giant = create_fire_giant().unwrap();
    player.change_combat_style(1).unwrap();
    assert_eq!(player.max_hit(&fire_giant), 31.into());
}

#[test]
fn test_enemy_slash_defence() {
    let fire_giant = create_fire_giant().unwrap();
    assert_eq!(
        fire_giant.max_defence_roll(&equipment::StyleType::Slash),
        4958.into()
    );
}

#[test]
fn test_standard_melee_dps_vs_enemy() {
    let mut player = create_melee_player_standard_gear().unwrap();
    player.change_combat_style(1).unwrap();
    let fire_giant = create_fire_giant().unwrap();
    assert!(player.dps(&fire_giant) - 5.716_776_671_298_441 < 1e-6);
}

#[test]
fn test_dragon_hunter_crossbow_accuracy() {
    let mut player = create_player_dragon_hunter_crossbow().unwrap();
    player.change_combat_style(1).unwrap();
    let mithril_dragon = create_mithril_dragon().unwrap();
    assert_eq!(player.max_accuracy_roll(&mithril_dragon), 26044.into());
}

#[test]
fn test_dragon_hunter_crossbow_max_hit() {
    let mut player = create_player_dragon_hunter_crossbow().unwrap();
    player.change_combat_style(1).unwrap();
    let mithril_dragon = create_mithril_dragon().unwrap();
    assert_eq!(player.max_hit(&mithril_dragon), 46.into());
}

#[test]
fn test_dragon_hunter_crossbow_dps() {
    let mut player = create_player_dragon_hunter_crossbow().unwrap();
    player.change_combat_style(1).unwrap();
    let mithril_dragon = create_mithril_dragon().unwrap();
    assert!(player.dps(&mithril_dragon) - 2.340_366_011_846_156_4 < 1e-6);
}
