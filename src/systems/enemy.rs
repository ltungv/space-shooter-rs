use crate::{
    components::{Enemy, HitBox},
    events::SpawnLaserEvent,
};
use bevy::prelude::*;

pub fn fire_laser(
    time: Res<Time>,
    mut spawn_laser_events: ResMut<Events<SpawnLaserEvent>>,
    entity: Entity,
    transform: &Transform,
    HitBox(hit_box): &HitBox,
    mut enemy: Mut<Enemy>,
) {
    enemy.laser_cooldown_timer.tick(time.delta_seconds);
    if enemy.laser_cooldown_timer.finished {
        enemy.laser_cooldown_timer.reset();
        let laser_translation = transform.translation - hit_box.x() * Vec3::unit_y();
        spawn_laser_events.send(SpawnLaserEvent {
            laser_translation,
            laser_source: entity,
        })
    }
}
