use crate::{
    component::{Animation, HitBox, Ship, ShipAnimationState, ShipLaser, TimeToLive, Velocity},
    constant::{
        ANIMATION_INTERVAL, ARENA_HEIGHT, ARENA_WIDTH, LASER_SPRITE_WIDTH,
        SHIP_LASER_INITIAL_VELOCITY, SHIP_LASER_SPRITE_HEIGHT, SHIP_LASER_TIME_TO_LIVE_DURATION,
        SPRITE_SCALING_FACTOR,
    },
    resource::GameState,
};
use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
};

/// Change ship's position based on the moving speed and moving direction. Movement is limited
/// to the window viewable area
pub fn ship_translation_clip(_ship: &Ship, hit_box: &HitBox, mut transform: Mut<Transform>) {
    // X-axis movement
    let max_offset_x_from_center = (ARENA_WIDTH - hit_box.width) / 2.;
    *transform.translation.x_mut() = transform
        .translation
        .x()
        .min(max_offset_x_from_center)
        .max(-max_offset_x_from_center);

    // Y-axis movement
    let max_offset_y_from_center = (ARENA_HEIGHT - hit_box.height) / 2.;
    *transform.translation.y_mut() = transform
        .translation
        .y()
        .min(max_offset_y_from_center)
        .max(-max_offset_y_from_center);
}

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

pub fn keyboard_fire_laser(
    mut commands: Commands,
    time: Res<Time>,
    kb_input: Res<Input<KeyCode>>,
    game_state: Res<GameState>,
    transform: &Transform,
    hit_box: &HitBox,
    mut ship: Mut<Ship>,
) {
    ship.laser_cooldown_timer.tick(time.delta_seconds);
    if kb_input.pressed(KeyCode::Space) && ship.laser_cooldown_timer.finished {
        ship.laser_cooldown_timer.reset();
        if let Some(texture_atlas_handle) = game_state.texture_atlas_handles.get("laser-bolts") {
            let translation = transform.translation + hit_box.height * Vec3::unit_y();
            commands
                .spawn(SpriteSheetComponents {
                    texture_atlas: texture_atlas_handle.clone(),
                    transform: Transform {
                        translation,
                        scale: Vec3::splat(SPRITE_SCALING_FACTOR),
                        ..Default::default()
                    },
                    sprite: TextureAtlasSprite::new(1),
                    ..Default::default()
                })
                .with(ShipLaser)
                .with(TimeToLive(Timer::new(
                    SHIP_LASER_TIME_TO_LIVE_DURATION,
                    false,
                )))
                .with(HitBox {
                    width: LASER_SPRITE_WIDTH * SPRITE_SCALING_FACTOR,
                    height: SHIP_LASER_SPRITE_HEIGHT * SPRITE_SCALING_FACTOR,
                })
                .with(Velocity(Vec2::new(
                    SHIP_LASER_INITIAL_VELOCITY.0,
                    SHIP_LASER_INITIAL_VELOCITY.1,
                )))
                .with(Animation {
                    idx_delta: 2,
                    sprite_count: 4,
                    timer: Timer::new(ANIMATION_INTERVAL, true),
                });
        }
    }
}

/// Change the ship's animation state and change the current index to the index of the sprite
/// that represents that state. The ship has to be in the new state for at least some set amount
/// of duration before being able to change its state again
pub fn ship_state_transition(
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
