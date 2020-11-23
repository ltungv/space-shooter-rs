use bevy::prelude::{Timer, Vec2};
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

/// Component that determines how entities move within the 2D space
#[derive(Debug, PartialEq, Default)]
pub struct Motion {
    // TODO: movement on different axis can have different speed
    pub max_speed: f32,
    pub velocity: Vec2,
}

/// Component that determines when to change the sprite and which sprite index to change to when
/// doing simple animation
#[derive(Debug)]
pub struct Animatable {
    pub sprite_idx_delta: u32,
    pub sprite_count: u32,
    pub cycle_timer: Timer,
}

/// Component that determines the smallest possible box that includes rendered the entity.
pub struct HitBox {
    pub width: f32,
    pub height: f32,
}
