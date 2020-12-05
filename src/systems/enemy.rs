use crate::{
    components::{Enemy, HitBox},
    constant::{ARENA_HEIGHT, ARENA_WIDTH},
};
use bevy::prelude::*;

pub fn despawn_out_of_bound(
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
        commands.despawn(entity);
    }
}
