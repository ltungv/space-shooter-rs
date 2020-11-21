use crate::components;
use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::prelude::*;

#[allow(clippy::too_many_arguments)]
pub fn player_movement(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    _player: &components::Player,
    move_speed: &components::MoveSpeed,
    move_direction: &components::MoveDirection,
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
    _player: &components::Player,
    mut move_direction: Mut<components::MoveDirection>,
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
    mut player: Mut<components::Player>,
    move_direction: &components::MoveDirection,
    mut sprite: Mut<TextureAtlasSprite>,
) {
    if let Some(now) = time.instant {
        if now.duration_since(player.last_transition_instant) >= player.stabilization_duration {
            let x_direction = move_direction.0.x();
            let new_animation_state = if x_direction < 0. {
                player.animation_state.transition_left()
            } else if x_direction > 0. {
                player.animation_state.transition_right()
            } else {
                player.animation_state.transition_stable()
            };

            if new_animation_state != player.animation_state {
                player.last_transition_instant = now;
                player.animation_state = new_animation_state;
                match player.animation_state {
                    components::PlayerAnimationState::FullLeft => sprite.index = 0,
                    components::PlayerAnimationState::HalfLeft => sprite.index = 1,
                    components::PlayerAnimationState::Stabilized => sprite.index = 2,
                    components::PlayerAnimationState::HalfRight => sprite.index = 3,
                    components::PlayerAnimationState::FullRight => sprite.index = 4,
                }
            }
        }
    }
}

pub fn entities_animation(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    texture_atlas_handle: &Handle<TextureAtlas>,
    mut sprite: Mut<TextureAtlasSprite>,
    mut animatable: Mut<components::Animatable>,
) {
    animatable.cycle_timer.tick(time.delta_seconds);
    if animatable.cycle_timer.finished {
        let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
        sprite.index = ((sprite.index as usize + animatable.sprite_cycle_delta)
            % texture_atlas.textures.len()) as u32;
    }
}
