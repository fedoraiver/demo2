use crate::game_play::{
    components::*,
    systems::setup::{CARD_HEIGHT, CARD_WIDTH},
};

use bevy::{prelude::*, render::mesh::*};

const SCALE_FACTOR: f32 = 0.08;
const RESET_FACTOR: f32 = 0.95;

pub fn tilt_card(
    query1: Query<(&Mesh2d, &IsTilting), (With<CardMarker>, Changed<IsTilting>)>,
    query2: Query<&Mesh2d, (With<CardMarker>, Without<IsTilting>)>,
    mut removals: RemovedComponents<IsTilting>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (mesh2d, is_tilting) in query1.iter() {
        if let Some(card_mesh) = meshes.get_mut(mesh2d) {
            if let Some(VertexAttributeValues::Float32x3(positions)) =
                card_mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
            {
                for pos in positions.iter_mut() {
                    let x = if pos[0] > 0.0 {
                        CARD_WIDTH / 2.0
                    } else {
                        -CARD_WIDTH / 2.0
                    };
                    let y = if pos[1] > 0.0 {
                        CARD_HEIGHT / 2.0
                    } else {
                        -CARD_HEIGHT / 2.0
                    };
                    if pos[0] * is_tilting.cursor_from_item_position.x > 0.0
                        && pos[1] * is_tilting.cursor_from_item_position.y > 0.0
                    {
                        pos[0] = (x + is_tilting.cursor_from_item_position.x * SCALE_FACTOR)
                            + (pos[0] - x - is_tilting.cursor_from_item_position.x * SCALE_FACTOR)
                                * RESET_FACTOR;
                        pos[1] = (y + is_tilting.cursor_from_item_position.y * SCALE_FACTOR)
                            + (pos[1] - y - is_tilting.cursor_from_item_position.y * SCALE_FACTOR)
                                * RESET_FACTOR;
                    } else if pos[0] * is_tilting.cursor_from_item_position.x > 0.0
                        && pos[1] * is_tilting.cursor_from_item_position.y < 0.0
                    {
                        pos[0] = x + (pos[0] - x) * RESET_FACTOR;
                        pos[1] = (y + is_tilting.cursor_from_item_position.y * SCALE_FACTOR)
                            + (pos[1] - y - is_tilting.cursor_from_item_position.y * SCALE_FACTOR)
                                * RESET_FACTOR;
                    } else if pos[0] * is_tilting.cursor_from_item_position.x < 0.0
                        && pos[1] * is_tilting.cursor_from_item_position.y > 0.0
                    {
                        pos[0] = (x + is_tilting.cursor_from_item_position.x * SCALE_FACTOR)
                            + (pos[0] - x - is_tilting.cursor_from_item_position.x * SCALE_FACTOR)
                                * RESET_FACTOR;
                        pos[1] = y + (pos[1] - y) * RESET_FACTOR;
                    } else if pos[0] * is_tilting.cursor_from_item_position.x < 0.0
                        && pos[1] * is_tilting.cursor_from_item_position.y < 0.0
                    {
                        pos[0] = x + (pos[0] - x) * RESET_FACTOR;
                        pos[1] = y + (pos[1] - y) * RESET_FACTOR;
                    }
                }
            }
        }
    }

    for entity in removals.read() {
        let mesh2d = query2.get(entity).unwrap();
        if let Some(card_mesh) = meshes.get_mut(mesh2d) {
            if let Some(VertexAttributeValues::Float32x3(positions)) =
                card_mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
            {
                for pos in positions.iter_mut() {
                    let x = if pos[0] > 0.0 {
                        CARD_WIDTH / 2.0
                    } else {
                        -CARD_WIDTH / 2.0
                    };
                    let y = if pos[1] > 0.0 {
                        CARD_HEIGHT / 2.0
                    } else {
                        -CARD_HEIGHT / 2.0
                    };
                    pos[0] = x;
                    pos[1] = y;
                }
            }
        }
    }
}
