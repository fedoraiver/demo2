use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

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
            cmd.spawn((
                AseSlice {
                    name: slice_name,
                    aseprite: cards_handle.clone(),
                },
                Sprite {
                    custom_size: Some(Vec2::new(CARD_WIDTH, CARD_HEIGHT)),
                    ..Default::default()
                },
                Transform::from_xyz(j as f32 * 70.0 - 420.0, i as f32 * 100.0 - 2.0 * 50.0, 0.0),
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

pub fn exit_game(keyboard_input: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}
