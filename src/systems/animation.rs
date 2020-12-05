use crate::components::Animation;
use bevy::prelude::*;

/// Periodically change the index to the sprite in the spritesheet
pub fn texture_atlas_cycle(
    time: Res<Time>,
    mut sprite: Mut<TextureAtlasSprite>,
    mut animation: Mut<Animation>,
) {
    animation.timer.tick(time.delta_seconds);
    if animation.timer.finished {
        sprite.index = (sprite.index + animation.idx_delta) % animation.sprite_count;
    }
}
