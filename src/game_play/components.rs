use bevy::prelude::*;

#[derive(Component)]
pub struct CardMarker;

#[derive(Component, Default)]
pub struct Hoverable;

#[derive(Component)]
#[require(Hoverable)]
pub struct Hovering;

#[derive(Component, Default)]
pub struct Selectable;

#[derive(Component)]
#[require(Selectable)]
pub struct Selected;

#[derive(Component, Default)]
pub struct Movable;

#[derive(Component)]
#[require(Movable)]
pub struct MovableByCursor;

#[derive(Component, Default)]
#[require(Movable)]
pub struct IsMoving {
    pub target_transform: Transform,
}
impl IsMoving {
    pub fn new(transform: Transform) -> Self {
        Self {
            target_transform: Transform {
                translation: Vec3::ZERO,
                rotation: transform.rotation,
                scale: transform.scale,
            },
        }
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct BasePosition {
    pub position: Vec3,
}

#[derive(Component)]
pub struct MainCamera;

pub trait ContainsPoint {
    fn contains_point(&self, point: Vec2) -> bool;
}

impl ContainsPoint for (&Sprite, &Transform) {
    fn contains_point(&self, point: Vec2) -> bool {
        let (sprite, transform) = *self;

        let size = match sprite.custom_size {
            Some(size) => size,
            None => {
                error!("Sprite missing custom_size");
                return false;
            }
        };

        let half_size = size / 2.0;

        let local_point = transform
            .compute_matrix()
            .inverse()
            .transform_point(point.extend(0.0));

        local_point.x >= -half_size.x
            && local_point.x <= half_size.x
            && local_point.y >= -half_size.y
            && local_point.y <= half_size.y
    }
}
