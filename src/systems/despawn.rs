use crate::{
    components::{Enemy, HitBox, TimeToLive},
    constant::{ARENA_HEIGHT, ARENA_WIDTH},
    event::EntityDespawnEvent,
    resource::GameState,
};
use bevy::prelude::*;

pub fn despawn_events_listener(
    mut commands: Commands,
    entity_despawn_events: Res<Events<EntityDespawnEvent>>,
    mut game_state: ResMut<GameState>,
) {
    for event in game_state
        .entity_despawn_event_reader
        .iter(&entity_despawn_events)
    {
        commands.despawn(event.entity);
    }
}

pub fn despawn_enemy_out_of_bound(
    mut entity_despawn_events: ResMut<Events<EntityDespawnEvent>>,
    entity: Entity,
    _enemy: &Enemy,
    HitBox(hit_box): &HitBox,
    transform: Mut<Transform>,
) {
    if transform.translation.y() + hit_box.y() / 2. <= -ARENA_HEIGHT / 2.
        || transform.translation.x() + hit_box.x() / 2. <= -ARENA_WIDTH / 2.
        || transform.translation.x() - hit_box.x() / 2. >= ARENA_WIDTH / 2.
    {
        entity_despawn_events.send(EntityDespawnEvent { entity });
    }
}

pub fn despawn_expired_time_to_live(
    mut entity_despawn_events: ResMut<Events<EntityDespawnEvent>>,
    time: Res<Time>,
    entity: Entity,
    mut ttl: Mut<TimeToLive>,
) {
    ttl.0.tick(time.delta_seconds);
    if ttl.0.finished {
        entity_despawn_events.send(EntityDespawnEvent { entity });
    }
}
