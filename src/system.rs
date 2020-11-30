use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
};
use rand::prelude::*;

use crate::{
    component::{Animation, Enemy, EnemySpawner, HitBox, Motion, Ship, ShipAnimationState},
    constant::{ARENA_HEIGHT, ARENA_WIDTH, SPRITE_SCALING_FACTOR},
    entity,
    resource::GameState,
};

// TODO: implement acceleration

/// Change ship's position based on the moving speed and moving direction. Movement is limited
/// to the window viewable area
#[allow(clippy::too_many_arguments)]
pub fn enemies_movement(
    // Resources
    time: Res<Time>,
    _enemy: &Enemy,
    motion: &Motion,
    mut transform: Mut<Transform>,
) {
    *transform.translation.x_mut() += time.delta_seconds * motion.velocity.x();
    *transform.translation.y_mut() += time.delta_seconds * motion.velocity.y();
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

        if let Some(enemy_data) = game_state.enemy_data.get(variant_name) {
            // Enemy comes from the top of the screen with random x-axis position
            let scaled_texture_size = enemy_data.texture_size * SPRITE_SCALING_FACTOR;
            let x_offset = (ARENA_WIDTH - scaled_texture_size.x()) / 2.;
            let translation_x = -x_offset + rng.gen::<f32>() * (2. * x_offset);
            let translation_y = (ARENA_HEIGHT + scaled_texture_size.y()) / 2.;

            entity::create_enemy(
                &mut commands,
                Vec3::new(translation_x, translation_y, 0.),
                enemy_data.clone(),
            );
        }
    }
}

/// Change ship's position based on the moving speed and moving direction. Movement is limited
/// to the window viewable area
#[allow(clippy::too_many_arguments)]
pub fn ship_movement(
    // Resources
    time: Res<Time>,
    _ship: &Ship,
    motion: &Motion,
    hit_box: &HitBox,
    mut transform: Mut<Transform>,
) {
    // X-axis movement
    let max_offset_x_from_center = (ARENA_WIDTH - hit_box.width) / 2.;
    *transform.translation.x_mut() += time.delta_seconds * motion.velocity.x();
    *transform.translation.x_mut() = transform
        .translation
        .x()
        .min(max_offset_x_from_center)
        .max(-max_offset_x_from_center);

    // Y-axis movement
    let max_offset_y_from_center = (ARENA_HEIGHT - hit_box.height) / 2.;
    *transform.translation.y_mut() += time.delta_seconds * motion.velocity.y();
    *transform.translation.y_mut() = transform
        .translation
        .y()
        .min(max_offset_y_from_center)
        .max(-max_offset_y_from_center);
}

/// Change ship's directions based on user's keyboard input
pub fn ship_control(kb_input: Res<Input<KeyCode>>, _ship: &Ship, mut motion: Mut<Motion>) {
    let mut x_direction = 0.;
    if kb_input.pressed(KeyCode::Left) {
        x_direction -= 1.;
    }
    if kb_input.pressed(KeyCode::Right) {
        x_direction += 1.;
    }

    let mut y_direction = 0.;
    if kb_input.pressed(KeyCode::Up) {
        y_direction += 1.;
    }
    if kb_input.pressed(KeyCode::Down) {
        y_direction -= 1.;
    }

    // Ensure ship speed is capped at `max_speed` when moving diagonally
    if x_direction != 0. && y_direction != 0. {
        *motion.velocity.y_mut() = (motion.max_speed / f32::sqrt(2.)) * y_direction;
        *motion.velocity.x_mut() = (motion.max_speed / f32::sqrt(2.)) * x_direction;
    } else {
        *motion.velocity.y_mut() = motion.max_speed * y_direction;
        *motion.velocity.x_mut() = motion.max_speed * x_direction;
    }
}

/// Change the ship's animation state and change the current index to the index of the sprite
/// that represents that state. The ship has to be in the new state for at least some set amount
/// of duration before being able to change its state again
pub fn ship_state_transition(
    time: Res<Time>,
    mut ship: Mut<Ship>,
    motion: &Motion,
    mut sprite: Mut<TextureAtlasSprite>,
) {
    // State is not changed rapidly so that animation can be perceived by the ship
    if let Some(now) = time.instant {
        if now.duration_since(ship.transition_instant) >= ship.transition_duration {
            // Determines the new state based on previous state and current moving direction
            let x_velocity = motion.velocity.x();
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
                ship.transition_instant = now;
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
