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

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
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

pub fn setup_card(mut commands: Commands, asset_server: Res<AssetServer>) {
    let cards_handle: Handle<Aseprite> = asset_server.load("cards.aseprite");
    for i in 0..SUITS.len() {
        for j in 0..VALUES.len() {
            let slice_name = format!("{}_{}", SUITS[i], VALUES[j]);
            commands.spawn((
                AseSlice {
                    name: slice_name,
                    aseprite: cards_handle.clone(),
                },
                Sprite {
                    custom_size: Some(Vec2::new(CARD_WIDTH, CARD_HEIGHT)),
                    ..Default::default()
                },
                Transform::from_xyz(
                    j as f32 * 70.0 - 13.0 * 35.0,
                    i as f32 * 100.0 - 2.0 * 50.0,
                    0.0,
                ),
            ));
        }
    }
}

pub fn setup_board(mut cmd: Commands, asset_server: Res<AssetServer>) {
    cmd.spawn((
        AseSlice {
            name: "board".into(),
            aseprite: asset_server.load("board.aseprite"),
        },
        Sprite {
            custom_size: Some(Vec2::new(CANVAS_WIDTH, CANVAS_HEIGHT)),
            ..Default::default()
        },
        Transform::from_xyz(0.0, 0.0, -1.0),
    ));
}

pub fn exit_game(keyboard_input: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}
