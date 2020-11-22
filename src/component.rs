use bevy::core::Timer;
use bevy::math::Vec2;
use std::time::{Duration, Instant};

/// Component that marks an entity to be an enemy in the game
#[derive(Debug, Default)]
pub struct Enemy;

/// Component that marks an entity to be a player in the game
#[derive(Debug)]
pub struct Player {
    pub animation_state: PlayerAnimationState,
    pub transition_duration: Duration,
    pub transition_instant: Instant,
}

/// Different states of the player when moving left/right
#[derive(Debug, PartialEq)]
pub enum PlayerAnimationState {
    FullLeft,
    HalfLeft,
    Stabilized,
    HalfRight,
    FullRight,
}

// TODO: Maybe new component: Velocity = MoveDirection + MoveSpeed

/// Component that determines an entity's moving direction along the x-axis and y-axis
#[derive(Debug, PartialEq, Default)]
pub struct MoveDirection(pub Vec2);

/// Component that determines the moving speed of an entity
#[derive(Debug)]
pub struct MoveSpeed(pub f32);

/// Component that determines when to change the sprite and which sprite index to change to when
/// doing simple animation
#[derive(Debug)]
pub struct Animatable {
    pub sprite_cycle_delta: usize,
    pub cycle_timer: Timer,
}
