use crate::components::EnemyVariant;
use bevy::prelude::*;

pub struct EnemyShipLaserCollisionEvent {
    pub enemy_entity: Entity,
    pub ship_laser_entity: Entity,
}

pub struct ShipLaserSpawnEvent {
    pub ship_translation: Vec3,
}

pub struct EnemySpawnEvent {
    pub enemy_variant: EnemyVariant,
    pub enemy_translation: Vec3,
}

pub struct EnemyDestroyedEvent {
    pub enemy_entity: Entity,
}
