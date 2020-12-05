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
            .add_system(entity::spawn_enemy.system())
            .add_system(entity::spawn_explosion.system())
            .add_system(entity::spawn_ship_laser.system())
            .add_system(systems::spawner::trigger_enemy_spawn.system())
            .add_system(systems::input::keyboard_control_ship.system())
            .add_system(systems::input::keyboard_fire_ship_laser.system())
            .add_system(systems::motion::apply_velocity.system())
            .add_system(systems::ship::limit_translation.system())
            .add_system(systems::ship::animation_state_transition.system())
            .add_system(systems::animation::texture_atlas_cycle.system())
            .add_system(systems::collide::enemy_ship_laser_collision_check.system())
            .add_system(systems::collide::enemy_ship_laser_collision_handle.system())
            .add_system(systems::cleanup::despawn_out_of_arena_enemy.system())
            .add_system(systems::cleanup::despawn_expired_time_to_live.system());
    }
}
