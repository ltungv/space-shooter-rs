use crate::{
    components::{EnemySpawner, EnemyVariant},
    constant::{
        ARENA_HEIGHT, ARENA_WIDTH, ENEMY_BIG_SPRITE_HEIGHT, ENEMY_BIG_SPRITE_WIDTH,
        ENEMY_MEDIUM_SPRITE_HEIGHT, ENEMY_MEDIUM_SPRITE_WIDTH, ENEMY_SMALL_SPRITE_HEIGHT,
        ENEMY_SMALL_SPRITE_WIDTH,
    },
    events::SpawnEnemyEvent,
};
use bevy::prelude::*;
use rand::prelude::*;

/// Go through all enemy spawners and check if they ready to spawn new entity,
/// create entity as the spawn timer finishes.
pub fn spawn_enemy_trigger(
    time: Res<Time>,
    mut spawn_enemy_events: ResMut<Events<SpawnEnemyEvent>>,
    mut enemy_spawner: Mut<EnemySpawner>,
) {
    enemy_spawner.timer.tick(time.delta_seconds);
    if enemy_spawner.timer.finished {
        // Choose the name of the enemy to be spawned
        let mut rng = rand::thread_rng();
        let enemy_variant = enemy_spawner
            .weights
            .choose_weighted(&mut rng, |item| item.1)
            .expect("Could not choose spawnable")
            .0;

        let (enemy_width, enemy_height) = match enemy_variant {
            EnemyVariant::Small => (ENEMY_SMALL_SPRITE_WIDTH, ENEMY_SMALL_SPRITE_HEIGHT),
            EnemyVariant::Medium => (ENEMY_MEDIUM_SPRITE_WIDTH, ENEMY_MEDIUM_SPRITE_HEIGHT),
            EnemyVariant::Big => (ENEMY_BIG_SPRITE_WIDTH, ENEMY_BIG_SPRITE_HEIGHT),
        };

        // Enemy comes from the top of the screen with random x-axis position
        let enemy_translation_x_range = ARENA_WIDTH - enemy_width;
        let enemy_translation_x_min = -enemy_translation_x_range / 2.;
        let enemy_translation = Vec3::new(
            enemy_translation_x_min + rng.gen::<f32>() * enemy_translation_x_range,
            (ARENA_HEIGHT + enemy_height) / 2.,
            0.,
        );

        spawn_enemy_events.send(SpawnEnemyEvent {
            enemy_variant,
            enemy_translation,
        });
    }
}
