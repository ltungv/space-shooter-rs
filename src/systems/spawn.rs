use crate::{
    components::{EnemySpawner, EnemyVariant},
    constant::{
        ARENA_HEIGHT, ARENA_WIDTH, ENEMY_BIG_SPRITE_HEIGHT, ENEMY_BIG_SPRITE_WIDTH,
        ENEMY_MEDIUM_SPRITE_HEIGHT, ENEMY_MEDIUM_SPRITE_WIDTH, ENEMY_SMALL_SPRITE_HEIGHT,
        ENEMY_SMALL_SPRITE_WIDTH, SPRITE_SCALING_FACTOR,
    },
    entity,
    resource::GameState,
};
use bevy::prelude::*;
use rand::prelude::*;

/// Go through all enemy spawners and check if they ready to spawn new entity,
/// create entity as the spawn timer finishes.
pub fn enemy_spawner_trigger(
    commands: Commands,
    time: Res<Time>,
    game_state: Res<GameState>,
    mut enemy_spawner: Mut<EnemySpawner>,
) {
    enemy_spawner.timer.tick(time.delta_seconds);
    if enemy_spawner.timer.just_finished {
        // Choose the name of the enemy to be spawned
        let mut rng = rand::thread_rng();
        let enemy_variant = enemy_spawner
            .weights
            .choose_weighted(&mut rng, |item| item.1)
            .expect("Could not choose spawnable")
            .0
            .clone();

        let (enemy_width, enemy_height) = match enemy_variant {
            EnemyVariant::Small => (
                ENEMY_SMALL_SPRITE_WIDTH * SPRITE_SCALING_FACTOR,
                ENEMY_SMALL_SPRITE_HEIGHT * SPRITE_SCALING_FACTOR,
            ),
            EnemyVariant::Medium => (
                ENEMY_MEDIUM_SPRITE_WIDTH * SPRITE_SCALING_FACTOR,
                ENEMY_MEDIUM_SPRITE_HEIGHT * SPRITE_SCALING_FACTOR,
            ),
            EnemyVariant::Big => (
                ENEMY_BIG_SPRITE_WIDTH * SPRITE_SCALING_FACTOR,
                ENEMY_BIG_SPRITE_HEIGHT * SPRITE_SCALING_FACTOR,
            ),
        };

        // Enemy comes from the top of the screen with random x-axis position
        let max_offset_x_from_center = (ARENA_WIDTH - enemy_width) / 2.;
        let rand_translation_x_range = rng.gen::<f32>() * 2. * max_offset_x_from_center;

        let translation_x = -max_offset_x_from_center + rand_translation_x_range;
        let translation_y = (ARENA_HEIGHT + enemy_height) / 2.;
        let translation = Vec3::new(translation_x, translation_y, 0.);

        entity::spawn_enemy(commands, game_state, enemy_variant, translation);
    }
}
