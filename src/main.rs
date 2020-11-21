use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
};

use std::time::{Duration, Instant};

const WINDOW_WIDTH: f32 = 600.0;
const WINDOW_HEIGHT: f32 = 800.0;

const PLAYER_SPRITE_HEIGHT: f32 = 24.0;
const PLAYER_SPEED: f32 = 500.0;
const PLAYER_SPRITE_WIDTH: f32 = 16.0;

const STABILIZATION_DELAY: Duration = Duration::from_millis(100);
const ANIMATION_INTERVAL: Duration = Duration::from_millis(200);

fn main() {
    App::build()
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
        .add_system(player_animation.system())
        .run();
}

#[derive(Debug, PartialEq, Default)]
struct MoveDirection(Vec2);

#[derive(Debug)]
struct MoveSpeed(f32);

#[derive(Debug, Default)]
struct Player;

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
struct AnimationCycleTimer(Timer);

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
        .min((WINDOW_WIDTH - width) / 2.0)
        // lower bound
        .max(-(WINDOW_WIDTH - width) / 2.0);

    // Y-axis movement
    *transform.translation.y_mut() += time.delta_seconds * move_direction.y() * move_speed;
    *transform.translation.y_mut() = transform
        .translation
        .y()
        // upper bound
        .min((WINDOW_HEIGHT - height) / 2.0)
        // lower bound
        .max(-(WINDOW_HEIGHT - height) / 2.0);
}

fn player_direction(
    kb_input: Res<Input<KeyCode>>,
    _player: &Player,
    mut move_direction: Mut<MoveDirection>,
) {
    *move_direction.0.y_mut() = 0.0;
    *move_direction.0.x_mut() = 0.0;

    if kb_input.pressed(KeyCode::Up) {
        *move_direction.0.y_mut() += 1.0;
    }
    if kb_input.pressed(KeyCode::Down) {
        *move_direction.0.y_mut() -= 1.0;
    }
    if kb_input.pressed(KeyCode::Left) {
        *move_direction.0.x_mut() -= 1.0;
    }
    if kb_input.pressed(KeyCode::Right) {
        *move_direction.0.x_mut() += 1.0;
    }
}

#[allow(clippy::too_many_arguments)]
fn player_animation(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    _player: &Player,
    MoveDirection(move_direction): &MoveDirection,
    texture_atlas_handle: &Handle<TextureAtlas>,
    mut sprite: Mut<TextureAtlasSprite>,
    mut animation_state: Mut<PlayerAnimationState>,
    mut last_state_transition_instant: Mut<LastStateTransitionInstant>,
    mut animation_cycle_timer: Mut<AnimationCycleTimer>,
) {
    animation_cycle_timer.0.tick(time.delta_seconds);
    if animation_cycle_timer.0.finished {
        let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
        sprite.index = ((sprite.index as usize + 5) % texture_atlas.textures.len()) as u32;
    }

    if let Some(now) = time.instant {
        if now.duration_since(last_state_transition_instant.0) >= STABILIZATION_DELAY {
            let x_direction = move_direction.x();
            let new_animation_state = if x_direction < 0.0 {
                animation_state.transition_left()
            } else if x_direction > 0.0 {
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
) {
    let ship_texture_handle = asset_server.load("textures/ship.png");
    let ship_texture_atlas = TextureAtlas::from_grid(
        ship_texture_handle,
        Vec2::new(PLAYER_SPRITE_WIDTH, PLAYER_SPRITE_HEIGHT),
        5,
        2,
    );
    let ship_texture_atlas_handle = texture_atlases.add(ship_texture_atlas);

    commands
        .spawn(Camera2dComponents::default())
        .spawn(SpriteSheetComponents {
            texture_atlas: ship_texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(4.0)),
            sprite: TextureAtlasSprite::new(2),
            ..Default::default()
        })
        .with(Player)
        .with(MoveSpeed(PLAYER_SPEED))
        .with(MoveDirection::default())
        .with(PlayerAnimationState::Stabilized)
        .with(LastStateTransitionInstant(Instant::now()))
        .with(AnimationCycleTimer(Timer::new(ANIMATION_INTERVAL, true)));
}
