use crate::components::TimeToLive;
use bevy::prelude::*;

pub fn despawn_expired(
    mut commands: Commands,
    time: Res<Time>,
    entity: Entity,
    mut ttl: Mut<TimeToLive>,
) {
    ttl.0.tick(time.delta_seconds);
    if ttl.0.finished {
        commands.despawn(entity);
    }
}
