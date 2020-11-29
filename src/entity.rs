use crate::{
    component::{
        Animation, Enemy, EnemySpawner, EnemyVariant, HitBox, Motion, Ship, ShipAnimationState,
    },
    constant::{SHIP_SPRITE_HEIGHT, SHIP_SPRITE_WIDTH},
    resource::EnemyData,
};
use bevy::prelude::*;
use std::time::{Duration, Instant};

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
            sprite: TextureAtlasSprite::new(2),
            ..Default::default()
        })
        .with(Ship {
            animation_state: ShipAnimationState::Stabilized,
            transition_instant: Instant::now(),
            transition_duration: Duration::from_millis(100),
        })
        .with(HitBox {
            width: SHIP_SPRITE_WIDTH,
            height: SHIP_SPRITE_HEIGHT,
        })
        .with(Motion {
            max_speed: 500.,
            velocity: Vec2::default(),
        })
        .with(Animation {
            idx_delta: 5,
            sprite_count: 10,
            timer: Timer::new(Duration::from_millis(200), true),
        });
}

/// Add a new entity to the world with all the needed components to represent an enemy
pub fn create_enemy(
    commands: &mut Commands,
    texture_atlas_handle: Handle<TextureAtlas>,
    variant: EnemyVariant,
    hit_box: HitBox,
    translation: Vec3,
) {
    commands
        .spawn(SpriteSheetComponents {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                translation,
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Enemy { variant })
        .with(hit_box)
        .with(Motion {
            max_speed: 100.0,
            velocity: Vec2::new(0.0, -80.),
        })
        .with(Animation {
            idx_delta: 1,
            sprite_count: 2,
            timer: Timer::new(Duration::from_millis(200), true),
        });
}

pub fn initialize_enemies_spawner(mut commands: Commands) {
    commands.spawn((EnemySpawner {
        timer: Timer::new(Duration::from_secs(1), true),
        weights: vec![
            ("small".to_string(), 5),
            ("medium".to_string(), 3),
            ("big".to_string(), 2),
        ],
    },));
}
