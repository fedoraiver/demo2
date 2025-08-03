use crate::game_play::{
    components::*,
    systems::setup::{CARD_HEIGHT, CARD_WIDTH},
    util::*,
};

use bevy::{prelude::*, render::mesh::*};

const SCALE_FACTOR: f32 = 0.125;

pub fn tilt_card(
    query: Query<(&Mesh2d, &IsTilting), (With<CardMarker>, Changed<IsTilting>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    q_window: Query<&Window>,
) {
    for (mesh2d, is_tilting) in query.iter() {
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
                        let delta =
                            window_to_world_position(is_tilting.delta, &q_camera, &q_window);
                        pos[0] += delta.x * SCALE_FACTOR;
                        pos[1] += delta.y * SCALE_FACTOR;
                    } else {
                        pos[0] = x;
                        pos[1] = y;
                    }
                }
            }
        }
    }
}
