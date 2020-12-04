use crate::{
    components::{HitBox, Ship, ShipAnimationState, Velocity},
    constant::{ARENA_HEIGHT, ARENA_WIDTH},
};
use bevy::prelude::*;

/// Change ship's position based on the moving speed and moving direction. Movement is limited
/// to the window viewable area
pub fn limit_ship_translation(
    _ship: &Ship,
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

/// Change the ship's animation state and change the current index to the index of the sprite
/// that represents that state. The ship has to be in the new state for at least some set amount
/// of duration before being able to change its state again
pub fn ship_animation_state_transition(
    time: Res<Time>,
    velocity: &Velocity,
    mut ship: Mut<Ship>,
    mut ship_animation_state: Mut<ShipAnimationState>,
    mut sprite: Mut<TextureAtlasSprite>,
) {
    ship.transition_timer.tick(time.delta_seconds);
    if ship.transition_timer.finished {
        let x_velocity = velocity.0.x();
        let new_animation_state = if x_velocity < 0. {
            match *ship_animation_state {
                ShipAnimationState::Stabilized => ShipAnimationState::HalfLeft,
                ShipAnimationState::HalfRight => ShipAnimationState::Stabilized,
                ShipAnimationState::FullRight => ShipAnimationState::HalfRight,
                ShipAnimationState::HalfLeft | ShipAnimationState::FullLeft => {
                    ShipAnimationState::FullLeft
                }
            }
        } else if x_velocity > 0. {
            match *ship_animation_state {
                ShipAnimationState::Stabilized => ShipAnimationState::HalfRight,
                ShipAnimationState::HalfLeft => ShipAnimationState::Stabilized,
                ShipAnimationState::FullLeft => ShipAnimationState::HalfLeft,
                ShipAnimationState::HalfRight | ShipAnimationState::FullRight => {
                    ShipAnimationState::FullRight
                }
            }
        } else {
            match *ship_animation_state {
                ShipAnimationState::FullLeft => ShipAnimationState::HalfLeft,
                ShipAnimationState::FullRight => ShipAnimationState::HalfRight,
                ShipAnimationState::Stabilized
                | ShipAnimationState::HalfRight
                | ShipAnimationState::HalfLeft => ShipAnimationState::Stabilized,
            }
        };

        // Updates if state is changed
        if new_animation_state != *ship_animation_state {
            ship.transition_timer.reset();
            *ship_animation_state = new_animation_state;
            sprite.index = match *ship_animation_state {
                ShipAnimationState::FullLeft => 0,
                ShipAnimationState::HalfLeft => 1,
                ShipAnimationState::Stabilized => 2,
                ShipAnimationState::HalfRight => 3,
                ShipAnimationState::FullRight => 4,
            };
        }
    }
}
