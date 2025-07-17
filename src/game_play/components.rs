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
    pub delta: Vec2,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct BasePosition {
    pub position: Vec3,
}

#[derive(Component, Debug, Clone, Copy)]
pub enum Shape {
    // Circle { radius: f32 },
    Rect { width: f32, height: f32 },
}

impl Shape {
    pub fn contains_point(&self, &center_point: &Vec2, &test_point: &Vec2) -> bool {
        match self {
            // Shape::Circle { radius } => {
            //     let dx = point_x - self.x;
            //     let dy = point_y - self.y;
            //     dx * dx + dy * dy <= radius * radius
            // }
            Shape::Rect { width, height } => {
                let half_w = width / 2.0;
                let half_h = height / 2.0;
                test_point.x >= center_point.x - half_w
                    && test_point.x <= center_point.x + half_w
                    && test_point.y >= center_point.y - half_h
                    && test_point.y <= center_point.y + half_h
            }
        }
    }
}

#[derive(Component)]
pub struct MainCamera;
