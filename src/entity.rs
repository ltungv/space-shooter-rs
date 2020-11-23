use crate::component::{Animatable, Enemy, HitBox, Motion, Player, PlayerAnimationState};
use bevy::prelude::{
    Commands, Handle, SpriteSheetComponents, TextureAtlas, TextureAtlasSprite, Timer, Transform,
    Vec3,
};
use std::time::{Duration, Instant};

/// Add a new entity to the world with all the needed components to represent a player
pub fn create_player(commands: &mut Commands, texture_atlas_handle: Handle<TextureAtlas>) {
    commands
        .spawn(SpriteSheetComponents {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(4.0)),
            sprite: TextureAtlasSprite::new(2),
            ..Default::default()
        })
        .with(Player {
            animation_state: PlayerAnimationState::Stabilized,
            transition_instant: Instant::now(),
            transition_duration: Duration::from_millis(100),
        })
        .with(Motion{
            max_speed: 500.,
            ..Default::default()
        })
        .with(HitBox {
            width: 16. * 4.,
            height: 24. * 4.,
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
