use crate::{
    component::{Animatable, Enemy, HitBox, Motion, Player, PlayerAnimationState, Spawner},
    constant::{
        ENEMY_BIG_SPRITE_WIDTH, PLAYER_SPRITE_HEIGHT, PLAYER_SPRITE_WIDTH,
        SPRITE_UNIFORM_SCALING_FACTOR,
    },
};
use bevy::prelude::{
    Commands, Handle, SpriteSheetComponents, TextureAtlas, TextureAtlasSprite, Timer, Transform,
    Vec2, Vec3,
};
use std::time::{Duration, Instant};

/// Add a new entity to the world with all the needed components to represent a player
pub fn create_player(commands: &mut Commands, texture_atlas_handle: Handle<TextureAtlas>) {
    commands
        .spawn(SpriteSheetComponents {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(SPRITE_UNIFORM_SCALING_FACTOR)),
            sprite: TextureAtlasSprite::new(2),
            ..Default::default()
        })
        .with(Player {
            animation_state: PlayerAnimationState::Stabilized,
            transition_instant: Instant::now(),
            transition_duration: Duration::from_millis(100),
        })
        .with(HitBox {
            width: PLAYER_SPRITE_WIDTH * SPRITE_UNIFORM_SCALING_FACTOR,
            height: PLAYER_SPRITE_HEIGHT * SPRITE_UNIFORM_SCALING_FACTOR,
        })
        .with(Motion {
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
pub fn create_enemy(
    commands: &mut Commands,
    texture_atlas_handle: Handle<TextureAtlas>,
    translation: Vec3,
) {
    commands
        .spawn(SpriteSheetComponents {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                translation,
                scale: Vec3::splat(SPRITE_UNIFORM_SCALING_FACTOR),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Enemy)
        .with(HitBox {
            width: ENEMY_BIG_SPRITE_WIDTH * SPRITE_UNIFORM_SCALING_FACTOR,
            height: ENEMY_BIG_SPRITE_WIDTH * SPRITE_UNIFORM_SCALING_FACTOR,
        })
        .with(Motion {
            max_speed: 100.0,
            velocity: Vec2::new(0.0, -80),
        })
        .with(Animatable {
            sprite_idx_delta: 1,
            sprite_count: 2,
            cycle_timer: Timer::new(Duration::from_millis(200), true),
        });
}

pub fn create_spawner(commands: &mut Commands, spawn_timer: Timer) {
    commands.spawn((Spawner { spawn_timer },));
}
