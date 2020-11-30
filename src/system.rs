use crate::{
    component::{
        Animation, Enemy, EnemySpawner, EnemyVariant, HitBox, Ship, ShipAnimationState, ShipLaser,
        TimeToLive, Velocity,
    },
    constant::{
        ANIMATION_INTERVAL, ARENA_HEIGHT, ARENA_WIDTH, ENEMY_BIG_SPRITE_HEIGHT,
        ENEMY_BIG_SPRITE_WIDTH, ENEMY_INITIAL_VELOCITY, ENEMY_MEDIUM_SPRITE_HEIGHT,
        ENEMY_MEDIUM_SPRITE_WIDTH, ENEMY_SMALL_SPRITE_HEIGHT, ENEMY_SMALL_SPRITE_WIDTH,
        LASER_SPRITE_WIDTH, SHIP_LASER_INITIAL_VELOCITY, SHIP_LASER_SPRITE_HEIGHT,
        SHIP_LASER_TIME_TO_LIVE_DURATION, SPRITE_SCALING_FACTOR,
    },
    resource::GameState,
};
use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
};
use rand::prelude::*;

// TODO: implement acceleration

/// Change ship's position based on the moving speed and moving direction. Movement is limited
/// to the window viewable area
#[allow(clippy::too_many_arguments)]
pub fn entities_movement(
    // Resources
    time: Res<Time>,
    velocity: &Velocity,
    mut transform: Mut<Transform>,
) {
    *transform.translation.x_mut() += time.delta_seconds * velocity.0.x();
    *transform.translation.y_mut() += time.delta_seconds * velocity.0.y();
}

pub fn enemies_despawner(
    mut commands: Commands,
    entity: Entity,
    _enemy: &Enemy,
    hit_box: &HitBox,
    transform: Mut<Transform>,
) {
    if transform.translation.y() + hit_box.height / 2. <= -ARENA_HEIGHT / 2.
        || transform.translation.x() + hit_box.width / 2. <= -ARENA_WIDTH / 2.
        || transform.translation.x() - hit_box.width / 2. >= ARENA_WIDTH / 2.
    {
        commands.despawn(entity);
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
                let hit_box = HitBox {
                    width: ENEMY_SMALL_SPRITE_WIDTH * SPRITE_SCALING_FACTOR,
                    height: ENEMY_SMALL_SPRITE_HEIGHT * SPRITE_SCALING_FACTOR,
                };

                let texture_atlas_handle = game_state
                    .texture_atlas_handles
                    .get("enemy-small")
                    .expect("Could not get small enemy texture atlas handle");

                Some((EnemyVariant::Small, hit_box, texture_atlas_handle))
            }
            "medium" => {
                let hit_box = HitBox {
                    width: ENEMY_MEDIUM_SPRITE_WIDTH * SPRITE_SCALING_FACTOR,
                    height: ENEMY_MEDIUM_SPRITE_HEIGHT * SPRITE_SCALING_FACTOR,
                };

                let texture_atlas_handle = game_state
                    .texture_atlas_handles
                    .get("enemy-medium")
                    .expect("Could not get medium enemy texture atlas handle");

                Some((EnemyVariant::Medium, hit_box, texture_atlas_handle))
            }
            "big" => {
                let hit_box = HitBox {
                    width: ENEMY_BIG_SPRITE_WIDTH * SPRITE_SCALING_FACTOR,
                    height: ENEMY_BIG_SPRITE_HEIGHT * SPRITE_SCALING_FACTOR,
                };

                let texture_atlas_handle = game_state
                    .texture_atlas_handles
                    .get("enemy-big")
                    .expect("Could not get big enemy texture atlas handle");

                Some((EnemyVariant::Big, hit_box, texture_atlas_handle))
            }
            _ => panic!("Unknown enemy type name"),
        };

        if let Some((variant, hit_box, texture_atlas_handle)) = enemy_data {
            // Enemy comes from the top of the screen with random x-axis position
            let max_offset_x_from_center = (ARENA_WIDTH - hit_box.width) / 2.;
            let rand_translation_x_range = rng.gen::<f32>() * 2. * max_offset_x_from_center;

            let translation_x = -max_offset_x_from_center + rand_translation_x_range;
            let translation_y = (ARENA_HEIGHT + hit_box.height) / 2.;
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
                .with(hit_box)
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

/// Change ship's position based on the moving speed and moving direction. Movement is limited
/// to the window viewable area
#[allow(clippy::too_many_arguments)]
pub fn ship_translation_clip(_ship: &Ship, hit_box: &HitBox, mut transform: Mut<Transform>) {
    // X-axis movement
    let max_offset_x_from_center = (ARENA_WIDTH - hit_box.width) / 2.;
    *transform.translation.x_mut() = transform
        .translation
        .x()
        .min(max_offset_x_from_center)
        .max(-max_offset_x_from_center);

    // Y-axis movement
    let max_offset_y_from_center = (ARENA_HEIGHT - hit_box.height) / 2.;
    *transform.translation.y_mut() = transform
        .translation
        .y()
        .min(max_offset_y_from_center)
        .max(-max_offset_y_from_center);
}

/// Change ship's directions based on user's keyboard input
pub fn keyboard_control_ship(
    keyboard_input: Res<Input<KeyCode>>,
    ship: &Ship,
    mut velocity: Mut<Velocity>,
) {
    let mut x_direction = 0.;
    if keyboard_input.pressed(KeyCode::Left) {
        x_direction -= 1.;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        x_direction += 1.;
    }

    let mut y_direction = 0.;
    if keyboard_input.pressed(KeyCode::Up) {
        y_direction += 1.;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        y_direction -= 1.;
    }

    // Ensure ship speed is capped at `max_speed` when moving diagonally
    if x_direction != 0. && y_direction != 0. {
        *velocity.0.y_mut() = (ship.move_speed / f32::sqrt(2.)) * y_direction;
        *velocity.0.x_mut() = (ship.move_speed / f32::sqrt(2.)) * x_direction;
    } else {
        *velocity.0.y_mut() = ship.move_speed * y_direction;
        *velocity.0.x_mut() = ship.move_speed * x_direction;
    }
}

pub fn keyboard_fire_laser(
    mut commands: Commands,
    time: Res<Time>,
    kb_input: Res<Input<KeyCode>>,
    game_state: Res<GameState>,
    transform: &Transform,
    hit_box: &HitBox,
    mut ship: Mut<Ship>,
) {
    ship.laser_cooldown_timer.tick(time.delta_seconds);
    if kb_input.pressed(KeyCode::Space) && ship.laser_cooldown_timer.finished {
        ship.laser_cooldown_timer.reset();
        if let Some(texture_atlas_handle) = game_state.texture_atlas_handles.get("laser-bolts") {
            let translation = transform.translation + hit_box.height * Vec3::unit_y();
            commands
                .spawn(SpriteSheetComponents {
                    texture_atlas: texture_atlas_handle.clone(),
                    transform: Transform {
                        translation,
                        scale: Vec3::splat(SPRITE_SCALING_FACTOR),
                        ..Default::default()
                    },
                    sprite: TextureAtlasSprite::new(1),
                    ..Default::default()
                })
                .with(ShipLaser)
                .with(TimeToLive(Timer::new(
                    SHIP_LASER_TIME_TO_LIVE_DURATION,
                    false,
                )))
                .with(HitBox {
                    width: LASER_SPRITE_WIDTH * SPRITE_SCALING_FACTOR,
                    height: SHIP_LASER_SPRITE_HEIGHT * SPRITE_SCALING_FACTOR,
                })
                .with(Velocity(Vec2::new(
                    SHIP_LASER_INITIAL_VELOCITY.0,
                    SHIP_LASER_INITIAL_VELOCITY.1,
                )))
                .with(Animation {
                    idx_delta: 2,
                    sprite_count: 4,
                    timer: Timer::new(ANIMATION_INTERVAL, true),
                });
        }
    }
}

pub fn entities_time_to_live(
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

/// Change the ship's animation state and change the current index to the index of the sprite
/// that represents that state. The ship has to be in the new state for at least some set amount
/// of duration before being able to change its state again
pub fn ship_state_transition(
    time: Res<Time>,
    velocity: &Velocity,
    mut ship: Mut<Ship>,
    mut sprite: Mut<TextureAtlasSprite>,
) {
    ship.transition_timer.tick(time.delta_seconds);
    if ship.transition_timer.finished {
        let x_velocity = velocity.0.x();
        let new_animation_state = if x_velocity < 0. {
            match ship.animation_state {
                ShipAnimationState::Stabilized => ShipAnimationState::HalfLeft,
                ShipAnimationState::HalfRight => ShipAnimationState::Stabilized,
                ShipAnimationState::FullRight => ShipAnimationState::HalfRight,
                ShipAnimationState::HalfLeft | ShipAnimationState::FullLeft => {
                    ShipAnimationState::FullLeft
                }
            }
        } else if x_velocity > 0. {
            match ship.animation_state {
                ShipAnimationState::Stabilized => ShipAnimationState::HalfRight,
                ShipAnimationState::HalfLeft => ShipAnimationState::Stabilized,
                ShipAnimationState::FullLeft => ShipAnimationState::HalfLeft,
                ShipAnimationState::HalfRight | ShipAnimationState::FullRight => {
                    ShipAnimationState::FullRight
                }
            }
        } else {
            match ship.animation_state {
                ShipAnimationState::FullLeft => ShipAnimationState::HalfLeft,
                ShipAnimationState::FullRight => ShipAnimationState::HalfRight,
                ShipAnimationState::Stabilized
                | ShipAnimationState::HalfRight
                | ShipAnimationState::HalfLeft => ShipAnimationState::Stabilized,
            }
        };

        // Updates if state is changed
        if new_animation_state != ship.animation_state {
            ship.transition_timer.reset();
            ship.animation_state = new_animation_state;
            sprite.index = match ship.animation_state {
                ShipAnimationState::FullLeft => 0,
                ShipAnimationState::HalfLeft => 1,
                ShipAnimationState::Stabilized => 2,
                ShipAnimationState::HalfRight => 3,
                ShipAnimationState::FullRight => 4,
            };
        }
    }
}

/// Periodically change the index to the sprite in the spritesheet
pub fn entities_animation(
    time: Res<Time>,
    mut sprite: Mut<TextureAtlasSprite>,
    mut animation: Mut<Animation>,
) {
    animation.timer.tick(time.delta_seconds);
    if animation.timer.finished {
        sprite.index = (sprite.index + animation.idx_delta) % animation.sprite_count;
    }
}
