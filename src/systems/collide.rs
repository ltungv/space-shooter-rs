use crate::{
    components::{Enemy, HitBox, ShipLaser, TimeToLive},
    constant::ANIMATION_INTERVAL,
    events::{CollisionEnemyShipLaserEvent, SpawnExplosionEvent},
    resource::EventReaders,
};
use bevy::prelude::*;

pub fn check_collision_enemy_ship_laser(
    mut collision_enemy_laser_events: ResMut<Events<CollisionEnemyShipLaserEvent>>,
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
                collision_enemy_laser_events.send(CollisionEnemyShipLaserEvent {
                    enemy_entity,
                    ship_laser_entity,
                });
            }
        }
    }
}

pub fn handle_collision_enemy_ship_laser(
    mut commands: Commands,
    collision_enemy_laser_events: Res<Events<CollisionEnemyShipLaserEvent>>,
    mut spawn_explosion_events: ResMut<Events<SpawnExplosionEvent>>,
    mut event_readers: ResMut<EventReaders>,
    query_enemy: Query<&Transform>,
) {
    for evt in event_readers
        .collision_enemy_ship_laser
        .iter(&collision_enemy_laser_events)
    {
        let enemy_transform = query_enemy
            .get(evt.enemy_entity)
            .expect("Could not get enemy transform component");
        spawn_explosion_events.send(SpawnExplosionEvent {
            explosion_translation: enemy_transform.translation,
            explosion_time_to_live: TimeToLive(Timer::new(ANIMATION_INTERVAL * 5, false)),
        });

        commands.despawn(evt.enemy_entity);
        commands.despawn(evt.ship_laser_entity);
    }
}
