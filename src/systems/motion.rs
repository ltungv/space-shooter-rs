use crate::components::Velocity;
use bevy::prelude::*;

// TODO: implement acceleration

/// Change entities' position based on its current velocity
pub fn apply_velocity_to_translation(
    // Resources
    time: Res<Time>,
    Velocity(velocity): &Velocity,
    mut transform: Mut<Transform>,
) {
    *transform.translation.x_mut() += time.delta_seconds * velocity.x();
    *transform.translation.y_mut() += time.delta_seconds * velocity.y();
}
