use bevy::prelude::*;
use std::collections::HashMap;

use std::time::{Duration, Instant};

use crate::components;
use crate::systems;

const PLAYER_SPEED: f32 = 500.;
const STABILIZATION_DURATION: Duration = Duration::from_millis(100);
const ANIMATION_INTERVAL: Duration = Duration::from_millis(200);

#[derive(Default)]
pub struct Game;

#[derive(Default)]
pub struct GameState {
    pub texture_atlas_handles: HashMap<String, Handle<TextureAtlas>>,
}

impl Plugin for Game {
    // this is where we set up our plugin
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<GameState>()
            .add_startup_system(load_spritesheets_as_texture_atlases.system())
            .add_startup_system(create_camera_entity.system())
            .add_startup_system(create_player_entity.system())
            .add_startup_system(create_enemy_entity.system())
            .add_system(systems::player_control.system())
            .add_system(systems::player_movement.system())
            .add_system(systems::player_state_transition.system())
            .add_system(systems::entities_animation.system());
    }
}

fn create_camera_entity(mut commands: Commands) {
    commands.spawn(Camera2dComponents::default());
}

fn create_player_entity(mut commands: Commands, game_state: Res<GameState>) {
    commands
        // PLAYER
        .spawn(SpriteSheetComponents {
            texture_atlas: game_state
                .texture_atlas_handles
                .get("ship")
                .unwrap()
                .clone(),
            transform: Transform::from_scale(Vec3::splat(4.)),
            sprite: TextureAtlasSprite::new(2),
            ..Default::default()
        })
        .with(components::Player {
            stabilization_duration: STABILIZATION_DURATION,
            animation_state: components::PlayerAnimationState::Stabilized,
            last_transition_instant: Instant::now(),
        })
        .with(components::MoveSpeed(PLAYER_SPEED))
        .with(components::MoveDirection::default())
        .with(components::Animatable {
            sprite_cycle_delta: 5,
            cycle_timer: Timer::new(ANIMATION_INTERVAL, true),
        });
}

pub fn create_enemy_entity(mut commands: Commands, game_state: Res<GameState>) {
    let texture_atlas_handles_map = &game_state.texture_atlas_handles;
    commands
        .spawn(SpriteSheetComponents {
            texture_atlas: texture_atlas_handles_map.get("enemy-big").unwrap().clone(),
            transform: Transform {
                scale: Vec3::splat(4.),
                translation: Vec3::new(150., 0., 0.),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(components::Enemy)
        .with(components::Animatable {
            sprite_cycle_delta: 1,
            cycle_timer: Timer::new(ANIMATION_INTERVAL, true),
        });
}

fn load_spritesheets_as_texture_atlases(
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut game_state: ResMut<GameState>,
) {
    let texture_atlas_handles_map = &mut game_state.texture_atlas_handles;
    texture_atlas_handles_map.insert(
        "ship".to_string(),
        texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("spritesheets/ship.png"),
            Vec2::new(16., 24.),
            5,
            2,
        )),
    );
    texture_atlas_handles_map.insert(
        "enemy-big".to_string(),
        texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("spritesheets/enemy-big.png"),
            Vec2::new(32., 32.),
            2,
            1,
        )),
    );
    texture_atlas_handles_map.insert(
        "enemy-medium".to_string(),
        texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("spritesheets/enemy-medium.png"),
            Vec2::new(32., 16.),
            2,
            1,
        )),
    );
    texture_atlas_handles_map.insert(
        "enemy-small".to_string(),
        texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("spritesheets/enemy-small.png"),
            Vec2::new(16., 16.),
            2,
            1,
        )),
    );
}
