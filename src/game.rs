use bevy::prelude::*;
use std::collections::HashMap;

use crate::entity;
use crate::system;

/// A plugin that add and initialize all the entities and systems for running the game
#[derive(Default)]
pub struct Game;

impl Plugin for Game {
    // this is where we set up our plugin
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<GameState>()
            .add_startup_system(load_spritesheets.system())
            .add_startup_system(initialize_entities.system())
            .add_system(system::player_control.system())
            .add_system(system::player_movement.system())
            .add_system(system::player_state_transition.system())
            .add_system(system::entities_animation.system());
    }
}

/// A structure for holding general game states that are shared across multiple systems
#[derive(Default)]
pub struct GameState {
    pub texture_atlas_handles: HashMap<String, Handle<TextureAtlas>>,
}

/// Create initial entities for the game to work
fn initialize_entities(mut commands: Commands, game_state: Res<GameState>) {
    commands.spawn(Camera2dComponents::default());
    entity::create_player(
        &mut commands,
        game_state
            .texture_atlas_handles
            .get("ship")
            .expect("Could not get player's texture atlas handle")
            .clone(),
    );
    // TODO: Remove enemy creation (this is only for testing puposes)
    entity::create_enemy(
        &mut commands,
        game_state
            .texture_atlas_handles
            .get("big-enemy")
            .expect("Could not get enemy's texture atlas handle")
            .clone(),
    );
}

// TODO: Correct size and position of sprites for some sprite sheets
/// Loads all the necessary assets and creates the initial entites for the game
fn load_spritesheets(
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<GameState>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Loading spritesheets into texture atlases
    game_state.texture_atlas_handles.insert(
        "ship".to_string(),
        texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("spritesheets/ship.png"),
            Vec2::new(16., 24.),
            5,
            2,
        )),
    );
    game_state.texture_atlas_handles.insert(
        "enemy-big".to_string(),
        texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("spritesheets/enemy-big.png"),
            Vec2::new(32., 32.),
            2,
            1,
        )),
    );
    game_state.texture_atlas_handles.insert(
        "enemy-medium".to_string(),
        texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("spritesheets/enemy-medium.png"),
            Vec2::new(32., 16.),
            2,
            1,
        )),
    );
    game_state.texture_atlas_handles.insert(
        "enemy-small".to_string(),
        texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("spritesheets/enemy-small.png"),
            Vec2::new(16., 16.),
            2,
            1,
        )),
    );
    game_state.texture_atlas_handles.insert(
        "explosion".to_string(),
        texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("spritesheets/explosion.png"),
            Vec2::new(16., 16.),
            5,
            1,
        )),
    );
    game_state.texture_atlas_handles.insert(
        "laser-bolts".to_string(),
        texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("spritesheets/explosion.png"),
            Vec2::new(16., 16.),
            2,
            2,
        )),
    );
}
