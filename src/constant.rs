use std::time::Duration;

pub const ARENA_WIDTH: f32 = 320.;
pub const ARENA_HEIGHT: f32 = 480.;
pub const ANIMATION_INTERVAL: Duration = Duration::from_millis(200);

pub const SHIP_MAX_SPEED: f32 = 300.;
pub const SHIP_STATE_TRANSITION_DURATION: Duration = Duration::from_millis(100);

pub const SHIP_SPRITE_WIDTH: f32 = 16.;
pub const SHIP_SPRITE_HEIGHT: f32 = 24.;

pub const ENEMY_MAX_SPEED: f32 = 80.;
pub const ENEMY_INITIAL_VELOCITY: (f32, f32) = (0., -80.);
pub const ENEMY_SPAWN_INTERVAL: Duration = Duration::from_secs(1);

pub const ENEMY_BIG_SPRITE_WIDTH: f32 = 32.;
pub const ENEMY_BIG_SPRITE_HEIGHT: f32 = 32.;

pub const ENEMY_MEDIUM_SPRITE_WIDTH: f32 = 32.;
pub const ENEMY_MEDIUM_SPRITE_HEIGHT: f32 = 16.;

pub const ENEMY_SMALL_SPRITE_WIDTH: f32 = 16.;
pub const ENEMY_SMALL_SPRITE_HEIGHT: f32 = 16.;

pub const SPAWN_WEIGHT_ENEMY_BIG: u8 = 6;
pub const SPAWN_WEIGHT_ENEMY_MEDIUM: u8 = 3;
pub const SPAWN_WEIGHT_ENEMY_SMAL: u8 = 1;
