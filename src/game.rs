use crate::{
    constant::{
        ENEMY_BIG_SPRITE_HEIGHT, ENEMY_BIG_SPRITE_WIDTH, ENEMY_MEDIUM_SPRITE_HEIGHT,
        ENEMY_MEDIUM_SPRITE_WIDTH, ENEMY_SMALL_SPRITE_HEIGHT, ENEMY_SMALL_SPRITE_WIDTH,
        PLAYER_SPRITE_HEIGHT, PLAYER_SPRITE_WIDTH,
    },
    entity, system,
};
use bevy::prelude::{
    AppBuilder, AssetServer, Assets, Camera2dComponents, Commands, Handle, IntoForEachSystem,
    IntoQuerySystem, Plugin, Res, ResMut, TextureAtlas, Timer, Vec2,
};
use std::{collections::HashMap, time::Duration};

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
            .add_system(system::enemies_spawner.system())
            .add_system(system::enemies_movement.system())
            .add_system(system::enemies_despawner.system())
            .add_system(system::entities_animation.system());
    }
}

/// A structure for holding general game states that are shared across multiple systems
#[derive(Default)]
pub struct GameState {
    pub texture_atlases: HashMap<String, (Handle<TextureAtlas>, Vec2)>,
}

/// Create initial entities for the game to work
fn initialize_entities(mut commands: Commands, game_state: Res<GameState>) {
    commands.spawn(Camera2dComponents::default());
    entity::create_player(
        &mut commands,
        game_state
            .texture_atlases
            .get("ship")
            .expect("Could not get player's texture atlas handle")
            .0
            .clone(),
    );
    entity::create_enemies_spawner(&mut commands, Timer::new(Duration::from_secs(2), true))
}

// TODO: Correct size and position of sprites for some sprite sheets
/// Loads all the necessary assets and creates the initial entites for the game
fn load_spritesheets(
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<GameState>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let sprite_size = Vec2::new(PLAYER_SPRITE_WIDTH, PLAYER_SPRITE_HEIGHT);
    game_state.texture_atlases.insert(
        "ship".to_string(),
        (
            texture_atlases.add(TextureAtlas::from_grid(
                asset_server.load("spritesheets/ship.png"),
                sprite_size,
                5,
                2,
            )),
            sprite_size,
        ),
    );

    let sprite_size = Vec2::new(ENEMY_BIG_SPRITE_WIDTH, ENEMY_BIG_SPRITE_HEIGHT);
    game_state.texture_atlases.insert(
        "enemy-big".to_string(),
        (
            texture_atlases.add(TextureAtlas::from_grid(
                asset_server.load("spritesheets/enemy-big.png"),
                sprite_size,
                2,
                1,
            )),
            sprite_size,
        ),
    );

    let sprite_size = Vec2::new(ENEMY_MEDIUM_SPRITE_WIDTH, ENEMY_MEDIUM_SPRITE_HEIGHT);
    game_state.texture_atlases.insert(
        "enemy-medium".to_string(),
        (
            texture_atlases.add(TextureAtlas::from_grid(
                asset_server.load("spritesheets/enemy-medium.png"),
                sprite_size,
                2,
                1,
            )),
            sprite_size,
        ),
    );

    let sprite_size = Vec2::new(ENEMY_SMALL_SPRITE_WIDTH, ENEMY_SMALL_SPRITE_HEIGHT);
    game_state.texture_atlases.insert(
        "enemy-small".to_string(),
        (
            texture_atlases.add(TextureAtlas::from_grid(
                asset_server.load("spritesheets/enemy-small.png"),
                sprite_size,
                2,
                1,
            )),
            sprite_size,
        ),
    );
}
