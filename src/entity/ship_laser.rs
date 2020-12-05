use crate::{
    components::{Animation, HitBox, ShipLaser, TimeToLive, Velocity},
    constant::{
        ANIMATION_INTERVAL, SHIP_LASER_INITIAL_VELOCITY, SHIP_LASER_SPRITE_HEIGHT,
        SHIP_LASER_SPRITE_WIDTH, SHIP_LASER_TIME_TO_LIVE_DURATION,
    },
    events::ShipLaserSpawnEvent,
    resource::{EventReaders, TextureAtlasHandles},
};
use bevy::prelude::*;

#[derive(Bundle)]
pub struct ShipLaserComponents {
    pub ship_laser: ShipLaser,
    pub time_to_live: TimeToLive,
    pub velocity: Velocity,
    pub hit_box: HitBox,
    pub animation: Animation,
}

pub fn ship_laser_spawn_event_listener(
    mut commands: Commands,
    ship_laser_spawn_events: Res<Events<ShipLaserSpawnEvent>>,
    texture_atlas_handles: Res<TextureAtlasHandles>,
    mut event_readers: ResMut<EventReaders>,
) {
    for ship_laser_spawn_event in event_readers
        .ship_laser_spawn
        .iter(&ship_laser_spawn_events)
    {
        commands
            .spawn(SpriteSheetComponents {
                texture_atlas: texture_atlas_handles.laser_bolts.clone(),
                transform: Transform {
                    translation: ship_laser_spawn_event.laser_translation,
                    ..Default::default()
                },
                sprite: TextureAtlasSprite::new(1),
                ..Default::default()
            })
            .with_bundle(ShipLaserComponents {
                ship_laser: ShipLaser,
                time_to_live: TimeToLive(Timer::new(SHIP_LASER_TIME_TO_LIVE_DURATION, false)),
                hit_box: HitBox(Vec2::new(SHIP_LASER_SPRITE_WIDTH, SHIP_LASER_SPRITE_HEIGHT)),
                velocity: Velocity(Vec2::new(
                    SHIP_LASER_INITIAL_VELOCITY.0,
                    SHIP_LASER_INITIAL_VELOCITY.1,
                )),
                animation: Animation {
                    idx_delta: 2,
                    sprite_count: 4,
                    timer: Timer::new(ANIMATION_INTERVAL, true),
                },
            });
    }
}
