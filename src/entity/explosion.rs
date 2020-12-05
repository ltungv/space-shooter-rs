use crate::{
    components::{Animation, Explosion, TimeToLive},
    constant::ANIMATION_INTERVAL,
    resource::GameState,
};
use bevy::prelude::*;

#[derive(Bundle)]
pub struct ExplostionComponents {
    pub explosion: Explosion,
    pub time_to_live: TimeToLive,
    pub animation: Animation,
}

/// Add a new entity to the world with all the needed components to represent a ship
pub fn spawn_explosion(
    mut commands: Commands,
    game_state: Res<GameState>,
    time_to_live: TimeToLive,
    translation: Vec3,
) {
    let texture_atlas = game_state.texture_atlas_handles["explosion"].clone();
    commands
        .spawn(SpriteSheetComponents {
            texture_atlas,
            transform: Transform {
                translation,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_bundle(ExplostionComponents {
            time_to_live,
            explosion: Explosion,
            animation: Animation {
                idx_delta: 1,
                sprite_count: 2,
                timer: Timer::new(ANIMATION_INTERVAL, true),
            },
        });
}
