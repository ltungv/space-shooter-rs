use crate::components::{EnemyVariant, TimeToLive};
use bevy::prelude::*;

pub struct ShipLaserSpawnEvent {
    pub laser_translation: Vec3,
}

pub struct EnemySpawnEvent {
    pub enemy_variant: EnemyVariant,
    pub enemy_translation: Vec3,
}

pub struct ExplosionSpawnEvent {
    pub explosion_translation: Vec3,
    pub explosion_time_to_live: TimeToLive,
}

pub struct EnemyShipLaserCollisionEvent {
    pub enemy_entity: Entity,
    pub ship_laser_entity: Entity,
}
