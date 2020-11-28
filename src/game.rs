use crate::constant::*;
use crate::entity;
use crate::system;
use bevy::prelude::*;
use std::collections::HashMap;

/// A plugin that add and initialize all the entities and systems for running the game
#[derive(Default)]
pub struct Game;

impl Plugin for Game {
    // this is where we set up our plugin
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<GameState>()
            .add_startup_system(initialize_enemy_texture_atlas_handles.system())
            .add_startup_system(entity::initialize_camera.system())
            .add_startup_system(entity::initialize_ship.system())
            .add_system(system::ship_control.system())
            .add_system(system::ship_movement.system())
            .add_system(system::ship_state_transition.system())
            .add_system(system::enemies_spawner.system())
            .add_system(system::enemies_movement.system())
            .add_system(system::enemies_despawner.system())
            .add_system(system::entities_animation.system());
    }
}

/// A structure for holding general game states that are shared across multiple systems
#[derive(Default)]
pub struct GameState {
    pub enemy_texture_atlas_handles: HashMap<String, Handle<TextureAtlas>>,
}

/// Loads all the necessary assets and creates the initial entites for the game
fn initialize_enemy_texture_atlas_handles(
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<GameState>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    game_state.enemy_texture_atlas_handles.insert(
        "small".to_string(),
        texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("spritesheets/enemy-small.png"),
            Vec2::new(ENEMY_SMALL_SPRITE_WIDTH, ENEMY_SMALL_SPRITE_HEIGHT),
            2,
            1,
        )),
    );

    game_state.enemy_texture_atlas_handles.insert(
        "medium".to_string(),
        texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("spritesheets/enemy-medium.png"),
            Vec2::new(ENEMY_MEDIUM_SPRITE_WIDTH, ENEMY_MEDIUM_SPRITE_HEIGHT),
            2,
            1,
        )),
    );

    game_state.enemy_texture_atlas_handles.insert(
        "big".to_string(),
        texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("spritesheets/enemy-big.png"),
            Vec2::new(ENEMY_BIG_SPRITE_WIDTH, ENEMY_BIG_SPRITE_HEIGHT),
            2,
            1,
        )),
    );
}
