use bevy::prelude::*;

use std::time::{Duration, Instant};

use crate::component::{Animatable, Enemy, Motion2D, Player, PlayerAnimationState};

/// Add a new entity to the world with all the needed components to represent a player
pub fn create_player(commands: &mut Commands, texture_atlas_handle: Handle<TextureAtlas>) {
    commands
        .spawn(SpriteSheetComponents {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(4.)),
            sprite: TextureAtlasSprite::new(2),
            ..Default::default()
        })
        .with(Player {
            animation_state: PlayerAnimationState::Stabilized,
            transition_instant: Instant::now(),
            transition_duration: Duration::from_millis(100),
        })
        .with(Motion2D {
            max_speed: 500.,
            ..Default::default()
        })
        .with(Animatable {
            sprite_idx_delta: 5,
            sprite_count: 10,
            cycle_timer: Timer::new(Duration::from_millis(200), true),
        });
}

/// Add a new entity to the world with all the needed components to represent an enemy
pub fn create_enemy(commands: &mut Commands, texture_atlas_handle: Handle<TextureAtlas>) {
    // TODO: Enemy's position should be determined by the caller of the function
    commands
        .spawn(SpriteSheetComponents {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                scale: Vec3::splat(4.),
                translation: Vec3::new(150., 0., 0.),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Enemy)
        .with(Animatable {
            sprite_idx_delta: 1,
            sprite_count: 2,
            cycle_timer: Timer::new(Duration::from_millis(200), true),
        });
}
