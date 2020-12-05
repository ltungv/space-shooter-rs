use crate::{
    components::{HitBox, Ship, ShipAnimationState, Velocity},
    events::SpawnShipLaserEvent,
};
use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
};

/// Change ship's directions based on user's keyboard input
pub fn keyboard_control(
    keyboard_input: Res<Input<KeyCode>>,
    ship: &Ship,
    mut velocity: Mut<Velocity>,
) {
    let mut x_direction = 0.;
    if keyboard_input.pressed(KeyCode::Left) {
        x_direction -= 1.;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        x_direction += 1.;
    }

    let mut y_direction = 0.;
    if keyboard_input.pressed(KeyCode::Up) {
        y_direction += 1.;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        y_direction -= 1.;
    }

    // Ensure ship speed is capped at `max_speed` when moving diagonally
    if x_direction != 0. && y_direction != 0. {
        *velocity.0.y_mut() = (ship.move_speed / f32::sqrt(2.)) * y_direction;
        *velocity.0.x_mut() = (ship.move_speed / f32::sqrt(2.)) * x_direction;
    } else {
        *velocity.0.y_mut() = ship.move_speed * y_direction;
        *velocity.0.x_mut() = ship.move_speed * x_direction;
    }
}

/// Change the ship's animation state and change the current index to the index of the sprite
/// that represents that state. The ship has to be in the new state for at least some set amount
/// of duration before being able to change its state again
pub fn animation_state_transition(
    time: Res<Time>,
    velocity: &Velocity,
    mut ship: Mut<Ship>,
    mut sprite: Mut<TextureAtlasSprite>,
) {
    ship.transition_timer.tick(time.delta_seconds);
    if ship.transition_timer.finished {
        let x_velocity = velocity.0.x();
        let new_animation_state = if x_velocity < 0. {
            match ship.animation_state {
                ShipAnimationState::Stabilized => ShipAnimationState::HalfLeft,
                ShipAnimationState::HalfRight => ShipAnimationState::Stabilized,
                ShipAnimationState::FullRight => ShipAnimationState::HalfRight,
                ShipAnimationState::HalfLeft | ShipAnimationState::FullLeft => {
                    ShipAnimationState::FullLeft
                }
            }
        } else if x_velocity > 0. {
            match ship.animation_state {
                ShipAnimationState::Stabilized => ShipAnimationState::HalfRight,
                ShipAnimationState::HalfLeft => ShipAnimationState::Stabilized,
                ShipAnimationState::FullLeft => ShipAnimationState::HalfLeft,
                ShipAnimationState::HalfRight | ShipAnimationState::FullRight => {
                    ShipAnimationState::FullRight
                }
            }
        } else {
            match ship.animation_state {
                ShipAnimationState::FullLeft => ShipAnimationState::HalfLeft,
                ShipAnimationState::FullRight => ShipAnimationState::HalfRight,
                ShipAnimationState::Stabilized
                | ShipAnimationState::HalfRight
                | ShipAnimationState::HalfLeft => ShipAnimationState::Stabilized,
            }
        };

        // Updates if state is changed
        if new_animation_state != ship.animation_state {
            ship.transition_timer.reset();
            ship.animation_state = new_animation_state;
            sprite.index = match ship.animation_state {
                ShipAnimationState::FullLeft => 0,
                ShipAnimationState::HalfLeft => 1,
                ShipAnimationState::Stabilized => 2,
                ShipAnimationState::HalfRight => 3,
                ShipAnimationState::FullRight => 4,
            };
        }
    }
}

pub fn fire_laser(
    time: Res<Time>,
    mut spawn_ship_laser_events: ResMut<Events<SpawnShipLaserEvent>>,
    transform: &Transform,
    HitBox(hit_box): &HitBox,
    mut ship: Mut<Ship>,
) {
    ship.laser_cooldown_timer.tick(time.delta_seconds);
    if ship.laser_cooldown_timer.finished {
        ship.laser_cooldown_timer.reset();
        let laser_translation = transform.translation + hit_box.x() * Vec3::unit_y();
        spawn_ship_laser_events.send(SpawnShipLaserEvent { laser_translation })
    }
}
