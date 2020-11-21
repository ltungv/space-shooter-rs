use bevy::core::Timer;
use bevy::math::Vec2;
use std::time::{Duration, Instant};

#[derive(Debug, Default)]
pub struct Enemy;

#[derive(Debug)]
pub struct Player {
    pub stabilization_duration: Duration,
    pub animation_state: PlayerAnimationState,
    pub last_transition_instant: Instant,
}

#[derive(Debug, PartialEq)]
pub enum PlayerAnimationState {
    FullLeft,
    HalfLeft,
    Stabilized,
    HalfRight,
    FullRight,
}

impl PlayerAnimationState {
    pub fn transition_left(&self) -> Self {
        match self {
            Self::FullLeft => Self::FullLeft,
            Self::HalfLeft => Self::FullLeft,
            Self::Stabilized => Self::HalfLeft,
            Self::HalfRight => Self::Stabilized,
            Self::FullRight => Self::HalfRight,
        }
    }

    pub fn transition_right(&self) -> Self {
        match self {
            Self::FullLeft => Self::HalfLeft,
            Self::HalfLeft => Self::Stabilized,
            Self::Stabilized => Self::HalfRight,
            Self::HalfRight => Self::FullRight,
            Self::FullRight => Self::FullRight,
        }
    }

    pub fn transition_stable(&self) -> Self {
        match self {
            Self::FullLeft => Self::HalfLeft,
            Self::HalfLeft => Self::Stabilized,
            Self::Stabilized => Self::Stabilized,
            Self::HalfRight => Self::Stabilized,
            Self::FullRight => Self::HalfRight,
        }
    }
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
