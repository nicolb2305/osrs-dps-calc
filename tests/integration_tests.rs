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
        let mut player = Player::default();
        player.levels = Levels {
            attack: 99.into(),
            strength: 99.into(),
            ..Default::default()
        };
        player.equip(equipped);
        player.change_combat_style(1).unwrap();

        player.active_prayers = vec![piety.clone()];

        Ok(player)
    } else {
        Err("Could not find 'Abyssal whip', 'Dragon defender' and 'Piety'.")
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

#[test]
fn test_standard_melee_accuracy() {
    let player = create_melee_player_standard_gear().unwrap();
    assert_eq!(player.max_accuracy_roll(), 21590.into());
}

#[test]
fn test_standard_melee_max_hit() {
    let player = create_melee_player_standard_gear().unwrap();
    assert_eq!(player.max_hit(), 31.into());
}

#[test]
fn test_enemy_defence() {
    let fire_giant = create_fire_giant().unwrap();
    assert_eq!(
        fire_giant.max_defence_roll(&equipment::StyleType::Slash),
        4958.into()
    );
}

#[test]
fn test_standard_melee_dps_vs_enemy() {
    let player = create_melee_player_standard_gear().unwrap();
    let fire_giant = create_fire_giant().unwrap();
    assert!(player.dps(&fire_giant) - 5.716_776_671_298_441 < 1e-6);
}
