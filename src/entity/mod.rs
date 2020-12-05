mod enemy;
mod enemy_spawner;
mod ship;
mod ship_laser;
mod explosion;

pub use enemy::*;
pub use enemy_spawner::*;
pub use ship::*;
pub use ship_laser::*;
pub use explosion::*;

use crate::constant::ARENA_SCALE;
use bevy::prelude::*;

/// Create a camera
pub fn initialize_camera(mut commands: Commands) {
    commands.spawn(Camera2dComponents {
        transform: Transform::from_scale(Vec3::splat(1. / ARENA_SCALE)),
        ..Default::default()
    });
}
