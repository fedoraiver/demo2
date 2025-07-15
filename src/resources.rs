use bevy::prelude::*;

/// We will store the world position of the mouse cursor here.
#[derive(Resource, Default)]
pub struct CursorWorldPosition {
    pub position: Vec2,
}
