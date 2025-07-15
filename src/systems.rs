use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_aseprite_ultra::prelude::*;

use crate::components::*;
use crate::resources::*;

const SUITS: [&str; 4] = ["spades", "hearts", "clubs", "diamonds"];
const VALUES: [&str; 13] = [
    "ace", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "jack", "queen",
    "king",
];
const CARD_WIDTH: f32 = 64.0;
const CARD_HEIGHT: f32 = 96.0;
const CANVAS_WIDTH: f32 = 1024.0;
const CANVAS_HEIGHT: f32 = 576.0;

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

pub fn setup_background(mut cmd: Commands, asset_server: Res<AssetServer>) {
    let cards_handle: Handle<Aseprite> = asset_server.load("cards.aseprite");
    let board_handle: Handle<Aseprite> = asset_server.load("board.aseprite");
    let icon_pack_handle: Handle<Aseprite> = asset_server.load("icon_pack.aseprite");
    cmd.spawn((
        AseSlice {
            name: "board".into(),
            aseprite: board_handle.clone(),
        },
        Sprite {
            custom_size: Some(Vec2::new(CANVAS_WIDTH, CANVAS_HEIGHT)),
            ..Default::default()
        },
        Transform::from_xyz(0.0, 0.0, -1.0),
    ));
    for i in 0..SUITS.len() {
        for j in 0..VALUES.len() {
            let slice_name = format!("{}_{}", SUITS[i], VALUES[j]);
            let pos_x = j as f32 * 70.0 - 6.0 * 70.0;
            let pos_y = i as f32 * 100.0 - 2.5 * 50.0;
            cmd.spawn((
                AseSlice {
                    name: slice_name,
                    aseprite: cards_handle.clone(),
                },
                Sprite {
                    custom_size: Some(Vec2::new(CARD_WIDTH, CARD_HEIGHT)),
                    ..Default::default()
                },
                Transform::from_xyz(pos_x, pos_y, 0.0),
                CardMarker,
                Shape::Rect {
                    width: CARD_WIDTH,
                    height: CARD_HEIGHT,
                },
                Hoverable { is_hovering: false },
            ));
        }
    }
    cmd.spawn((
        AseSlice {
            name: "gamble_text".into(),
            aseprite: icon_pack_handle.clone(),
        },
        Sprite {
            custom_size: Some(Vec2::new(640.0, 256.0)),
            ..Default::default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));
}

pub fn cursor_hover_system(
    mut query: Query<(&Transform, &Shape, &mut Hoverable)>,
    cursor_position: Res<CursorWorldPosition>,
) {
    for (transform, shape, mut hoverable) in query.iter_mut() {
        let cursor_pos = cursor_position.position;
        if shape.contains_point(
            transform.translation.truncate(),
            Vec2 {
                x: cursor_pos.x,
                y: cursor_pos.y,
            },
        ) {
            hoverable.is_hovering = true;
        } else {
            hoverable.is_hovering = false;
        }
    }
}

pub fn get_cursor_world_position_system(
    mut cursor_world_position: ResMut<CursorWorldPosition>,
    // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = match q_camera.single() {
        Ok((camera, transform)) => (camera, transform),
        Err(_) => {
            error!("No main camera found!");
            return;
        }
    };

    // There is only one primary window, so we can similarly get it from the query:
    let window = match q_window.single() {
        Ok(window) => window,
        Err(_) => {
            error!("No primary window found!");
            return;
        }
    };

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate())
    {
        cursor_world_position.position = world_position;
        // For Debug
        // info!("cursor position: {},{}", world_position.x, world_position.y);
    }
}

pub fn card_hover_system(mut query: Query<(&Hoverable, &Transform), With<CardMarker>>) {
    for (hoverable, transform) in query.iter() {
        if hoverable.is_hovering {
            // For Debug
            // info!(
            //     "Hovering over card at position: ({}, {})",
            //     transform.translation.x, transform.translation.y
            // );

            // To-Do: Hover logic:上下左右循环移动
        }
    }
}

pub fn exit_game(keyboard_input: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}
