mod components;
mod constant;
mod entity;
mod events;
mod game;
mod resource;
mod systems;

use bevy::prelude::{App, DefaultPlugins, WindowDescriptor};
use constant::{ARENA_HEIGHT, ARENA_SCALE, ARENA_WIDTH};

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Space shooter!".to_string(),
            width: (ARENA_WIDTH * ARENA_SCALE) as u32,
            height: (ARENA_HEIGHT * ARENA_SCALE) as u32,
            resizable: false,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(game::Game)
        .run();
}
