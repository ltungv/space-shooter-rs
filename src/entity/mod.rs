mod enemy;
mod enemy_spawner;
mod explosion;
mod laser;
mod ship;
mod weapon;

pub use enemy::*;
pub use weapon::*;
pub use enemy_spawner::*;
pub use explosion::*;
pub use laser::*;
pub use ship::*;

use crate::constant::ARENA_SCALE;
use bevy::prelude::*;

/// Create a camera
pub fn initialize_camera(mut commands: Commands) {
    commands.spawn(Camera2dComponents {
        transform: Transform::from_scale(Vec3::splat(1. / ARENA_SCALE)),
        ..Default::default()
    });
}
