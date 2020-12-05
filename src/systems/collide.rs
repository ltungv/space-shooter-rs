use crate::{
    components::{Enemy, HitBox, ShipLaser},
    events::EnemyShipLaserCollisionEvent,
};
use bevy::prelude::*;

pub fn enemy_with_laser(
    mut enemy_laser_collision_events: ResMut<Events<EnemyShipLaserCollisionEvent>>,
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
                enemy_laser_collision_events.send(EnemyShipLaserCollisionEvent {
                    enemy_entity,
                    ship_laser_entity,
                });
            }
        }
    }
}
