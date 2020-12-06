use crate::{components::Weapon, events::SpawnLaserEvent};
use bevy::prelude::*;

pub fn fire_laser(
    time: Res<Time>,
    mut spawn_laser_events: ResMut<Events<SpawnLaserEvent>>,
    weapon_parent: &Parent,
    weapon_global_transform: &GlobalTransform,
    mut weapon: Mut<Weapon>,
) {
    weapon.cooldown_timer.tick(time.delta_seconds);
    if weapon.cooldown_timer.finished {
        weapon.cooldown_timer.reset();
        spawn_laser_events.send(SpawnLaserEvent {
            laser_translation: weapon_global_transform.translation,
            laser_source: weapon_parent.0,
            laser_velocity: weapon.laser_velocity.clone(),
            laser_hit_box: weapon.laser_hit_box.clone(),
            laser_time_to_live: weapon.laser_time_to_live.clone(),
            laser_initial_sprite_idx: weapon.laser_initial_sprite_idx,
        })
    }
}
