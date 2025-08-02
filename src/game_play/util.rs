use bevy::prelude::*;

use crate::{game_play::components::*, resources::*};

pub fn spawn_poker_card(
    suit: PokerSuit,
    point: PokerPoint,
    transform: Transform,
    cmd: &mut Commands,
    observer_query: &mut Query<&mut Observer>,
    asset_server: &Res<AssetServer>,
    cards_metadata: &Res<CardsAsePriteMetadata>,
    meshes: &mut ResMut<Assets<Mesh>>,
    material: &mut ResMut<Assets<MyTextureAtlasMaterial>>,
) -> Entity {
    let card_name = format!("{}_{}", suit.to_string(), point.to_string());
    let card_aseprite_slice_rect = cards_metadata.hashmap.get(&card_name).unwrap();
    // if let Some(VertexAttributeValues::Float32x3(positions)) =
    //     card_mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
    // {
    //     for pos in positions.iter_mut() {
    //         debug!("{},{},{}", pos[0], pos[1], pos[2]);
    //         pos[0] += 12.0;
    //         break;
    //     }
    // }
    let entity = cmd
        .spawn((
            Name::new(format!("Card_{}_{}", suit.to_string(), point.to_string())),
            Mesh2d(meshes.add(Mesh::from(Rectangle::from_size(vec2(
                card_aseprite_slice_rect.w,
                card_aseprite_slice_rect.h,
            ))))),
            MeshMaterial2d(material.add(MyTextureAtlasMaterial {
                texture: asset_server.load("images/cards.png"),
                offset: vec2(card_aseprite_slice_rect.x, card_aseprite_slice_rect.y),
                size: vec2(card_aseprite_slice_rect.w, card_aseprite_slice_rect.h),
                texture_size: cards_metadata.texture_size,
            })),
            transform,
            CardMarker,
            Hoverable,
            Selectable,
            MovableByCursor,
            children![(
                Sprite {
                    color: Color::srgba(0.0, 0.0, 0.0, 0.5),
                    custom_size: Some(Vec2::new(
                        card_aseprite_slice_rect.w,
                        card_aseprite_slice_rect.h
                    )),
                    ..default()
                },
                Transform {
                    translation: Vec3::new(3.0, -3.0, -0.5),
                    ..default()
                },
                Name::new("CardShadow"),
            ),],
        ))
        .id();

    for mut observer in observer_query.iter_mut() {
        observer.watch_entity(entity);
    }

    entity
}

pub fn window_to_world_position(
    window_position: Vec2,
    q_camera: &Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    q_windows: &Query<&Window>,
) -> Vec2 {
    if let Ok(window) = q_windows.single() {
        let ndc = Vec3::new(
            (window_position.x / window.width()) * 2.0,
            (-window_position.y / window.height()) * 2.0,
            0.0,
        );

        if let Ok((camera, camera_transform)) = q_camera.single() {
            if let Some(world_pos) = camera.ndc_to_world(camera_transform, ndc) {
                return world_pos.truncate();
            }
        }
    }
    Vec2::ZERO
}
