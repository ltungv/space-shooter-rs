use crate::component::{Enemy, HitBox, ShipLaser};
use crate::event::EntityDespawnEvent;
use bevy::prelude::*;

pub fn collide_laser_enemies(
    mut entity_despawn_events: ResMut<Events<EntityDespawnEvent>>,
    ship_lasers_query: Query<(Entity, &ShipLaser, &HitBox, &Transform)>,
    enemies_query: Query<(Entity, &Enemy, &HitBox, &Transform)>,
) {
    for (ship_laser_entity, _ship_laser, HitBox(ship_laser_hit_box), ship_laser_transform) in
        ship_lasers_query.iter()
    {
        for (enemy_entity, _enemy, HitBox(enemy_hit_box), enemy_transform) in enemies_query.iter() {
            if bevy::sprite::collide_aabb::collide(
                ship_laser_transform.translation,
                *ship_laser_hit_box,
                enemy_transform.translation,
                *enemy_hit_box,
            )
            .is_some()
            {
                entity_despawn_events.send(EntityDespawnEvent {
                    entity: enemy_entity,
                });
                entity_despawn_events.send(EntityDespawnEvent {
                    entity: ship_laser_entity,
                });
            }
        }
    }
}
