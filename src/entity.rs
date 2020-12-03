use crate::{
    component::{Animation, EnemySpawner, HitBox, Ship, ShipAnimationState, Velocity},
    constant::{
        ANIMATION_INTERVAL, ENEMY_SPAWN_INTERVAL, SHIP_INITIAL_MOVE_SPEED,
        SHIP_LASER_COOLDOWN_DURATION, SHIP_SPRITE_HEIGHT, SHIP_SPRITE_WIDTH,
        SHIP_STATE_TRANSITION_DURATION, SPAWN_WEIGHT_ENEMY_BIG, SPAWN_WEIGHT_ENEMY_MEDIUM,
        SPAWN_WEIGHT_ENEMY_SMALL, SPRITE_SCALING_FACTOR,
    },
};
use bevy::prelude::*;

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
            // global_transform: GlobalTransform::from_scale(Vec3::splat(SPRITE_SCALING_FACTOR)),
            sprite: TextureAtlasSprite::new(2),
            ..Default::default()
        })
        .with(Ship {
            move_speed: SHIP_INITIAL_MOVE_SPEED,
            animation_state: ShipAnimationState::Stabilized,
            laser_cooldown_timer: Timer::new(SHIP_LASER_COOLDOWN_DURATION, false),
            transition_timer: Timer::new(SHIP_STATE_TRANSITION_DURATION, false),
        })
        .with(HitBox(Vec2::new(
            SHIP_SPRITE_WIDTH * SPRITE_SCALING_FACTOR,
            SHIP_SPRITE_HEIGHT * SPRITE_SCALING_FACTOR,
        )))
        .with(Velocity(Vec2::default()))
        .with(Animation {
            idx_delta: 5,
            sprite_count: 10,
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
