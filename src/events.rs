use crate::components::{EnemyVariant, TimeToLive};
use bevy::prelude::*;

pub struct SpawnLaserEvent {
    pub laser_translation: Vec3,
    pub laser_source: Entity,
}

pub struct SpawnEnemyEvent {
    pub enemy_variant: EnemyVariant,
    pub enemy_translation: Vec3,
}

pub struct SpawnExplosionEvent {
    pub explosion_translation: Vec3,
    pub explosion_time_to_live: TimeToLive,
}

pub struct CollisionLaserEnemyEvent {
    pub laser_entity: Entity,
    pub enemy_entity: Entity,
}

pub struct CollisionLaserShipEvent {
    pub laser_entity: Entity,
    pub ship_entity: Entity,
}
