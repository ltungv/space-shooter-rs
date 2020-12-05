use crate::components::{EnemyVariant, TimeToLive};
use bevy::prelude::*;

pub struct SpawnShipLaserEvent {
    pub laser_translation: Vec3,
}

pub struct SpawnEnemyEvent {
    pub enemy_variant: EnemyVariant,
    pub enemy_translation: Vec3,
}

pub struct SpawnExplosionEvent {
    pub explosion_translation: Vec3,
    pub explosion_time_to_live: TimeToLive,
}

pub struct CollisionEnemyShipLaserEvent {
    pub enemy_entity: Entity,
    pub ship_laser_entity: Entity,
}
