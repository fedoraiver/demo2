use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

#[derive(Resource, Default)]
pub struct CursorWorldPosition {
    pub position: Vec2,
}
#[derive(Resource, Default)]
pub struct CursorWorldPositionLastFrame {
    pub position: Vec2,
}

#[derive(Resource, Default)]
pub struct ClickWorldPosition {
    pub position: Vec2,
}

#[derive(Resource, Default)]
pub struct AsepriteHandle {
    pub cards: Handle<Aseprite>,
    pub background: Handle<Aseprite>,
    pub other: Handle<Aseprite>,
}
