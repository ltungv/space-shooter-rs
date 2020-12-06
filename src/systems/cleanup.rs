use crate::{
    components::{Enemy, HitBox, TimeToLive},
    constant::{ARENA_HEIGHT, ARENA_WIDTH},
};
use bevy::prelude::*;

pub fn despawn_expired_time_to_live(
    mut commands: Commands,
    time: Res<Time>,
    entity: Entity,
    mut time_to_live: Mut<TimeToLive>,
) {
    time_to_live.0.tick(time.delta_seconds);
    if time_to_live.0.finished {
        commands.despawn(entity);
    }
}

pub fn despawn_out_of_arena_enemy(
    mut commands: Commands,
    entity: Entity,
    _enemy: &Enemy,
    HitBox(hit_box): &HitBox,
    transform: &Transform,
) {
    if transform.translation.y() + hit_box.y() / 2. <= -ARENA_HEIGHT / 2.
        || transform.translation.x() + hit_box.x() / 2. <= -ARENA_WIDTH / 2.
        || transform.translation.x() - hit_box.x() / 2. >= ARENA_WIDTH / 2.
    {
        commands.despawn_recursive(entity);
    }
}
