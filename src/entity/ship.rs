use crate::{
    components::{
        Animation, ConstrainedToArena, HitBox, Ship, ShipAnimationState, TimeToLive, Velocity,
        Weapon,
    },
    constant::{
        ANIMATION_INTERVAL, SHIP_INITIAL_MOVE_SPEED, SHIP_LASER_COOLDOWN_DURATION,
        SHIP_LASER_INITIAL_VELOCITY, SHIP_LASER_SPRITE_HEIGHT, SHIP_LASER_SPRITE_WIDTH,
        SHIP_LASER_TIME_TO_LIVE_DURATION, SHIP_SPRITE_HEIGHT, SHIP_SPRITE_WIDTH,
        SHIP_STATE_TRANSITION_DURATION,
    },
    entity::WeaponComponents,
    resource::TextureAtlasHandles,
};
use bevy::prelude::*;

#[derive(Bundle)]
pub struct ShipComponents {
    pub ship: Ship,
    pub constrained_to_arena: ConstrainedToArena,
    pub velocity: Velocity,
    pub hit_box: HitBox,
    pub animation: Animation,
}

/// Add a new entity to the world with all the needed components to represent a ship
pub fn initialize_ship(mut commands: Commands, texture_atlas_handles: Res<TextureAtlasHandles>) {
    commands
        .spawn(SpriteSheetComponents {
            texture_atlas: texture_atlas_handles.ship.clone(),
            sprite: TextureAtlasSprite::new(2),
            ..Default::default()
        })
        .with_bundle(ShipComponents {
            ship: Ship {
                animation_state: ShipAnimationState::Stabilized,
                move_speed: SHIP_INITIAL_MOVE_SPEED,
                transition_timer: Timer::new(SHIP_STATE_TRANSITION_DURATION, false),
            },
            constrained_to_arena: ConstrainedToArena,
            hit_box: HitBox(Vec2::new(SHIP_SPRITE_WIDTH, SHIP_SPRITE_HEIGHT)),
            velocity: Velocity(Vec2::default()),
            animation: Animation {
                idx_delta: 5,
                sprite_count: 10,
                timer: Timer::new(ANIMATION_INTERVAL, true),
            },
        })
        .with_children(|parent| {
            let mut weapon_cooldown_timer = Timer::new(SHIP_LASER_COOLDOWN_DURATION, false);
            weapon_cooldown_timer.tick(SHIP_LASER_COOLDOWN_DURATION.as_secs_f32());

            parent.spawn(WeaponComponents {
                weapon: Weapon {
                    cooldown_timer: weapon_cooldown_timer,
                    laser_velocity: Velocity(Vec2::new(
                        SHIP_LASER_INITIAL_VELOCITY.0,
                        SHIP_LASER_INITIAL_VELOCITY.1,
                    )),
                    laser_hit_box: HitBox(Vec2::new(
                        SHIP_LASER_SPRITE_WIDTH,
                        SHIP_LASER_SPRITE_HEIGHT,
                    )),
                    laser_time_to_live: TimeToLive(Timer::new(
                        SHIP_LASER_TIME_TO_LIVE_DURATION,
                        false,
                    )),
                    laser_initial_sprite_idx: 1,
                },
                transform: Transform {
                    translation: SHIP_SPRITE_HEIGHT * Vec3::unit_y(),
                    ..Default::default()
                },
                global_transform: Default::default(),
            });
        });
}
