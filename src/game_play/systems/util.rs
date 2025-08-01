use crate::game_play::components::*;
use crate::resources::CardsMetadata;
use crate::visual_effect::crt_post_processing::*;

use bevy::prelude::*;

use strum::*;
const CARD_WIDTH: f32 = 64.0;
const CARD_HEIGHT: f32 = 96.0;
const CANVAS_WIDTH: f32 = 1024.0;
const CANVAS_HEIGHT: f32 = 576.0;
const X_SPACING: f32 = 8.0;
const Y_SPACING: f32 = 12.0;
pub const Z_INDEX_MAX: f32 = 1000.0;

pub fn setup_background(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
    cards_metadata: Res<CardsMetadata>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials1: ResMut<Assets<BackgroundMaterial>>,
    mut materials2: ResMut<Assets<GambleTextMaterial>>,
    mut observer_query: Query<&mut Observer>,
) {
    let mesh = Mesh::from(Rectangle::from_size(Vec2::new(CANVAS_WIDTH, CANVAS_HEIGHT)));
    let texture = asset_server.load("images/background.png");
    cmd.spawn((
        Name::new("Background"),
        Mesh2d(meshes.add(mesh)),
        MeshMaterial2d(materials1.add(BackgroundMaterial { texture })),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    let mesh = Mesh::from(Rectangle::from_size(Vec2::new(640.0, 256.0)));
    let texture = asset_server.load("images/gamble_text.png");
    cmd.spawn((
        Name::new("GambleText"),
        Mesh2d(meshes.add(mesh)),
        MeshMaterial2d(materials2.add(GambleTextMaterial { texture })),
        Transform::from_xyz(-200.0, 160.0, 0.25),
        Visibility::Hidden,
    ));
    let start_x = -((CARD_WIDTH + X_SPACING) * 13.0) / 2.0 + (CARD_WIDTH + X_SPACING) / 2.0;
    let start_y = ((CARD_HEIGHT + Y_SPACING) * 4.0) / 2.0 - (CARD_HEIGHT + Y_SPACING) / 2.0;
    for (row, suit) in PokerSuit::iter().enumerate() {
        for (col, point) in PokerPoint::iter().enumerate() {
            let x = start_x + col as f32 * (CARD_WIDTH + X_SPACING);
            let y = start_y - row as f32 * (CARD_HEIGHT + Y_SPACING);
            spawn_poker_card(
                &mut cmd,
                suit,
                point,
                Transform::from_xyz(x, y, 1.0),
                &mut observer_query,
                &asset_server,
                &cards_metadata,
            );
        }
    }
}

pub fn setup_camera(mut cmd: Commands) {
    cmd.spawn((
        Name::new("MainCamera"),
        Camera2d,
        MainCamera,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: bevy::render::camera::ScalingMode::AutoMin {
                min_width: (CANVAS_WIDTH),
                min_height: (CANVAS_HEIGHT),
            },
            near: -Z_INDEX_MAX,
            ..OrthographicProjection::default_2d()
        }),
        PostProcessSettings {
            saturation: 1.8,
            contrast: 1.0,
            gamma: 1.0,
            brightness: 0.03,
        },
    ));
}

pub fn spawn_poker_card(
    cmd: &mut Commands,
    suit: PokerSuit,
    point: PokerPoint,
    transform: Transform,
    observer_query: &mut Query<&mut Observer>,
    asset_server: &Res<AssetServer>,
    cards_metadata: &Res<CardsMetadata>,
) -> Entity {
    let card_name = format!("{}_{}", suit.to_string(), point.to_string());
    let entity = cmd
        .spawn((
            Name::new(format!("Card_{}_{}", suit.to_string(), point.to_string())),
            Sprite {
                custom_size: Some(Vec2::new(CARD_WIDTH, CARD_HEIGHT)),
                image: asset_server.load("images/cards.png"),
                rect: Some(Rect::from_corners(
                    vec2(
                        cards_metadata.hashmap.get(&card_name).unwrap().bounds.x,
                        cards_metadata.hashmap.get(&card_name).unwrap().bounds.y,
                    ),
                    vec2(
                        cards_metadata.hashmap.get(&card_name).unwrap().bounds.x
                            + cards_metadata.hashmap.get(&card_name).unwrap().bounds.w,
                        cards_metadata.hashmap.get(&card_name).unwrap().bounds.y
                            + cards_metadata.hashmap.get(&card_name).unwrap().bounds.h,
                    ),
                )),
                ..Default::default()
            },
            transform,
            Pickable::default(),
            CardMarker,
            Hoverable,
            Selectable,
            MovableByCursor,
            children![(
                Sprite {
                    color: Color::srgba(0.0, 0.0, 0.0, 0.5),
                    custom_size: Some(Vec2::new(CARD_WIDTH, CARD_HEIGHT)),
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
