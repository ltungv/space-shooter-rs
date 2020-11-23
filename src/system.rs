use crate::{
    component::{Animatable, Enemy, HitBox, Motion, Player, PlayerAnimationState},
    constant::{ARENA_HEIGHT, ARENA_WIDTH},
};
use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::{Commands, Entity, Mut, Res, TextureAtlasSprite, Time, Transform},
};

/// Change player's position based on the moving speed and moving direction. Movement is limited
/// to the window viewable area
#[allow(clippy::too_many_arguments)]
pub fn enemies_movement(
    // Resources
    time: Res<Time>,
    _enemy: &Enemy,
    motion: &Motion,
    mut transform: Mut<Transform>,
) {
    *transform.translation.x_mut() += time.delta_seconds * motion.velocity.x();
    *transform.translation.y_mut() += time.delta_seconds * motion.velocity.y();
}

pub fn enemies_despawner(
    mut commands: Commands,
    entity: Entity,
    _enemy: &Enemy,
    hit_box: &HitBox,
    transform: Mut<Transform>,
) {
    if transform.translation.y() + hit_box.height / 2. <= -ARENA_HEIGHT / 2. {
        commands.despawn(entity);
    }
}

/// Change player's position based on the moving speed and moving direction. Movement is limited
/// to the window viewable area
#[allow(clippy::too_many_arguments)]
pub fn player_movement(
    // Resources
    time: Res<Time>,
    _player: &Player,
    motion: &Motion,
    hit_box: &HitBox,
    mut transform: Mut<Transform>,
) {
    // X-axis movement
    let player_width_offset = (ARENA_WIDTH - hit_box.width) / 2.;
    *transform.translation.x_mut() += time.delta_seconds * motion.velocity.x();
    *transform.translation.x_mut() = transform
        .translation
        .x()
        // update bound
        .min(player_width_offset)
        // lower bound
        .max(-player_width_offset);

    // Y-axis movement
    let player_height_offset = (ARENA_HEIGHT - hit_box.height) / 2.;
    *transform.translation.y_mut() += time.delta_seconds * motion.velocity.y();
    *transform.translation.y_mut() = transform
        .translation
        .y()
        // upper bound
        .min(player_height_offset)
        // lower bound
        .max(-player_height_offset);
}

/// Change player's directions based on user's keyboard input
pub fn player_control(kb_input: Res<Input<KeyCode>>, _player: &Player, mut motion: Mut<Motion>) {
    let mut x_direction = 0.;
    if kb_input.pressed(KeyCode::Left) {
        x_direction -= 1.;
    }
    if kb_input.pressed(KeyCode::Right) {
        x_direction += 1.;
    }

    let mut y_direction = 0.;
    if kb_input.pressed(KeyCode::Up) {
        y_direction += 1.;
    }
    if kb_input.pressed(KeyCode::Down) {
        y_direction -= 1.;
    }

    // Ensure player speed is capped at `max_speed` when moving diagonally
    if x_direction != 0. && y_direction != 0. {
        *motion.velocity.y_mut() = (motion.max_speed / f32::sqrt(2.)) * y_direction;
        *motion.velocity.x_mut() = (motion.max_speed / f32::sqrt(2.)) * x_direction;
    } else {
        *motion.velocity.y_mut() = motion.max_speed * y_direction;
        *motion.velocity.x_mut() = motion.max_speed * x_direction;
    }
}

/// Change the player's animation state and change the current index to the index of the sprite
/// that represents that state. The player has to be in the new state for at least some set amount
/// of duration before being able to change its state again
pub fn player_state_transition(
    time: Res<Time>,
    mut player: Mut<Player>,
    motion: &Motion,
    mut sprite: Mut<TextureAtlasSprite>,
) {
    // State is not changed rapidly so that animation can be perceived by the player
    if let Some(now) = time.instant {
        if now.duration_since(player.transition_instant) >= player.transition_duration {
            // Determines the new state based on previous state and current moving direction
            let x_velocity = motion.velocity.x();
            let new_animation_state = if x_velocity < 0. {
                match player.animation_state {
                    PlayerAnimationState::Stabilized => PlayerAnimationState::HalfLeft,
                    PlayerAnimationState::HalfRight => PlayerAnimationState::Stabilized,
                    PlayerAnimationState::FullRight => PlayerAnimationState::HalfRight,
                    PlayerAnimationState::HalfLeft | PlayerAnimationState::FullLeft => {
                        PlayerAnimationState::FullLeft
                    }
                }
            } else if x_velocity > 0. {
                match player.animation_state {
                    PlayerAnimationState::Stabilized => PlayerAnimationState::HalfRight,
                    PlayerAnimationState::HalfLeft => PlayerAnimationState::Stabilized,
                    PlayerAnimationState::FullLeft => PlayerAnimationState::HalfLeft,
                    PlayerAnimationState::HalfRight | PlayerAnimationState::FullRight => {
                        PlayerAnimationState::FullRight
                    }
                }
            } else {
                match player.animation_state {
                    PlayerAnimationState::FullLeft => PlayerAnimationState::HalfLeft,
                    PlayerAnimationState::FullRight => PlayerAnimationState::HalfRight,
                    PlayerAnimationState::Stabilized
                    | PlayerAnimationState::HalfRight
                    | PlayerAnimationState::HalfLeft => PlayerAnimationState::Stabilized,
                }
            };

            // Updates if state is changed
            if new_animation_state != player.animation_state {
                player.transition_instant = now;
                player.animation_state = new_animation_state;
                sprite.index = match player.animation_state {
                    PlayerAnimationState::FullLeft => 0,
                    PlayerAnimationState::HalfLeft => 1,
                    PlayerAnimationState::Stabilized => 2,
                    PlayerAnimationState::HalfRight => 3,
                    PlayerAnimationState::FullRight => 4,
                };
            }
        }
    }
}

/// Periodically change the index to the sprite in the spritesheet
pub fn entities_animation(
    time: Res<Time>,
    mut sprite: Mut<TextureAtlasSprite>,
    mut animatable: Mut<Animatable>,
) {
    animatable.cycle_timer.tick(time.delta_seconds);
    if animatable.cycle_timer.finished {
        sprite.index = (sprite.index + animatable.sprite_idx_delta) % animatable.sprite_count;
    }
}
