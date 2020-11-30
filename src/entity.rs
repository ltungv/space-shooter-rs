use crate::{
    component::{Animation, Enemy, EnemySpawner, HitBox, Motion, Ship, ShipAnimationState},
    constant::*,
    resource::EnemyData,
};
use bevy::prelude::*;
use std::time::Instant;

/// Create a camera
pub fn initialize_camera(mut commands: Commands) {
    commands.spawn(Camera2dComponents::default());
}

/// Add a new entity to the world with all the needed components to represent a ship
pub fn initialize_ship(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_atlas = TextureAtlas::from_grid(
        asset_server.load("spritesheets/ship.png"),
        Vec2::new(SHIP_SPRITE_WIDTH, SHIP_SPRITE_HEIGHT),
        5,
        2,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn(SpriteSheetComponents {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(SPRITE_SCALING_FACTOR)),
            sprite: TextureAtlasSprite::new(2),
            ..Default::default()
        })
        .with(Ship {
            animation_state: ShipAnimationState::Stabilized,
            transition_instant: Instant::now(),
            transition_duration: SHIP_STATE_TRANSITION_DURATION,
        })
        .with(HitBox {
            width: SHIP_SPRITE_WIDTH * SPRITE_SCALING_FACTOR,
            height: SHIP_SPRITE_HEIGHT * SPRITE_SCALING_FACTOR,
        })
        .with(Motion {
            max_speed: SHIP_MAX_SPEED,
            velocity: Vec2::default(),
        })
        .with(Animation {
            idx_delta: 5,
            sprite_count: 10,
            timer: Timer::new(ANIMATION_INTERVAL, true),
        });
}

/// Add a new entity to the world with all the needed components to represent an enemy
pub fn create_enemy(commands: &mut Commands, translation: Vec3, enemy_data: EnemyData) {
    commands
        .spawn(SpriteSheetComponents {
            texture_atlas: enemy_data.texture_atlas_handle,
            transform: Transform {
                scale: Vec3::splat(SPRITE_SCALING_FACTOR),
                translation,
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Enemy {
            variant: enemy_data.variant,
        })
        .with(HitBox {
            width: enemy_data.texture_size.x() * SPRITE_SCALING_FACTOR,
            height: enemy_data.texture_size.y() * SPRITE_SCALING_FACTOR,
        })
        .with(Motion {
            max_speed: ENEMY_MAX_SPEED,
            velocity: Vec2::new(ENEMY_INITIAL_VELOCITY.0, ENEMY_INITIAL_VELOCITY.1),
        })
        .with(Animation {
            idx_delta: 1,
            sprite_count: 2,
            timer: Timer::new(ANIMATION_INTERVAL, true),
        });
}

/// Create a new enemy spawner
pub fn initialize_enemies_spawner(mut commands: Commands) {
    commands.spawn((EnemySpawner {
        timer: Timer::new(ENEMY_SPAWN_INTERVAL, true),
        weights: vec![
            ("small".to_string(), SPAWN_WEIGHT_ENEMY_SMALL),
            ("medium".to_string(), SPAWN_WEIGHT_ENEMY_MEDIUM),
            ("big".to_string(), SPAWN_WEIGHT_ENEMY_BIG),
        ],
    },));
}
