use crate::{
    components::{Enemy, HitBox, Laser, Ship},
    constant::ANIMATION_INTERVAL,
    events::{CollisionLaserEnemyEvent, CollisionLaserShipEvent, SpawnExplosionEvent},
    resource::EventReaders,
};
use bevy::{app::AppExit, prelude::*};

pub fn check_laser(
    mut collision_laser_ship_events: ResMut<Events<CollisionLaserShipEvent>>,
    mut collision_laser_enemy_events: ResMut<Events<CollisionLaserEnemyEvent>>,
    query_laser: Query<(Entity, &Laser, &HitBox, &Transform)>,
    query_ship: Query<(Entity, &Ship, &HitBox, &Transform)>,
    query_enemy: Query<(Entity, &Enemy, &HitBox, &Transform)>,
) {
    for (laser_entity, laser, HitBox(laser_hit_box), laser_transform) in query_laser.iter() {
        if query_ship.get(laser.source).is_err() {
            for (ship_entity, _ship, HitBox(ship_hit_box), ship_transform) in query_ship.iter() {
                if bevy::sprite::collide_aabb::collide(
                    laser_transform.translation,
                    *laser_hit_box,
                    ship_transform.translation,
                    *ship_hit_box,
                )
                .is_some()
                {
                    collision_laser_ship_events.send(CollisionLaserShipEvent {
                        laser_entity,
                        ship_entity,
                    });
                }
            }
        }

        if query_enemy.get(laser.source).is_err() {
            for (enemy_entity, _enemy, HitBox(enemy_hit_box), enemy_transform) in query_enemy.iter()
            {
                if bevy::sprite::collide_aabb::collide(
                    laser_transform.translation,
                    *laser_hit_box,
                    enemy_transform.translation,
                    *enemy_hit_box,
                )
                .is_some()
                {
                    collision_laser_enemy_events.send(CollisionLaserEnemyEvent {
                        laser_entity,
                        enemy_entity,
                    });
                }
            }
        }
    }
}

pub fn handle_laser_ship(
    mut commands: Commands,
    collision_laser_ship_events: Res<Events<CollisionLaserShipEvent>>,
    mut app_exit_events: ResMut<Events<AppExit>>,
    mut spawn_explosion_events: ResMut<Events<SpawnExplosionEvent>>,
    mut event_readers: ResMut<EventReaders>,
    query_ship_transform: Query<&Transform>,
) {
    for evt in event_readers
        .collision_laser_ship
        .iter(&collision_laser_ship_events)
    {
        let ship_transform = query_ship_transform
            .get(evt.ship_entity)
            .expect("Could not get ship transform component");
        spawn_explosion_events.send(SpawnExplosionEvent {
            explosion_translation: ship_transform.translation,
            explosion_time_to_live_duration: ANIMATION_INTERVAL * 5,
        });

        commands.despawn_recursive(evt.ship_entity);
        commands.despawn(evt.laser_entity);
        app_exit_events.send(AppExit);
    }
}

pub fn handle_laser_enemy(
    mut commands: Commands,
    collision_laser_enemy_events: Res<Events<CollisionLaserEnemyEvent>>,
    mut spawn_explosion_events: ResMut<Events<SpawnExplosionEvent>>,
    mut event_readers: ResMut<EventReaders>,
    query_enemy_transform: Query<&Transform>,
) {
    for evt in event_readers
        .collision_laser_enemy
        .iter(&collision_laser_enemy_events)
    {
        let enemy_transform = query_enemy_transform
            .get(evt.enemy_entity)
            .expect("Could not get enemy transform component");
        spawn_explosion_events.send(SpawnExplosionEvent {
            explosion_translation: enemy_transform.translation,
            explosion_time_to_live_duration: ANIMATION_INTERVAL * 5,
        });

        commands.despawn_recursive(evt.enemy_entity);
        commands.despawn(evt.laser_entity);
    }
}
