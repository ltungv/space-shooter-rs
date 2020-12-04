use crate::{entity, event, resource, systems};
use bevy::prelude::*;

/// A plugin that add and initialize all the entities and systems for running the game
#[derive(Default)]
pub struct Game;

impl Plugin for Game {
    // this is where we set up our plugin
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<event::EntityDespawnEvent>()
            .add_resource(resource::GameState::default())
            .add_startup_system(resource::initialize_texture_atlases.system())
            .add_startup_system(entity::initialize_camera.system())
            .add_startup_system(entity::initialize_ship.system())
            .add_startup_system(entity::initialize_enemy_spawner.system())
            .add_system(systems::spawn::enemy_spawner_trigger.system())
            .add_system(systems::input::keyboard_control_ship.system())
            .add_system(systems::input::keyboard_fire_ship_laser.system())
            .add_system(systems::ship::ship_animation_state_transition.system())
            .add_system(systems::ship::limit_ship_translation.system())
            .add_system(systems::motion::apply_velocity_to_translation.system())
            .add_system(systems::collide::laser_collides_enemy.system())
            .add_system(systems::animation::texture_atlas_cycle_animation.system())
            .add_system(systems::despawn::despawn_enemy_out_of_bound.system())
            .add_system(systems::despawn::despawn_expired_time_to_live.system())
            .add_system(systems::despawn::despawn_events_listener.system());
    }
}
