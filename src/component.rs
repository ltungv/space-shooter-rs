use bevy::prelude::{Timer, Vec2};
use std::time::{Duration, Instant};

#[derive(Debug, PartialEq)]
pub enum EnemyVariant {
    Small,
    Medium,
    Big,
}

/// Component that marks an entity to be an enemy in the game
#[derive(Debug)]
pub struct Enemy {
    pub variant: EnemyVariant,
}

/// Different states of the  when moving left/right
#[derive(Debug, PartialEq)]
pub enum ShipAnimationState {
    FullLeft,
    HalfLeft,
    Stabilized,
    HalfRight,
    FullRight,
}

/// Component that marks an entity to be a ship in the game
#[derive(Debug)]
pub struct Ship {
    pub animation_state: ShipAnimationState,
    pub transition_duration: Duration,
    pub transition_instant: Instant,
}

#[derive(Debug)]
pub struct Spawner {
    pub spawn_timer: Timer,

    /// Name-Probability pairs
    pub spawn_prob_weights: Vec<(String, u8)>,
}

/// Component that determines how entities move within the 2D space
#[derive(Debug)]
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
#[derive(Debug)]
pub struct HitBox {
    pub width: f32,
    pub height: f32,
}
