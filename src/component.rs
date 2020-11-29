use bevy::prelude::*;
use std::time::{Duration, Instant};

#[derive(Debug, PartialEq, Clone)]
pub enum EnemyVariant {
    Small,
    Medium,
    Big,
}

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
pub struct EnemySpawner {
    pub timer: Timer,
    pub weights: Vec<(String, u8)>,
}

/// Component that determines how entities move within the 2D space
#[derive(Debug)]
pub struct Motion {
    pub max_speed: f32,
    pub velocity: Vec2,
}

/// Component that determines when to change the sprite and which sprite index to change to when
/// doing simple animation
#[derive(Debug)]
pub struct Animation {
    pub idx_delta: u32,
    pub sprite_count: u32,
    pub timer: Timer,
}

/// Component that determines the smallest possible box that includes rendered the entity.
#[derive(Debug, Clone)]
pub struct HitBox {
    pub width: f32,
    pub height: f32,
}
