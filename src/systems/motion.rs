use crate::{
    components::{ConstrainedToArena, HitBox, Velocity},
    constant::{ARENA_HEIGHT, ARENA_WIDTH},
};
use bevy::prelude::*;

// TODO: implement acceleration

/// Change entities' position based on its current velocity
pub fn apply_velocity(
    // Resources
    time: Res<Time>,
    Velocity(velocity): &Velocity,
    mut transform: Mut<Transform>,
) {
    *transform.translation.x_mut() += time.delta_seconds * velocity.x();
    *transform.translation.y_mut() += time.delta_seconds * velocity.y();
}

pub fn constrained_to_arena(
    _constrained_to_arena: &ConstrainedToArena,
    HitBox(hit_box): &HitBox,
    mut transform: Mut<Transform>,
) {
    // X-axis movement
    let max_offset_x_from_center = (ARENA_WIDTH - hit_box.x()) / 2.;
    *transform.translation.x_mut() = transform
        .translation
        .x()
        .min(max_offset_x_from_center)
        .max(-max_offset_x_from_center);

    // Y-axis movement
    let max_offset_y_from_center = (ARENA_HEIGHT - hit_box.y()) / 2.;
    *transform.translation.y_mut() = transform
        .translation
        .y()
        .min(max_offset_y_from_center)
        .max(-max_offset_y_from_center);
}
