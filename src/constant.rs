use std::time::Duration;

pub const ARENA_WIDTH: f32 = 180.;
pub const ARENA_HEIGHT: f32 = 240.;
pub const ARENA_SCALE: f32 = 2.0;
pub const ANIMATION_INTERVAL: Duration = Duration::from_millis(200);

pub const SHIP_INITIAL_MOVE_SPEED: f32 = 100.;
pub const SHIP_SPRITE_WIDTH: f32 = 16.;
pub const SHIP_SPRITE_HEIGHT: f32 = 24.;
pub const SHIP_STATE_TRANSITION_DURATION: Duration = Duration::from_millis(100);

pub const SHIP_LASER_SPRITE_WIDTH: f32 = 5.;
pub const SHIP_LASER_SPRITE_HEIGHT: f32 = 13.;
pub const SHIP_LASER_INITIAL_VELOCITY: (f32, f32) = (0., 100.);
pub const SHIP_LASER_COOLDOWN_DURATION: Duration = Duration::from_millis(400);
pub const SHIP_LASER_TIME_TO_LIVE_DURATION: Duration = Duration::from_secs(3);

pub const ENEMY_INITIAL_VELOCITY: (f32, f32) = (0., -50.);
pub const ENEMY_SPAWN_INTERVAL: Duration = Duration::from_millis(1000);
pub const SPAWN_WEIGHT_ENEMY_SMALL: u8 = 6;
pub const SPAWN_WEIGHT_ENEMY_MEDIUM: u8 = 3;
pub const SPAWN_WEIGHT_ENEMY_BIG: u8 = 1;

pub const ENEMY_BIG_SPRITE_WIDTH: f32 = 32.;
pub const ENEMY_BIG_SPRITE_HEIGHT: f32 = 32.;

pub const ENEMY_MEDIUM_SPRITE_WIDTH: f32 = 32.;
pub const ENEMY_MEDIUM_SPRITE_HEIGHT: f32 = 16.;

pub const ENEMY_SMALL_SPRITE_WIDTH: f32 = 16.;
pub const ENEMY_SMALL_SPRITE_HEIGHT: f32 = 16.;

pub const EXPLOSION_SPRITE_WIDTH: f32 = 16.;
pub const EXPLOSION_SPRITE_HEIGHT: f32 = 16.;
