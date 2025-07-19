use crate::game_play::components::*;
use crate::resources::*;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_aseprite_ultra::prelude::*;
use strum::*;

const CARD_WIDTH: f32 = 64.0;
const CARD_HEIGHT: f32 = 96.0;
const CANVAS_WIDTH: f32 = 1024.0;
const CANVAS_HEIGHT: f32 = 576.0;
const X_SPACING: f32 = 8.0;
const Y_SPACING: f32 = 12.0;

pub fn setup_background(mut cmd: Commands, aseprite_handle: Res<AsepriteHandle>) {
    cmd.spawn((
        AseSlice {
            name: "board".into(),
            aseprite: aseprite_handle.background.clone(),
        },
        Sprite {
            custom_size: Some(Vec2::new(CANVAS_WIDTH, CANVAS_HEIGHT)),
            ..Default::default()
        },
        Transform::from_xyz(0.0, 0.0, -1.0),
    ));
    cmd.spawn((
        AseSlice {
            name: "gamble_text".into(),
            aseprite: aseprite_handle.other.clone(),
        },
        Sprite {
            custom_size: Some(Vec2::new(640.0, 256.0)),
            ..Default::default()
        },
        Transform::from_xyz(0.0, 100.0, 2.0),
    ));
    let start_x = -((CARD_WIDTH + X_SPACING) * 13.0) / 2.0 + (CARD_WIDTH + X_SPACING) / 2.0;
    let start_y = ((CARD_HEIGHT + Y_SPACING) * 4.0) / 2.0 - (CARD_HEIGHT + Y_SPACING) / 2.0;
    for (row, suit) in PokerSuit::iter().enumerate() {
        for (col, point) in PokerPoint::iter().enumerate() {
            let x = start_x + col as f32 * (CARD_WIDTH + X_SPACING);
            let y = start_y - row as f32 * (CARD_HEIGHT + Y_SPACING);
            spawn_poker_card(
                &mut cmd,
                aseprite_handle.cards.clone(),
                suit,
                point,
                Transform::from_xyz(x, y, 0.0),
            );
        }
    }
}

pub fn setup_camera(mut cmd: Commands) {
    cmd.spawn((
        Camera2d,
        MainCamera,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: bevy::render::camera::ScalingMode::AutoMin {
                min_width: (CANVAS_WIDTH),
                min_height: (CANVAS_HEIGHT),
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}

pub fn get_cursor_world_position(
    mut cursor_world_position: ResMut<CursorWorldPosition>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = match q_camera.single() {
        Ok((camera, transform)) => (camera, transform),
        Err(_) => {
            error!("No main camera found!");
            return;
        }
    };

    let window = match q_window.single() {
        Ok(window) => window,
        Err(_) => {
            error!("No primary window found!");
            return;
        }
    };

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate())
    {
        cursor_world_position.position = world_position;
        trace!("cursor position: {},{}", world_position.x, world_position.y);
    }
}

pub fn update_cursor_positon_last_frame(
    cursor_position: Res<CursorWorldPosition>,
    mut cursor_position_last_frame: ResMut<CursorWorldPositionLastFrame>,
) {
    cursor_position_last_frame.position = cursor_position.position;
}

pub fn spawn_poker_card(
    cmd: &mut Commands,
    aseprite_handle: Handle<Aseprite>,
    suit: PokerSuit,
    point: PokerPoint,
    transform: Transform,
) -> Entity {
    let slice_name = format!("{}_{}", suit.to_string(), point.to_string());
    cmd.spawn((
        AseSlice {
            name: slice_name.into(),
            aseprite: aseprite_handle,
        },
        Sprite {
            custom_size: Some(Vec2::new(CARD_WIDTH, CARD_HEIGHT)),
            ..Default::default()
        },
        transform,
        CardMarker,
        Hoverable,
        Selectable,
        MovableByCursor,
    ))
    .id()
}
