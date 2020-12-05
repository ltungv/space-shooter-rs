use crate::{
    components::{Animation, Enemy, EnemyVariant, HitBox, Velocity},
    constant::{
        ANIMATION_INTERVAL, ENEMY_BIG_SPRITE_HEIGHT, ENEMY_BIG_SPRITE_WIDTH,
        ENEMY_INITIAL_VELOCITY, ENEMY_MEDIUM_SPRITE_HEIGHT, ENEMY_MEDIUM_SPRITE_WIDTH,
        ENEMY_SMALL_SPRITE_HEIGHT, ENEMY_SMALL_SPRITE_WIDTH,
    },
    resource::GameState,
};
use bevy::prelude::*;

#[derive(Bundle)]
pub struct EnemyComponents {
    pub enemy: Enemy,
    pub enemy_variant: EnemyVariant,
    pub velocity: Velocity,
    pub hit_box: HitBox,
    pub animation: Animation,
}

pub fn spawn_enemy(
    mut commands: Commands,
    game_state: Res<GameState>,
    enemy_variant: EnemyVariant,
    translation: Vec3,
) {
    let (hit_box_vec2, texture_atlas_handle_key) = match enemy_variant {
        EnemyVariant::Small => (
            Vec2::new(ENEMY_SMALL_SPRITE_WIDTH, ENEMY_SMALL_SPRITE_HEIGHT),
            "enemy-small",
        ),
        EnemyVariant::Medium => (
            Vec2::new(ENEMY_MEDIUM_SPRITE_WIDTH, ENEMY_MEDIUM_SPRITE_HEIGHT),
            "enemy-medium",
        ),
        EnemyVariant::Big => (
            Vec2::new(ENEMY_BIG_SPRITE_WIDTH, ENEMY_BIG_SPRITE_HEIGHT),
            "enemy-big",
        ),
    };
    let texture_atlas = game_state.texture_atlas_handles[texture_atlas_handle_key].clone();

    commands
        .spawn(SpriteSheetComponents {
            texture_atlas,
            transform: Transform {
                translation,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_bundle(EnemyComponents {
            enemy: Enemy,
            enemy_variant,
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
