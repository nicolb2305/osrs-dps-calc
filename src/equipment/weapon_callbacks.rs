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
        StyleType::Stab | StyleType::Slash | StyleType::Crush if enemy.has_attribute(&Undead) => {
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
