use crate::{
    component::{EnemyVariant, HitBox},
    constant::*,
};
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct EnemyData {
    pub variant: EnemyVariant,
    pub hit_box: HitBox,
    pub texture_atlas_handle: Handle<TextureAtlas>,
}

/// A structure for holding general game states that are shared across multiple systems
#[derive(Default)]
pub struct GameState {
    pub enemy_data: HashMap<String, EnemyData>,
}

/// Loads all the necessary assets and creates the initial entites for the game
pub fn initialize_enemy_data(
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<GameState>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture = asset_server.load("spritesheets/enemy-small.png");
    let texture_size = Vec2::new(ENEMY_SMALL_SPRITE_WIDTH, ENEMY_SMALL_SPRITE_HEIGHT);
    let texture_atlas = TextureAtlas::from_grid(texture, texture_size, 2, 1);
    game_state.enemy_data.insert(
        "small".to_string(),
        EnemyData {
            variant: EnemyVariant::Small,
            hit_box: HitBox {
                width: texture_size.x(),
                height: texture_size.y(),
            },
            texture_atlas_handle: texture_atlases.add(texture_atlas),
        },
    );

    let texture = asset_server.load("spritesheets/enemy-medium.png");
    let texture_size = Vec2::new(ENEMY_MEDIUM_SPRITE_WIDTH, ENEMY_MEDIUM_SPRITE_HEIGHT);
    let texture_atlas = TextureAtlas::from_grid(texture, texture_size, 2, 1);
    game_state.enemy_data.insert(
        "medium".to_string(),
        EnemyData {
            variant: EnemyVariant::Medium,
            hit_box: HitBox {
                width: texture_size.x(),
                height: texture_size.y(),
            },
            texture_atlas_handle: texture_atlases.add(texture_atlas),
        },
    );

    let texture = asset_server.load("spritesheets/enemy-big.png");
    let texture_size = Vec2::new(ENEMY_BIG_SPRITE_WIDTH, ENEMY_BIG_SPRITE_HEIGHT);
    let texture_atlas = TextureAtlas::from_grid(texture, texture_size, 2, 1);
    game_state.enemy_data.insert(
        "big".to_string(),
        EnemyData {
            variant: EnemyVariant::Big,
            hit_box: HitBox {
                width: texture_size.x(),
                height: texture_size.y(),
            },
            texture_atlas_handle: texture_atlases.add(texture_atlas),
        },
    );
}
