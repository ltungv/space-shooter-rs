use crate::{
    components::{Animation, HitBox, Ship, ShipAnimationState, Velocity},
    constant::{
        ANIMATION_INTERVAL, SHIP_INITIAL_MOVE_SPEED, SHIP_LASER_COOLDOWN_DURATION,
        SHIP_SPRITE_HEIGHT, SHIP_SPRITE_WIDTH, SHIP_STATE_TRANSITION_DURATION,
    },
};
use bevy::prelude::*;

#[derive(Bundle)]
pub struct ShipComponents {
    pub ship: Ship,
    pub ship_animation_state: ShipAnimationState,
    pub velocity: Velocity,
    pub hit_box: HitBox,
    pub animation: Animation,
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
        .with_bundle(ShipComponents {
            ship: Ship {
                move_speed: SHIP_INITIAL_MOVE_SPEED,
                laser_cooldown_timer: Timer::new(SHIP_LASER_COOLDOWN_DURATION, false),
                transition_timer: Timer::new(SHIP_STATE_TRANSITION_DURATION, false),
            },
            ship_animation_state: ShipAnimationState::Stabilized,
            hit_box: HitBox(Vec2::new(SHIP_SPRITE_WIDTH, SHIP_SPRITE_HEIGHT)),
            velocity: Velocity(Vec2::default()),
            animation: Animation {
                idx_delta: 5,
                sprite_count: 10,
                timer: Timer::new(ANIMATION_INTERVAL, true),
            },
        });
}
