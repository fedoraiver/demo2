use bevy::prelude::*;

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
