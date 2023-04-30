use lazy_static::lazy_static;

use crate::equipment::Equipment;

use super::{
    Ammunition, Body, Cape, Feet, Hands, Head, Legs, Neck, Ring, Shield, WeaponOneHanded,
    WeaponTwoHanded,
};

lazy_static! {
    static ref DEFAULT_ITEM_EQUIPMENT: Equipment = Equipment {
        name: "Empty".to_string(),
        ..Default::default()
    };
}

macro_rules! create_default_items {
    ($(($item:tt, $item_name:ident)),*) => {
        $(
            lazy_static! {
                static ref $item_name: $item = $item {
                    inner: DEFAULT_ITEM_EQUIPMENT.clone(),
                    ..Default::default()
                };
            }

            impl Default for &$item {
                fn default() -> Self {
                    &$item_name
                }
            }
        )*
    };
}

create_default_items!(
    (WeaponOneHanded, DEFAULT_ITEM_WEAPON_ONE_HANDED),
    (WeaponTwoHanded, DEFAULT_ITEM_WEAPON_TWO_HANDED),
    (Shield, DEFAULT_ITEM_SHIELD),
    (Head, DEFAULT_ITEM_HEAD),
    (Cape, DEFAULT_ITEM_CAPE),
    (Neck, DEFAULT_ITEM_NECK),
    (Ammunition, DEFAULT_ITEM_AMMUNITION),
    (Body, DEFAULT_ITEM_BODY),
    (Legs, DEFAULT_ITEM_LEGS),
    (Hands, DEFAULT_ITEM_HANDS),
    (Feet, DEFAULT_ITEM_FEET),
    (Ring, DEFAULT_ITEM_RING)
);
