use crate::{
    constant::{
        ENEMY_BIG_SPRITE_HEIGHT, ENEMY_BIG_SPRITE_WIDTH, ENEMY_MEDIUM_SPRITE_HEIGHT,
        ENEMY_MEDIUM_SPRITE_WIDTH, ENEMY_SMALL_SPRITE_HEIGHT, ENEMY_SMALL_SPRITE_WIDTH,
        EXPLOSION_SPRITE_HEIGHT, EXPLOSION_SPRITE_WIDTH, SHIP_SPRITE_HEIGHT, SHIP_SPRITE_WIDTH,
    },
    events::{
        CollisionEnemyShipLaserEvent, SpawnEnemyEvent, SpawnExplosionEvent, SpawnShipLaserEvent,
    },
};
use bevy::prelude::*;

#[derive(Default)]
pub struct EventReaders {
    pub collision_enemy_ship_laser: EventReader<CollisionEnemyShipLaserEvent>,
    pub spawn_enemy: EventReader<SpawnEnemyEvent>,
    pub spawn_explosion: EventReader<SpawnExplosionEvent>,
    pub spawn_ship_laser: EventReader<SpawnShipLaserEvent>,
}

pub struct TextureAtlasHandles {
    pub ship: Handle<TextureAtlas>,
    pub enemy_small: Handle<TextureAtlas>,
    pub enemy_medium: Handle<TextureAtlas>,
    pub enemy_big: Handle<TextureAtlas>,
    pub laser_bolts: Handle<TextureAtlas>,
    pub explosion: Handle<TextureAtlas>,
}

impl FromResources for TextureAtlasHandles {
    fn from_resources(resources: &Resources) -> Self {
        let asset_server = resources
            .get::<AssetServer>()
            .expect("Could not get asset server");

        let mut texture_atlases = resources
            .get_mut::<Assets<TextureAtlas>>()
            .expect("Could not get texture atlas asset");

        // SHIP SPRITESHEET
        let texture_atlas = TextureAtlas::from_grid(
            asset_server.load("spritesheets/ship.png"),
            Vec2::new(SHIP_SPRITE_WIDTH, SHIP_SPRITE_HEIGHT),
            5,
            2,
        );
        let ship = texture_atlases.add(texture_atlas);

        // SMALL ENEMY SPRITESHEET
        let texture_atlas = TextureAtlas::from_grid(
            asset_server.load("spritesheets/enemy-small.png"),
            Vec2::new(ENEMY_SMALL_SPRITE_WIDTH, ENEMY_SMALL_SPRITE_HEIGHT),
            2,
            1,
        );
        let enemy_small = texture_atlases.add(texture_atlas);

        // MEDIUM ENEMY SPRITESHEET
        let texture_atlas = TextureAtlas::from_grid(
            asset_server.load("spritesheets/enemy-medium.png"),
            Vec2::new(ENEMY_MEDIUM_SPRITE_WIDTH, ENEMY_MEDIUM_SPRITE_HEIGHT),
            2,
            1,
        );
        let enemy_medium = texture_atlases.add(texture_atlas);

        // BIG ENEMY SPRITESHEET
        let texture_atlas = TextureAtlas::from_grid(
            asset_server.load("spritesheets/enemy-big.png"),
            Vec2::new(ENEMY_BIG_SPRITE_WIDTH, ENEMY_BIG_SPRITE_HEIGHT),
            2,
            1,
        );
        let enemy_big = texture_atlases.add(texture_atlas);

        // LASER SPRITESHEET
        let mut texture_atlas = TextureAtlas::new_empty(
            asset_server.load("spritesheets/laser-bolts.png"),
            Vec2::new(32., 32.),
        );
        // enemy_laser_01
        texture_atlas.add_texture(bevy::sprite::Rect {
            min: Vec2::new(6., 7.),
            max: Vec2::new(11., 12.),
        });
        // ship_laser_01
        texture_atlas.add_texture(bevy::sprite::Rect {
            min: Vec2::new(6., 18.),
            max: Vec2::new(11., 30.),
        });
        // enemy_laser_02
        texture_atlas.add_texture(bevy::sprite::Rect {
            min: Vec2::new(20., 7.),
            max: Vec2::new(25., 12.),
        });
        // ship_laser_02
        texture_atlas.add_texture(bevy::sprite::Rect {
            min: Vec2::new(20., 18.),
            max: Vec2::new(25., 31.),
        });
        let laser_bolts = texture_atlases.add(texture_atlas);

        // EXPLOSION SPRITESHEET
        let texture_atlas = TextureAtlas::from_grid(
            asset_server.load("spritesheets/explosion.png"),
            Vec2::new(EXPLOSION_SPRITE_WIDTH, EXPLOSION_SPRITE_HEIGHT),
            5,
            1,
        );
        let explosion = texture_atlases.add(texture_atlas);

        Self {
            ship,
            enemy_small,
            enemy_medium,
            enemy_big,
            laser_bolts,
            explosion,
        }
    }
}
