use crate::entity;
use crate::resource;
use crate::system;
use bevy::prelude::*;

/// A plugin that add and initialize all the entities and systems for running the game
#[derive(Default)]
pub struct Game;

impl Plugin for Game {
    // this is where we set up our plugin
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<resource::GameState>()
            .add_startup_system(resource::initialize_enemy_data.system())
            .add_startup_system(entity::initialize_camera.system())
            .add_startup_system(entity::initialize_ship.system())
            .add_startup_system(entity::initialize_enemies_spawner.system())
            .add_system(system::ship_control.system())
            .add_system(system::ship_movement.system())
            .add_system(system::ship_state_transition.system())
            .add_system(system::enemies_spawner.system())
            .add_system(system::enemies_movement.system())
            .add_system(system::enemies_despawner.system())
            .add_system(system::entities_animation.system());
    }
}

