use crate::{
    components::{HitBox, Ship, Velocity},
    entity,
    resource::GameState,
};
use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
};

/// Change ship's directions based on user's keyboard input
pub fn keyboard_control_ship(
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

pub fn keyboard_fire_ship_laser(
    commands: Commands,
    time: Res<Time>,
    kb_input: Res<Input<KeyCode>>,
    game_state: Res<GameState>,
    transform: &Transform,
    HitBox(hit_box): &HitBox,
    mut ship: Mut<Ship>,
) {
    ship.laser_cooldown_timer.tick(time.delta_seconds);
    if kb_input.pressed(KeyCode::Space) && ship.laser_cooldown_timer.finished {
        ship.laser_cooldown_timer.reset();
        let laser_translation = transform.translation + hit_box.x() * Vec3::unit_y();
        entity::spawn_ship_laser(commands, game_state, laser_translation);
    }
}
