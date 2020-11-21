use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
};

use std::collections::HashMap;
use std::time::{Duration, Instant};

const WINDOW_WIDTH: f32 = 600.;
const WINDOW_HEIGHT: f32 = 800.;

const PLAYER_SPRITE_HEIGHT: f32 = 24.;
const PLAYER_SPEED: f32 = 500.;
const PLAYER_SPRITE_WIDTH: f32 = 16.;

const STABILIZATION_DELAY: Duration = Duration::from_millis(100);
const ANIMATION_INTERVAL: Duration = Duration::from_millis(200);

fn main() {
    App::build()
        .init_resource::<TextureAtlasHandles>()
        .add_resource(WindowDescriptor {
            title: "Space shooter!".to_string(),
            width: WINDOW_WIDTH as u32,
            height: WINDOW_HEIGHT as u32,
            resizable: false,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(player_direction.system())
        .add_system(player_movement.system())
        .add_system(player_state_transition.system())
        .add_system(entities_animation.system())
        .run();
}

#[derive(Default)]
struct TextureAtlasHandles(HashMap<String, Handle<TextureAtlas>>);

#[derive(Debug, PartialEq, Default)]
struct MoveDirection(Vec2);

#[derive(Debug)]
struct MoveSpeed(f32);

#[derive(Debug, Default)]
struct Player;

#[derive(Debug, Default)]
struct Enemy;

#[derive(Debug, PartialEq)]
enum PlayerAnimationState {
    FullLeft,
    HalfLeft,
    Stabilized,
    HalfRight,
    FullRight,
}

impl PlayerAnimationState {
    fn transition_left(&self) -> Self {
        match self {
            Self::FullLeft => Self::FullLeft,
            Self::HalfLeft => Self::FullLeft,
            Self::Stabilized => Self::HalfLeft,
            Self::HalfRight => Self::Stabilized,
            Self::FullRight => Self::HalfRight,
        }
    }

    fn transition_right(&self) -> Self {
        match self {
            Self::FullLeft => Self::HalfLeft,
            Self::HalfLeft => Self::Stabilized,
            Self::Stabilized => Self::HalfRight,
            Self::HalfRight => Self::FullRight,
            Self::FullRight => Self::FullRight,
        }
    }

    fn transition_stable(&self) -> Self {
        match self {
            Self::FullLeft => Self::HalfLeft,
            Self::HalfLeft => Self::Stabilized,
            Self::Stabilized => Self::Stabilized,
            Self::HalfRight => Self::Stabilized,
            Self::FullRight => Self::HalfRight,
        }
    }
}

#[derive(Debug)]
struct Animatable {
    sprite_cycle_delta: usize,
    cycle_timer: Timer,
}

#[derive(Debug)]
struct LastStateTransitionInstant(Instant);

impl Default for LastStateTransitionInstant {
    fn default() -> Self {
        Self(Instant::now())
    }
}

fn player_movement(
    time: Res<Time>,
    _player: &Player,
    MoveSpeed(move_speed): &MoveSpeed,
    MoveDirection(move_direction): &MoveDirection,
    mut transform: Mut<Transform>,
) {
    // Get size of the player's sprite on screen
    let width = PLAYER_SPRITE_WIDTH * transform.scale.x();
    let height = PLAYER_SPRITE_HEIGHT * transform.scale.y();

    // X-axis movement
    *transform.translation.x_mut() += time.delta_seconds * move_direction.x() * move_speed;
    *transform.translation.x_mut() = transform
        .translation
        .x()
        // update bound
        .min((WINDOW_WIDTH - width) / 2.)
        // lower bound
        .max(-(WINDOW_WIDTH - width) / 2.);

    // Y-axis movement
    *transform.translation.y_mut() += time.delta_seconds * move_direction.y() * move_speed;
    *transform.translation.y_mut() = transform
        .translation
        .y()
        // upper bound
        .min((WINDOW_HEIGHT - height) / 2.)
        // lower bound
        .max(-(WINDOW_HEIGHT - height) / 2.);
}

fn player_direction(
    kb_input: Res<Input<KeyCode>>,
    _player: &Player,
    mut move_direction: Mut<MoveDirection>,
) {
    *move_direction.0.y_mut() = 0.;
    *move_direction.0.x_mut() = 0.;

    if kb_input.pressed(KeyCode::Up) {
        *move_direction.0.y_mut() += 1.;
    }
    if kb_input.pressed(KeyCode::Down) {
        *move_direction.0.y_mut() -= 1.;
    }
    if kb_input.pressed(KeyCode::Left) {
        *move_direction.0.x_mut() -= 1.;
    }
    if kb_input.pressed(KeyCode::Right) {
        *move_direction.0.x_mut() += 1.;
    }
}

#[allow(clippy::too_many_arguments)]
fn entities_animation(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    texture_atlas_handle: &Handle<TextureAtlas>,
    mut sprite: Mut<TextureAtlasSprite>,
    mut animatable: Mut<Animatable>,
) {
    animatable.cycle_timer.tick(time.delta_seconds);
    if animatable.cycle_timer.finished {
        let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
        sprite.index = ((sprite.index as usize + animatable.sprite_cycle_delta)
            % texture_atlas.textures.len()) as u32;
    }
}

#[allow(clippy::too_many_arguments)]
fn player_state_transition(
    time: Res<Time>,
    _player: &Player,
    MoveDirection(move_direction): &MoveDirection,
    mut sprite: Mut<TextureAtlasSprite>,
    mut animation_state: Mut<PlayerAnimationState>,
    mut last_state_transition_instant: Mut<LastStateTransitionInstant>,
) {
    if let Some(now) = time.instant {
        if now.duration_since(last_state_transition_instant.0) >= STABILIZATION_DELAY {
            let x_direction = move_direction.x();
            let new_animation_state = if x_direction < 0. {
                animation_state.transition_left()
            } else if x_direction > 0. {
                animation_state.transition_right()
            } else {
                animation_state.transition_stable()
            };

            if new_animation_state != *animation_state {
                last_state_transition_instant.0 = now;
                *animation_state = new_animation_state;
                match *animation_state {
                    PlayerAnimationState::FullLeft => sprite.index = 0,
                    PlayerAnimationState::HalfLeft => sprite.index = 1,
                    PlayerAnimationState::Stabilized => sprite.index = 2,
                    PlayerAnimationState::HalfRight => sprite.index = 3,
                    PlayerAnimationState::FullRight => sprite.index = 4,
                }
            }
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut texture_atlas_handles: ResMut<TextureAtlasHandles>,
) {
    let texture_atlas_handles_map = &mut texture_atlas_handles.0;
    texture_atlas_handles_map.insert(
        "ship".to_string(),
        texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("spritesheets/ship.png"),
            Vec2::new(16., 24.),
            5,
            2,
        )),
    );
    texture_atlas_handles_map.insert(
        "enemy-big".to_string(),
        texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("spritesheets/enemy-big.png"),
            Vec2::new(32., 32.),
            2,
            1,
        )),
    );
    texture_atlas_handles_map.insert(
        "enemy-medium".to_string(),
        texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("spritesheets/enemy-medium.png"),
            Vec2::new(32., 16.),
            2,
            1,
        )),
    );
    texture_atlas_handles_map.insert(
        "enemy-small".to_string(),
        texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("spritesheets/enemy-small.png"),
            Vec2::new(16., 16.),
            2,
            1,
        )),
    );

    commands
        .spawn(Camera2dComponents::default())
        .spawn(SpriteSheetComponents {
            texture_atlas: texture_atlas_handles_map.get("ship").unwrap().clone(),
            transform: Transform::from_scale(Vec3::splat(4.)),
            sprite: TextureAtlasSprite::new(2),
            ..Default::default()
        })
        .with(Player)
        .with(MoveSpeed(PLAYER_SPEED))
        .with(MoveDirection::default())
        .with(PlayerAnimationState::Stabilized)
        .with(LastStateTransitionInstant(Instant::now()))
        .with(Animatable {
            sprite_cycle_delta: 5,
            cycle_timer: Timer::new(ANIMATION_INTERVAL, true),
        })
        .spawn(SpriteSheetComponents {
            texture_atlas: texture_atlas_handles_map.get("enemy-big").unwrap().clone(),
            transform: Transform {
                scale: Vec3::splat(4.),
                translation: Vec3::new(150., 0., 0.),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Enemy)
        .with(Animatable {
            sprite_cycle_delta: 1,
            cycle_timer: Timer::new(ANIMATION_INTERVAL, true),
        })
        .spawn(SpriteSheetComponents {
            texture_atlas: texture_atlas_handles_map
                .get("enemy-medium")
                .unwrap()
                .clone(),
            transform: Transform {
                scale: Vec3::splat(4.),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Enemy)
        .with(Animatable {
            sprite_cycle_delta: 1,
            cycle_timer: Timer::new(ANIMATION_INTERVAL, true),
        })
        .spawn(SpriteSheetComponents {
            texture_atlas: texture_atlas_handles_map
                .get("enemy-small")
                .unwrap()
                .clone(),
            transform: Transform {
                scale: Vec3::splat(4.),
                translation: Vec3::new(-150., 0., 0.),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Enemy)
        .with(Animatable {
            sprite_cycle_delta: 1,
            cycle_timer: Timer::new(ANIMATION_INTERVAL, true),
        });
}
