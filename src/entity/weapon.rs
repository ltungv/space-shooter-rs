use crate::components::Weapon;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct WeaponComponents {
    pub weapon: Weapon,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}
