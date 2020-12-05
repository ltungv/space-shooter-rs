use crate::{
    components::{Animation, Explosion, TimeToLive},
    constant::ANIMATION_INTERVAL,
    events::SpawnExplosionEvent,
    resource::{EventReaders, TextureAtlasHandles},
};
use bevy::prelude::*;

#[derive(Bundle)]
pub struct ExplosionComponents {
    pub explosion: Explosion,
    pub time_to_live: TimeToLive,
    pub animation: Animation,
}

pub fn spawn_explosion(
    mut commands: Commands,
    spawn_explosion_events: Res<Events<SpawnExplosionEvent>>,
    texture_atlas_handles: Res<TextureAtlasHandles>,
    mut event_readers: ResMut<EventReaders>,
) {
    for evt in event_readers.spawn_explosion.iter(&spawn_explosion_events) {
        commands
            .spawn(SpriteSheetComponents {
                texture_atlas: texture_atlas_handles.explosion.clone(),
                transform: Transform {
                    translation: evt.explosion_translation,
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_bundle(ExplosionComponents {
                explosion: Explosion,
                time_to_live: evt.explosion_time_to_live.clone(),
                animation: Animation {
                    idx_delta: 1,
                    sprite_count: 2,
                    timer: Timer::new(ANIMATION_INTERVAL, true),
                },
            });
    }
}
