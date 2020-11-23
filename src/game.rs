use crate::{
    constant::{
        ARENA_HEIGHT, ENEMY_BIG_SPRITE_HEIGHT, ENEMY_BIG_SPRITE_WIDTH, ENEMY_MEDIUM_SPRITE_HEIGHT,
        ENEMY_MEDIUM_SPRITE_WIDTH, ENEMY_SMALL_SPRITE_HEIGHT, ENEMY_SMALL_SPRITE_WIDTH,
        PLAYER_SPRITE_HEIGHT, PLAYER_SPRITE_WIDTH, SPRITE_UNIFORM_SCALING_FACTOR,
    },
    entity, system,
};
use bevy::prelude::{
    AppBuilder, AssetServer, Assets, Camera2dComponents, Commands, Handle, IntoForEachSystem,
    IntoQuerySystem, Plugin, Res, ResMut, TextureAtlas, Vec2, Vec3,
};
use std::collections::HashMap;

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
            .add_system(system::enemies_movement.system())
            .add_system(system::enemies_despawner.system())
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
    entity::create_enemy(
        &mut commands,
        game_state
            .texture_atlas_handles
            .get("enemy-small")
            .expect("Could not get small enemy's texture atlas handle")
            .clone(),
        50.,
        Vec2::new(0.0, -100.),
        Vec3::new(
            -150.,
            (ARENA_HEIGHT + ENEMY_BIG_SPRITE_HEIGHT * SPRITE_UNIFORM_SCALING_FACTOR) / 2.,
            0.,
        ),
    );

    entity::create_enemy(
        &mut commands,
        game_state
            .texture_atlas_handles
            .get("enemy-medium")
            .expect("Could not get medium enemy's texture atlas handle")
            .clone(),
        50.,
        Vec2::new(0.0, -80.),
        Vec3::new(
            0.,
            (ARENA_HEIGHT + ENEMY_BIG_SPRITE_HEIGHT * SPRITE_UNIFORM_SCALING_FACTOR) / 2.,
            0.,
        ),
    );

    entity::create_enemy(
        &mut commands,
        game_state
            .texture_atlas_handles
            .get("enemy-big")
            .expect("Could not get big enemy's texture atlas handle")
            .clone(),
        50.,
        Vec2::new(0.0, -60.),
        Vec3::new(
            150.,
            (ARENA_HEIGHT + ENEMY_BIG_SPRITE_HEIGHT * SPRITE_UNIFORM_SCALING_FACTOR) / 2.,
            0.,
        ),
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
            Vec2::new(PLAYER_SPRITE_WIDTH, PLAYER_SPRITE_HEIGHT),
            5,
            2,
        )),
    );
    game_state.texture_atlas_handles.insert(
        "enemy-big".to_string(),
        texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("spritesheets/enemy-big.png"),
            Vec2::new(ENEMY_BIG_SPRITE_WIDTH, ENEMY_BIG_SPRITE_HEIGHT),
            2,
            1,
        )),
    );
    game_state.texture_atlas_handles.insert(
        "enemy-medium".to_string(),
        texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("spritesheets/enemy-medium.png"),
            Vec2::new(ENEMY_MEDIUM_SPRITE_WIDTH, ENEMY_MEDIUM_SPRITE_HEIGHT),
            2,
            1,
        )),
    );
    game_state.texture_atlas_handles.insert(
        "enemy-small".to_string(),
        texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("spritesheets/enemy-small.png"),
            Vec2::new(ENEMY_SMALL_SPRITE_WIDTH, ENEMY_SMALL_SPRITE_HEIGHT),
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
