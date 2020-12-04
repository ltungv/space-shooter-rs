mod enemy;
mod enemy_spawner;
mod ship;
mod ship_laser;

pub use enemy::*;
pub use enemy_spawner::*;
pub use ship::*;
pub use ship_laser::*;

use bevy::prelude::*;

/// Create a camera
pub fn initialize_camera(mut commands: Commands) {
    commands.spawn(Camera2dComponents::default());
}
