use crate::{
    component::{Animation, Enemy, EnemySpawner, EnemyVariant, HitBox, Velocity},
    constant::{
        ANIMATION_INTERVAL, ARENA_HEIGHT, ARENA_WIDTH, ENEMY_BIG_SPRITE_HEIGHT,
        ENEMY_BIG_SPRITE_WIDTH, ENEMY_INITIAL_VELOCITY, ENEMY_MEDIUM_SPRITE_HEIGHT,
        ENEMY_MEDIUM_SPRITE_WIDTH, ENEMY_SMALL_SPRITE_HEIGHT, ENEMY_SMALL_SPRITE_WIDTH,
        SPRITE_SCALING_FACTOR,
    },
    event::EntityDespawnEvent,
    resource::GameState,
};
use bevy::prelude::*;
use rand::prelude::*;

pub fn enemies_despawner(
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

/// Go through all enemy spawners and check if they ready to spawn new entity,
/// create entity as the spawn timer finishes.
pub fn enemies_spawner(
    mut commands: Commands,
    time: Res<Time>,
    game_state: Res<GameState>,
    mut spawner: Mut<EnemySpawner>,
) {
    spawner.timer.tick(time.delta_seconds);
    if spawner.timer.just_finished {
        // Choose the name of the enemy to be spawned
        let mut rng = rand::thread_rng();
        let variant_name = &spawner
            .weights
            .choose_weighted(&mut rng, |item| item.1)
            .expect("Could not choose spawnable")
            .0;

        let enemy_data = match variant_name.as_str() {
            "small" => {
                let hit_box = HitBox(Vec2::new(
                    ENEMY_SMALL_SPRITE_WIDTH * SPRITE_SCALING_FACTOR,
                    ENEMY_SMALL_SPRITE_HEIGHT * SPRITE_SCALING_FACTOR,
                ));

                let texture_atlas_handle = game_state
                    .texture_atlas_handles
                    .get("enemy-small")
                    .expect("Could not get small enemy texture atlas handle");

                Some((EnemyVariant::Small, hit_box, texture_atlas_handle))
            }
            "medium" => {
                let hit_box = HitBox(Vec2::new(
                    ENEMY_MEDIUM_SPRITE_WIDTH * SPRITE_SCALING_FACTOR,
                    ENEMY_MEDIUM_SPRITE_HEIGHT * SPRITE_SCALING_FACTOR,
                ));

                let texture_atlas_handle = game_state
                    .texture_atlas_handles
                    .get("enemy-medium")
                    .expect("Could not get medium enemy texture atlas handle");

                Some((EnemyVariant::Medium, hit_box, texture_atlas_handle))
            }
            "big" => {
                let hit_box = HitBox(Vec2::new(
                    ENEMY_BIG_SPRITE_WIDTH * SPRITE_SCALING_FACTOR,
                    ENEMY_BIG_SPRITE_HEIGHT * SPRITE_SCALING_FACTOR,
                ));

                let texture_atlas_handle = game_state
                    .texture_atlas_handles
                    .get("enemy-big")
                    .expect("Could not get big enemy texture atlas handle");

                Some((EnemyVariant::Big, hit_box, texture_atlas_handle))
            }
            _ => panic!("Unknown enemy type name"),
        };

        if let Some((variant, HitBox(hit_box), texture_atlas_handle)) = enemy_data {
            // Enemy comes from the top of the screen with random x-axis position
            let max_offset_x_from_center = (ARENA_WIDTH - hit_box.x()) / 2.;
            let rand_translation_x_range = rng.gen::<f32>() * 2. * max_offset_x_from_center;

            let translation_x = -max_offset_x_from_center + rand_translation_x_range;
            let translation_y = (ARENA_HEIGHT + hit_box.y()) / 2.;
            let translation = Vec3::new(translation_x, translation_y, 0.);

            commands
                .spawn(SpriteSheetComponents {
                    texture_atlas: texture_atlas_handle.clone(),
                    transform: Transform {
                        scale: Vec3::splat(SPRITE_SCALING_FACTOR),
                        translation,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with(Enemy { variant })
                .with(HitBox(hit_box))
                .with(Velocity(Vec2::new(
                    ENEMY_INITIAL_VELOCITY.0,
                    ENEMY_INITIAL_VELOCITY.1,
                )))
                .with(Animation {
                    idx_delta: 1,
                    sprite_count: 2,
                    timer: Timer::new(ANIMATION_INTERVAL, true),
                });
        }
    }
}
