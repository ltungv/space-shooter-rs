mod component;
mod entity;
mod game;
mod system;

use bevy::prelude::*;

pub const WINDOW_WIDTH: f32 = 600.;
pub const WINDOW_HEIGHT: f32 = 800.;

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Space shooter!".to_string(),
            width: WINDOW_WIDTH as u32,
            height: WINDOW_HEIGHT as u32,
            resizable: false,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(game::Game::default())
        .run();
}
