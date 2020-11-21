use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
};

use std::time;

const PLAYER_SPEED: f32 = 300.0;
const PLAYER_SPRITE_WIDTH: f32 = 16.0;
const PLAYER_SPRITE_HEIGHT: f32 = 24.0;
const STABILIZATION_DELAY: time::Duration = time::Duration::from_millis(150);
const ANIMATION_INTERVAL: time::Duration = time::Duration::from_millis(200);
const WINDOW_WIDTH: f32 = 480.0;
const WINDOW_HEIGHT: f32 = 720.0;

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
        .add_startup_system(setup.system())
        .add_system(player_animation.system())
        .add_system(player_keyboard_input_control.system())
        .run();
}

#[derive(Default)]
struct MoveDuration {
    left: time::Duration,
    right: time::Duration,
    stabilizing: time::Duration,
}

#[derive(Debug, PartialEq)]
enum PlayerAnimationState {
    FullLeft,
    HalfLeft,
    Stabilized,
    HalfRight,
    FullRight,
}

struct Player {
    speed: f32,
    animation_state: PlayerAnimationState,
}

fn player_keyboard_input_control(
    time: Res<Time>,
    kb_input: Res<Input<KeyCode>>,
    mut player_transforms: Query<(&mut Player, &mut Transform, &mut MoveDuration)>,
) {
    // Get movement direction based on key presses
    let mut y_direction = 0.0;
    let mut x_direction = 0.0;
    if kb_input.pressed(KeyCode::Up) {
        y_direction += 1.0;
    }
    if kb_input.pressed(KeyCode::Down) {
        y_direction -= 1.0;
    }
    if kb_input.pressed(KeyCode::Left) {
        x_direction -= 1.0;
    }
    if kb_input.pressed(KeyCode::Right) {
        x_direction += 1.0;
    }

    for (mut player, mut player_transform, mut player_move_duration) in player_transforms.iter_mut()
    {
        // Change the player's animation state based on the movement
        if x_direction < 0.0 {
            player_move_duration.right = time::Duration::new(0, 0);
            player_move_duration.stabilizing = Default::default();
            if player_move_duration.left >= STABILIZATION_DELAY {
                player.animation_state = PlayerAnimationState::FullLeft;
            } else {
                player.animation_state = PlayerAnimationState::HalfLeft;
                player_move_duration.left += time.delta;
            }
        } else if x_direction > 0.0 {
            player_move_duration.left = Default::default();
            player_move_duration.stabilizing = Default::default();
            if player_move_duration.right >= STABILIZATION_DELAY {
                player.animation_state = PlayerAnimationState::FullRight;
            } else {
                player.animation_state = PlayerAnimationState::HalfRight;
                player_move_duration.right += time.delta;
            }
        } else {
            player_move_duration.left = Default::default();
            player_move_duration.right = Default::default();
            if player_move_duration.stabilizing >= STABILIZATION_DELAY {
                player.animation_state = PlayerAnimationState::Stabilized;
            } else {
                if player.animation_state == PlayerAnimationState::FullRight {
                    player.animation_state = PlayerAnimationState::HalfRight;
                }
                if player.animation_state == PlayerAnimationState::FullLeft {
                    player.animation_state = PlayerAnimationState::HalfLeft;
                }
                player_move_duration.stabilizing += time.delta;
            }
        }

        // Change the player's position state based on the movement
        let player_width = PLAYER_SPRITE_WIDTH * player_transform.scale.x();
        *player_transform.translation.x_mut() += time.delta_seconds * x_direction * player.speed;
        *player_transform.translation.x_mut() = player_transform
            .translation
            .x()
            // update bound
            .min((WINDOW_WIDTH - player_width) / 2.0)
            // lower bound
            .max(-(WINDOW_WIDTH - player_width) / 2.0);
        *player_transform.translation.y_mut() += time.delta_seconds * y_direction * player.speed;
        *player_transform.translation.y_mut() = player_transform
            .translation
            .y()
            // upper bound
            .min((WINDOW_HEIGHT - player_height) / 2.0)
            // lower bound
            .max(-(WINDOW_HEIGHT - player_height) / 2.0);
    }
}

fn player_animation(
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &Player,
        &mut Timer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (player, timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        if timer.finished {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 5) % texture_atlas.textures.len()) as u32;
            match player.animation_state {
                PlayerAnimationState::FullLeft => {
                    if sprite.index != 0 && sprite.index != 5 {
                        sprite.index = 0
                    }
                }
                PlayerAnimationState::HalfLeft => {
                    if sprite.index != 1 && sprite.index != 6 {
                        sprite.index = 1
                    }
                }
                PlayerAnimationState::Stabilized => {
                    if sprite.index != 2 && sprite.index != 7 {
                        sprite.index = 2
                    }
                }
                PlayerAnimationState::HalfRight => {
                    if sprite.index != 3 && sprite.index != 8 {
                        sprite.index = 3
                    }
                }
                PlayerAnimationState::FullRight => {
                    if sprite.index != 4 && sprite.index != 9 {
                        sprite.index = 4
                    }
                }
            }
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let ship_texture_handle = asset_server.load("textures/ship.png");
    let ship_texture_atlas = TextureAtlas::from_grid(
        ship_texture_handle,
        Vec2::new(PLAYER_SPRITE_WIDTH, PLAYER_SPRITE_HEIGHT),
        5,
        2,
    );
    let ship_texture_atlas_handle = texture_atlases.add(ship_texture_atlas);

    commands
        .spawn(Camera2dComponents::default())
        .spawn(SpriteSheetComponents {
            texture_atlas: ship_texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(4.0)),
            ..Default::default()
        })
        .with(Player {
            speed: PLAYER_SPEED,
            animation_state: PlayerAnimationState::Stabilized,
        })
        .with(MoveDuration::default())
        .with(Timer::new(ANIMATION_INTERVAL, true));
}
