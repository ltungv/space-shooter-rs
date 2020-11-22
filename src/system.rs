use bevy::prelude::*;

use crate::component::{Animatable, MoveDirection, MoveSpeed, Player, PlayerAnimationState};

/// Change player's position based on the moving speed and moving direction. Movement is limited
/// to the window viewable area
#[allow(clippy::too_many_arguments)]
pub fn player_movement(
    // Resources
    time: Res<Time>,
    window_description: Res<WindowDescriptor>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    // Player
    _player: &Player,
    move_speed: &MoveSpeed,
    move_direction: &MoveDirection,
    // SpriteSheetComponents
    sprite: &TextureAtlasSprite,
    texture_atlas_handle: &Handle<TextureAtlas>,
    mut transform: Mut<Transform>,
) {
    // TODO: Component for storing the boundaries with having to recalculate on every pass
    // Dimensions of the game's window
    let window_width = window_description.width as f32;
    let window_height = window_description.height as f32;
    // Dimensions of the sprite that represents the player
    let texture_atlas = texture_atlases
        .get(texture_atlas_handle)
        .expect("Could not get player's texture atlas");
    let texture_rect = texture_atlas.textures[sprite.index as usize];
    // Dimensions of the sprite that represents the player,
    // after the scaling factor is applied
    let player_width = texture_rect.width() * transform.scale.x();
    let player_height = texture_rect.height() * transform.scale.y();

    // X-axis movement
    *transform.translation.x_mut() += time.delta_seconds * move_direction.0.x() * move_speed.0;
    *transform.translation.x_mut() = transform
        .translation
        .x()
        // update bound
        .min((window_width - player_width) / 2.)
        // lower bound
        .max(-(window_width - player_width) / 2.);

    // Y-axis movement
    *transform.translation.y_mut() += time.delta_seconds * move_direction.0.y() * move_speed.0;
    *transform.translation.y_mut() = transform
        .translation
        .y()
        // upper bound
        .min((window_height - player_height) / 2.)
        // lower bound
        .max(-(window_height - player_height) / 2.);
}

/// Change player's directions based on user's keyboard input
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

/// Change the player's animation state and change the current index to the index of the sprite
/// that represents that state. The player has to be in the new state for at least some set amount
/// of duration before being able to change its state again
pub fn player_state_transition(
    time: Res<Time>,
    mut player: Mut<Player>,
    move_direction: &MoveDirection,
    mut sprite: Mut<TextureAtlasSprite>,
) {
    // State is not changed rapidly so that animation can be perceived by the player
    if let Some(now) = time.instant {
        if now.duration_since(player.transition_instant) >= player.transition_duration {
            // Determines the new state based on previous state and current moving direction
            let x_direction = move_direction.0.x();
            let new_animation_state = if x_direction < 0. {
                match player.animation_state {
                    PlayerAnimationState::Stabilized => PlayerAnimationState::HalfLeft,
                    PlayerAnimationState::HalfRight => PlayerAnimationState::Stabilized,
                    PlayerAnimationState::FullRight => PlayerAnimationState::HalfRight,
                    PlayerAnimationState::HalfLeft | PlayerAnimationState::FullLeft => {
                        PlayerAnimationState::FullLeft
                    }
                }
            } else if x_direction > 0. {
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
    texture_atlases: Res<Assets<TextureAtlas>>,
    texture_atlas_handle: &Handle<TextureAtlas>,
    mut sprite: Mut<TextureAtlasSprite>,
    mut animatable: Mut<Animatable>,
) {
    animatable.cycle_timer.tick(time.delta_seconds);
    if animatable.cycle_timer.finished {
        let texture_atlas = texture_atlases
            .get(texture_atlas_handle)
            .expect("Could not get entity's texture atlas");
        sprite.index = ((sprite.index as usize + animatable.sprite_cycle_delta)
            % texture_atlas.textures.len()) as u32;
    }
}
