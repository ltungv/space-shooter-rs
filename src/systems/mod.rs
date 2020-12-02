mod enemies;
mod ship;

use crate::component::{Animation, TimeToLive, Velocity};
use bevy::prelude::*;

pub use enemies::{enemies_despawner, enemies_spawner};
pub use ship::{
    keyboard_control_ship, keyboard_fire_laser, ship_state_transition, ship_translation_clip,
};

// TODO: implement acceleration

/// Change ship's position based on the moving speed and moving direction. Movement is limited
/// to the window viewable area
pub fn entities_movement(
    // Resources
    time: Res<Time>,
    velocity: &Velocity,
    mut transform: Mut<Transform>,
) {
    *transform.translation.x_mut() += time.delta_seconds * velocity.0.x();
    *transform.translation.y_mut() += time.delta_seconds * velocity.0.y();
}

pub fn entities_time_to_live(
    mut commands: Commands,
    time: Res<Time>,
    entity: Entity,
    mut ttl: Mut<TimeToLive>,
) {
    ttl.0.tick(time.delta_seconds);
    if ttl.0.finished {
        commands.despawn(entity);
    }
}

/// Periodically change the index to the sprite in the spritesheet
pub fn entities_animation(
    time: Res<Time>,
    mut sprite: Mut<TextureAtlasSprite>,
    mut animation: Mut<Animation>,
) {
    animation.timer.tick(time.delta_seconds);
    if animation.timer.finished {
        sprite.index = (sprite.index + animation.idx_delta) % animation.sprite_count;
    }
}
