use crate::{
    components::{EnemySpawner, EnemyVariant},
    constant::{
        ENEMY_SPAWN_INTERVAL, SPAWN_WEIGHT_ENEMY_BIG, SPAWN_WEIGHT_ENEMY_MEDIUM,
        SPAWN_WEIGHT_ENEMY_SMALL,
    },
};
use bevy::prelude::*;

#[derive(Bundle)]
pub struct EnemySpawnerComponents {
    pub enemy_spawner: EnemySpawner,
}

/// Create a new enemy spawner
pub fn initialize_enemy_spawner(mut commands: Commands) {
    commands.spawn(EnemySpawnerComponents {
        enemy_spawner: EnemySpawner {
            timer: Timer::new(ENEMY_SPAWN_INTERVAL, true),
            weights: vec![
                (EnemyVariant::Small, SPAWN_WEIGHT_ENEMY_SMALL),
                (EnemyVariant::Medium, SPAWN_WEIGHT_ENEMY_MEDIUM),
                (EnemyVariant::Big, SPAWN_WEIGHT_ENEMY_BIG),
            ],
        },
    });
}
