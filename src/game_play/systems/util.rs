use crate::game_play::components::*;
use crate::game_play::systems::mouse_input_handle::*;
use crate::resources::*;

use bevy::prelude::*;
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
        Pickable::default(),
        CardMarker,
        Hoverable,
        Selectable,
        MovableByCursor,
    ))
    .observe(cursor_over_on_hoverble_item)
    .observe(mock_cursor_over_on_hoverble_item)
    .observe(cursor_out_on_hoverable_item)
    .observe(mock_cursor_out_on_hoverable_item)
    .observe(cursor_click_on_selectable_item)
    .observe(cursor_drag_start_on_movable_by_cursor_item)
    .observe(cursor_move_on_movable_by_cursor_item)
    .observe(cursor_drag_end_on_movable_by_cursor_item)
    .id()
}
