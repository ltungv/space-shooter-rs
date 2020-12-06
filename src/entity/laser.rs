use crate::{
    components::{Animation, HitBox, Laser, TimeToLive, Velocity},
    constant::ANIMATION_INTERVAL,
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
) {
    for evt in event_readers.spawn_laser.iter(&spawn_laser_events) {
        commands
            .spawn(SpriteSheetComponents {
                texture_atlas: texture_atlas_handles.laser_bolts.clone(),
                transform: Transform {
                    translation: evt.laser_translation,
                    ..Default::default()
                },
                sprite: TextureAtlasSprite::new(evt.laser_initial_sprite_idx),
                ..Default::default()
            })
            .with_bundle(LaserComponents {
                laser: Laser {
                    source: evt.laser_source,
                },
                time_to_live: evt.laser_time_to_live.clone(),
                hit_box: evt.laser_hit_box.clone(),
                velocity: evt.laser_velocity.clone(),
                animation: Animation {
                    idx_delta: 2,
                    sprite_count: 4,
                    timer: Timer::new(ANIMATION_INTERVAL, true),
                },
            });
    }
}
