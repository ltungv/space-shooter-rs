use crate::{
    components::{Animation, Enemy, EnemyVariant, HitBox, Velocity},
    constant::{
        ANIMATION_INTERVAL, ENEMY_BIG_SPRITE_HEIGHT, ENEMY_BIG_SPRITE_WIDTH,
        ENEMY_INITIAL_VELOCITY, ENEMY_MEDIUM_SPRITE_HEIGHT, ENEMY_MEDIUM_SPRITE_WIDTH,
        ENEMY_SMALL_SPRITE_HEIGHT, ENEMY_SMALL_SPRITE_WIDTH,
    },
    events::EnemySpawnEvent,
    resource::{EventReaders, TextureAtlasHandles},
};
use bevy::prelude::*;

#[derive(Bundle)]
pub struct EnemyComponents {
    pub enemy: Enemy,
    pub velocity: Velocity,
    pub hit_box: HitBox,
    pub animation: Animation,
}

pub fn enemy_spawn_event_listener(
    mut commands: Commands,
    enemy_spawn_events: Res<Events<EnemySpawnEvent>>,
    texture_atlas_handles: Res<TextureAtlasHandles>,
    mut event_readers: ResMut<EventReaders>,
) {
    for evt in event_readers.enemy_spawn.iter(&enemy_spawn_events) {
        let (hit_box_vec2, texture_atlas) = match evt.enemy_variant {
            EnemyVariant::Small => (
                Vec2::new(ENEMY_SMALL_SPRITE_WIDTH, ENEMY_SMALL_SPRITE_HEIGHT),
                texture_atlas_handles.enemy_small.clone(),
            ),
            EnemyVariant::Medium => (
                Vec2::new(ENEMY_MEDIUM_SPRITE_WIDTH, ENEMY_MEDIUM_SPRITE_HEIGHT),
                texture_atlas_handles.enemy_medium.clone(),
            ),
            EnemyVariant::Big => (
                Vec2::new(ENEMY_BIG_SPRITE_WIDTH, ENEMY_BIG_SPRITE_HEIGHT),
                texture_atlas_handles.enemy_big.clone(),
            ),
        };

        commands
            .spawn(SpriteSheetComponents {
                texture_atlas,
                transform: Transform {
                    translation: evt.enemy_translation,
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_bundle(EnemyComponents {
                enemy: Enemy {
                    variant: evt.enemy_variant.clone(),
                },
                hit_box: HitBox(hit_box_vec2),
                velocity: Velocity(Vec2::new(
                    ENEMY_INITIAL_VELOCITY.0,
                    ENEMY_INITIAL_VELOCITY.1,
                )),
                animation: Animation {
                    idx_delta: 1,
                    sprite_count: 2,
                    timer: Timer::new(ANIMATION_INTERVAL, true),
                },
            });
    }
}
