use crate::{
    constant::ANIMATION_INTERVAL,
    components::{TimeToLive, Enemy, HitBox, ShipLaser},
    events::{EnemyShipLaserCollisionEvent, ExplosionSpawnEvent},
    resource::EventReaders,
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

pub fn enemy_ship_laser_collision_event_listener(
    mut commands: Commands,
    enemy_ship_laser_collision_events: Res<Events<EnemyShipLaserCollisionEvent>>,
    mut explosion_spawn_events: ResMut<Events<ExplosionSpawnEvent>>,
    mut event_readers: ResMut<EventReaders>,
    query_enemy: Query<&Transform>,
) {
    for enemy_ship_laser_collision_event in event_readers
        .enemy_ship_laser_collision
        .iter(&enemy_ship_laser_collision_events)
    {
        let enemy_transform = query_enemy
            .get(enemy_ship_laser_collision_event.enemy_entity)
            .expect("Could not get enemy transform component");
        explosion_spawn_events.send(ExplosionSpawnEvent {
            explosion_translation: enemy_transform.translation,
            explosion_time_to_live: TimeToLive(Timer::new(ANIMATION_INTERVAL * 5, false)),
        });

        commands.despawn(enemy_ship_laser_collision_event.enemy_entity);
        commands.despawn(enemy_ship_laser_collision_event.ship_laser_entity);
    }
}
