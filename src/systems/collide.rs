use crate::{
    components::{Enemy, HitBox, Laser, Ship, TimeToLive},
    constant::ANIMATION_INTERVAL,
    events::{CollisionLaserEnemyEvent, SpawnExplosionEvent},
    resource::EventReaders,
};
use bevy::prelude::*;

pub fn check_laser_enemy(
    mut collision_laser_enemy_events: ResMut<Events<CollisionLaserEnemyEvent>>,
    query_laser: Query<(Entity, &Laser, &HitBox, &Transform)>,
    query_enemy: Query<(Entity, &Enemy, &HitBox, &Transform)>,
    query_ship: Query<&Ship>,
) {
    for (laser_entity, laser, HitBox(laser_hit_box), laser_transform) in query_laser.iter() {
        if query_ship.get(laser.source).is_ok() {
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

pub fn handle_laser_enemy(
    mut commands: Commands,
    collision_laser_enemy_events: Res<Events<CollisionLaserEnemyEvent>>,
    mut spawn_explosion_events: ResMut<Events<SpawnExplosionEvent>>,
    mut event_readers: ResMut<EventReaders>,
    query_enemy: Query<&Transform>,
) {
    for evt in event_readers
        .collision_laser_enemy
        .iter(&collision_laser_enemy_events)
    {
        let enemy_transform = query_enemy
            .get(evt.enemy_entity)
            .expect("Could not get enemy transform component");
        spawn_explosion_events.send(SpawnExplosionEvent {
            explosion_translation: enemy_transform.translation,
            explosion_time_to_live: TimeToLive(Timer::new(ANIMATION_INTERVAL * 5, false)),
        });

        commands.despawn(evt.enemy_entity);
        commands.despawn(evt.laser_entity);
    }
}
