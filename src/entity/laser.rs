use crate::{
    components::{Animation, HitBox, Laser, Ship, TimeToLive, Velocity},
    constant::{
        ANIMATION_INTERVAL, ENEMY_LASER_INITIAL_VELOCITY, ENEMY_LASER_SPRITE_HEIGHT,
        ENEMY_LASER_SPRITE_WIDTH, ENEMY_LASER_TIME_TO_LIVE_DURATION, SHIP_LASER_INITIAL_VELOCITY,
        SHIP_LASER_SPRITE_HEIGHT, SHIP_LASER_SPRITE_WIDTH, SHIP_LASER_TIME_TO_LIVE_DURATION,
    },
    events::SpawnLaserEvent,
    resource::{EventReaders, TextureAtlasHandles},
};
use bevy::prelude::*;

#[derive(Bundle)]
pub struct LaserComponents {
    pub laser: Laser,
    pub time_to_live: TimeToLive,
    pub velocity: Velocity,
    pub hit_box: HitBox,
    pub animation: Animation,
}

pub fn spawn_laser(
    mut commands: Commands,
    spawn_laser_events: Res<Events<SpawnLaserEvent>>,
    texture_atlas_handles: Res<TextureAtlasHandles>,
    mut event_readers: ResMut<EventReaders>,
    query_ship: Query<&Ship>,
) {
    for evt in event_readers.spawn_laser.iter(&spawn_laser_events) {
        let mut sprite = TextureAtlasSprite::new(0);
        let mut time_to_live = TimeToLive(Timer::new(ENEMY_LASER_TIME_TO_LIVE_DURATION, false));
        let mut hit_box = HitBox(Vec2::new(
            ENEMY_LASER_SPRITE_WIDTH,
            ENEMY_LASER_SPRITE_HEIGHT,
        ));
        let mut velocity = Velocity(Vec2::new(
            ENEMY_LASER_INITIAL_VELOCITY.0,
            ENEMY_LASER_INITIAL_VELOCITY.1,
        ));

        if query_ship.get(evt.laser_source).is_ok() {
            sprite = TextureAtlasSprite::new(1);
            time_to_live = TimeToLive(Timer::new(SHIP_LASER_TIME_TO_LIVE_DURATION, false));
            hit_box = HitBox(Vec2::new(SHIP_LASER_SPRITE_WIDTH, SHIP_LASER_SPRITE_HEIGHT));
            velocity = Velocity(Vec2::new(
                SHIP_LASER_INITIAL_VELOCITY.0,
                SHIP_LASER_INITIAL_VELOCITY.1,
            ));
        }

        commands
            .spawn(SpriteSheetComponents {
                texture_atlas: texture_atlas_handles.laser_bolts.clone(),
                transform: Transform {
                    translation: evt.laser_translation,
                    ..Default::default()
                },
                sprite,
                ..Default::default()
            })
            .with_bundle(LaserComponents {
                laser: Laser {
                    source: evt.laser_source,
                },
                time_to_live,
                hit_box,
                velocity,
                animation: Animation {
                    idx_delta: 2,
                    sprite_count: 4,
                    timer: Timer::new(ANIMATION_INTERVAL, true),
                },
            });
    }
}
