use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

#[derive(Resource, Default)]
pub struct AsepriteHandle {
    pub cards: Handle<Aseprite>,
    pub background: Handle<Aseprite>,
    pub other: Handle<Aseprite>,
}

#[derive(Resource, Default)]
pub struct ZIndexManager {
    pub z_index: f32,
}

impl ZIndexManager {
    pub fn next(&mut self) -> f32 {
        self.z_index += 1.0;
        self.z_index
    }
}

#[derive(Resource, Default)]
pub struct CursorPressedAtItem {
    pub position: Vec2,
}
