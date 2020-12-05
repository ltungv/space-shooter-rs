use crate::{entity, events, resource, systems};
use bevy::prelude::*;

/// A plugin that add and initialize all the entities and systems for running the game
#[derive(Default)]
pub struct Game;

impl Plugin for Game {
    // this is where we set up our plugin
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<resource::TextureAtlasHandles>()
            .init_resource::<resource::EventReaders>()
            .add_event::<events::EnemySpawnEvent>()
            .add_event::<events::ShipLaserSpawnEvent>()
            .add_event::<events::ExplosionSpawnEvent>()
            .add_event::<events::EnemyShipLaserCollisionEvent>()
            .add_startup_system(entity::initialize_camera.system())
            .add_startup_system(entity::initialize_ship.system())
            .add_startup_system(entity::initialize_enemy_spawner.system())
            .add_system(entity::enemy_spawn_event_listener.system())
            .add_system(entity::explosion_spawn_event_listener.system())
            .add_system(entity::ship_laser_spawn_event_listener.system())
            .add_system(systems::spawner::trigger_enemy_spawn.system())
            .add_system(systems::input::keyboard_control_ship.system())
            .add_system(systems::input::keyboard_fire_ship_laser.system())
            .add_system(systems::motion::apply_velocity_to_translation.system())
            .add_system(systems::ship::limit_translation.system())
            .add_system(systems::ship::animation_state_transition.system())
            .add_system(systems::animation::texture_atlas_cycle.system())
            .add_system(systems::collide::enemy_with_laser.system())
            .add_system(systems::collide::enemy_ship_laser_collision_event_listener.system())
            .add_system(systems::enemy::despawn_out_of_bound.system())
            .add_system(systems::time_to_live::despawn_expired.system());
    }
}
