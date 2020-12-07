use crate::{
    components::{Animation, Enemy, EnemyVariant, HitBox, Velocity, Weapon},
    constant::{
        ANIMATION_INTERVAL, ENEMY_BIG_SPRITE_HEIGHT, ENEMY_BIG_SPRITE_WIDTH,
        ENEMY_INITIAL_VELOCITY, ENEMY_LASER_COOLDOWN_DURATION, ENEMY_LASER_INITIAL_VELOCITY,
        ENEMY_LASER_SPRITE_HEIGHT, ENEMY_LASER_SPRITE_WIDTH, ENEMY_LASER_TIME_TO_LIVE_DURATION,
        ENEMY_MEDIUM_SPRITE_HEIGHT, ENEMY_MEDIUM_SPRITE_WIDTH, ENEMY_SMALL_SPRITE_HEIGHT,
        ENEMY_SMALL_SPRITE_WIDTH,
    },
    entity::WeaponComponents,
    events::SpawnEnemyEvent,
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

pub fn spawn_enemy(
    mut commands: Commands,
    spawn_enemy_events: Res<Events<SpawnEnemyEvent>>,
    texture_atlas_handles: Res<TextureAtlasHandles>,
    mut event_readers: ResMut<EventReaders>,
) {
    for evt in event_readers.spawn_enemy.iter(&spawn_enemy_events) {
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
                    variant: evt.enemy_variant,
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

        match evt.enemy_variant {
            EnemyVariant::Medium | EnemyVariant::Big => {
                commands.with_children(|parent| {
                    let mut weapon_cooldown_timer =
                        Timer::new(ENEMY_LASER_COOLDOWN_DURATION, false);
                    weapon_cooldown_timer.tick(ENEMY_LASER_COOLDOWN_DURATION.as_secs_f32());

                    parent.spawn(WeaponComponents {
                        weapon: Weapon {
                            cooldown_timer: weapon_cooldown_timer,
                            laser_velocity: Velocity(Vec2::new(
                                ENEMY_LASER_INITIAL_VELOCITY.0,
                                ENEMY_LASER_INITIAL_VELOCITY.1,
                            )),
                            laser_hit_box: HitBox(Vec2::new(
                                ENEMY_LASER_SPRITE_WIDTH,
                                ENEMY_LASER_SPRITE_HEIGHT,
                            )),
                            laser_time_to_live_duration: ENEMY_LASER_TIME_TO_LIVE_DURATION,
                            laser_initial_sprite_idx: 0,
                        },
                        transform: Transform {
                            translation: -hit_box_vec2.y() * Vec3::unit_y(),
                            ..Default::default()
                        },
                        global_transform: Default::default(),
                    });
                });
            }
            _ => {}
        }
    }
}
