use crate::component::{EnemyVariant, HitBox};
use bevy::prelude::*;

pub struct EnemyData {
    pub variant: EnemyVariant,
    pub hit_box: HitBox,
    pub texture_atlas_handle: Handle<TextureAtlas>,
}
