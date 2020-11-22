use bevy::prelude::*;

use std::time::{Duration, Instant};

use crate::component::{Animatable, Enemy, MoveDirection, MoveSpeed, Player, PlayerAnimationState};

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
        .with(MoveSpeed(500.))
        .with(MoveDirection::default())
        .with(Animatable {
            sprite_cycle_delta: 5,
            cycle_timer: Timer::new(Duration::from_millis(200), true),
        });
}

pub fn create_enemy(commands: &mut Commands, texture_atlas_handle: Handle<TextureAtlas>) {
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
            sprite_cycle_delta: 1,
            cycle_timer: Timer::new(Duration::from_millis(200), true),
        });
}
