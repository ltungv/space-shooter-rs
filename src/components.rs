use bevy::prelude::*;
use std::time::Duration;

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
    pub move_speed: f32,
    pub transition_timer: Timer,
}

/// The type of enemy
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum EnemyVariant {
    Small,
    Medium,
    Big,
}

/// Component marks an entity to be an enemy
#[derive(Debug)]
pub struct Enemy {
    pub variant: EnemyVariant,
}

/// Component determines the spawn rate and spawn probability of entities
#[derive(Debug)]
pub struct EnemySpawner {
    pub timer: Timer,
    pub weights: Vec<(EnemyVariant, u8)>,
}

#[derive(Debug)]
pub struct Laser {
    pub source: Entity,
}

#[derive(Debug)]
pub struct Weapon {
    pub cooldown_timer: Timer,
    pub laser_velocity: Velocity,
    pub laser_hit_box: HitBox,
    pub laser_time_to_live_duration: Duration,
    pub laser_initial_sprite_idx: u32,
}

#[derive(Debug)]
pub struct Explosion;

/// Component that determines when to change the sprite and which sprite index to change to when
/// doing simple animation
#[derive(Debug)]
pub struct Animation {
    pub idx_delta: u32,
    pub sprite_count: u32,
    pub timer: Timer,
}

#[derive(Debug)]
pub struct ConstrainedToArena;

#[derive(Debug, Clone, Copy)]
pub struct Velocity(pub Vec2);

/// Component that determines the smallest possible box that includes rendered the entity.
#[derive(Debug, Clone, Copy)]
pub struct HitBox(pub Vec2);

#[derive(Debug, Clone)]
pub struct TimeToLive(pub Timer);
