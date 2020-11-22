mod component;
mod entity;
mod game;
mod system;

use bevy::prelude::*;

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Space shooter!".to_string(),
            width: 600,
            height: 800,
            resizable: false,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(game::Game::default())
        .run();
}
