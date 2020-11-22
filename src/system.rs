use bevy::prelude::*;

use crate::component::{Animatable, MoveDirection, MoveSpeed, Player, PlayerAnimationState};
use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

#[allow(clippy::too_many_arguments)]
pub fn player_movement(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    _player: &Player,
    move_speed: &MoveSpeed,
    move_direction: &MoveDirection,
    texture_atlas_handle: &Handle<TextureAtlas>,
    sprite: &TextureAtlasSprite,
    mut transform: Mut<Transform>,
) {
    let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
    let texture_rect = texture_atlas.textures[sprite.index as usize];

    // Get size of the player's sprite on screen
    let width = texture_rect.width() * transform.scale.x();
    let height = texture_rect.height() * transform.scale.y();

    // X-axis movement
    *transform.translation.x_mut() += time.delta_seconds * move_direction.0.x() * move_speed.0;
    *transform.translation.x_mut() = transform
        .translation
        .x()
        // update bound
        .min((WINDOW_WIDTH - width) / 2.)
        // lower bound
        .max(-(WINDOW_WIDTH - width) / 2.);

    // Y-axis movement
    *transform.translation.y_mut() += time.delta_seconds * move_direction.0.y() * move_speed.0;
    *transform.translation.y_mut() = transform
        .translation
        .y()
        // upper bound
        .min((WINDOW_HEIGHT - height) / 2.)
        // lower bound
        .max(-(WINDOW_HEIGHT - height) / 2.);
}

pub fn player_control(
    kb_input: Res<Input<KeyCode>>,
    _player: &Player,
    mut move_direction: Mut<MoveDirection>,
) {
    *move_direction.0.y_mut() = 0.;
    *move_direction.0.x_mut() = 0.;

    if kb_input.pressed(KeyCode::Up) {
        *move_direction.0.y_mut() += 1.;
    }
    if kb_input.pressed(KeyCode::Down) {
        *move_direction.0.y_mut() -= 1.;
    }
    if kb_input.pressed(KeyCode::Left) {
        *move_direction.0.x_mut() -= 1.;
    }
    if kb_input.pressed(KeyCode::Right) {
        *move_direction.0.x_mut() += 1.;
    }
}

pub fn player_state_transition(
    time: Res<Time>,
    mut player: Mut<Player>,
    move_direction: &MoveDirection,
    mut sprite: Mut<TextureAtlasSprite>,
) {
    if let Some(now) = time.instant {
        if now.duration_since(player.transition_instant) >= player.transition_duration {
            let x_direction = move_direction.0.x();
            let new_animation_state = if x_direction < 0. {
                match player.animation_state {
                    PlayerAnimationState::FullLeft | PlayerAnimationState::HalfLeft => {
                        PlayerAnimationState::FullLeft
                    }
                    PlayerAnimationState::Stabilized => PlayerAnimationState::HalfLeft,
                    PlayerAnimationState::HalfRight => PlayerAnimationState::Stabilized,
                    PlayerAnimationState::FullRight => PlayerAnimationState::HalfRight,
                }
            } else if x_direction > 0. {
                match player.animation_state {
                    PlayerAnimationState::FullLeft => PlayerAnimationState::HalfLeft,
                    PlayerAnimationState::HalfLeft => PlayerAnimationState::Stabilized,
                    PlayerAnimationState::Stabilized => PlayerAnimationState::HalfRight,
                    PlayerAnimationState::HalfRight | PlayerAnimationState::FullRight => {
                        PlayerAnimationState::FullRight
                    }
                }
            } else {
                match player.animation_state {
                    PlayerAnimationState::FullLeft => PlayerAnimationState::HalfLeft,
                    PlayerAnimationState::Stabilized
                    | PlayerAnimationState::HalfRight
                    | PlayerAnimationState::HalfLeft => PlayerAnimationState::Stabilized,
                    PlayerAnimationState::FullRight => PlayerAnimationState::HalfRight,
                }
            };

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

pub fn entities_animation(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    texture_atlas_handle: &Handle<TextureAtlas>,
    mut sprite: Mut<TextureAtlasSprite>,
    mut animatable: Mut<Animatable>,
) {
    animatable.cycle_timer.tick(time.delta_seconds);
    if animatable.cycle_timer.finished {
        let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
        sprite.index = ((sprite.index as usize + animatable.sprite_cycle_delta)
            % texture_atlas.textures.len()) as u32;
    }
}
