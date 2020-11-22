use bevy::core::Timer;
use bevy::math::Vec2;
use std::time::{Duration, Instant};

#[derive(Debug, Default)]
pub struct Enemy;

#[derive(Debug)]
pub struct Player {
    pub animation_state: PlayerAnimationState,
    pub transition_duration: Duration,
    pub transition_instant: Instant,
}

#[derive(Debug, PartialEq)]
pub enum PlayerAnimationState {
    FullLeft,
    HalfLeft,
    Stabilized,
    HalfRight,
    FullRight,
}

#[derive(Debug, PartialEq, Default)]
pub struct MoveDirection(pub Vec2);

#[derive(Debug)]
pub struct MoveSpeed(pub f32);

#[derive(Debug)]
pub struct Animatable {
    pub sprite_cycle_delta: usize,
    pub cycle_timer: Timer,
}
