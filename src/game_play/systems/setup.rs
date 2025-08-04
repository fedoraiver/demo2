use crate::game_play::components::*;
use crate::game_play::util::spawn_poker_card;
use crate::resources::CardsAsePriteMetadata;
use crate::visual_effect::crt_post_processing::*;

use bevy::prelude::*;
use bevy_trauma_shake::*;
use rand::*;

pub const CARD_WIDTH: f32 = 64.0;
pub const CARD_HEIGHT: f32 = 96.0;
const CANVAS_WIDTH: f32 = 1024.0;
const CANVAS_HEIGHT: f32 = 576.0;
pub const Z_INDEX_MAX: f32 = 1000.0;

pub fn setup_background(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
    cards_metadata: Res<CardsAsePriteMetadata>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials1: ResMut<Assets<BackgroundMaterial>>,
    mut materials2: ResMut<Assets<MyTextureAtlasMaterial>>,
    mut materials3: ResMut<Assets<ColorMaterial>>,
    mut observer_query: Query<&mut Observer>,
) {
    let mut rng = rand::thread_rng();
    let random = vec3(
        rng.gen_range(1.0e2..1.0e4),
        rng.gen_range(1.0e2..1.0e4),
        rng.gen_range(1.0e2..1.0e4),
    );
    debug!("{}", random);

    cmd.spawn((
        Name::new("Background"),
        Mesh2d(meshes.add(Mesh::from(Rectangle::from_size(vec2(
            CANVAS_WIDTH,
            CANVAS_HEIGHT,
        ))))),
        MeshMaterial2d(materials1.add(BackgroundMaterial { random: random })),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Pickable::IGNORE,
    ));
    // use strum::*;
    // const X_SPACING: f32 = 8.0;
    // const Y_SPACING: f32 = 12.0;
    // let start_x = -((CARD_WIDTH + X_SPACING) * 13.0) / 2.0 + (CARD_WIDTH + X_SPACING) / 2.0;
    // let start_y = ((CARD_HEIGHT + Y_SPACING) * 4.0) / 2.0 - (CARD_HEIGHT + Y_SPACING) / 2.0;
    // for (row, suit) in PokerSuit::iter().enumerate() {
    //     for (col, point) in PokerPoint::iter().enumerate() {
    //         let x = start_x + col as f32 * (CARD_WIDTH + X_SPACING);
    //         let y = start_y - row as f32 * (CARD_HEIGHT + Y_SPACING);
    //         spawn_poker_card(
    //             suit,
    //             point,
    //             Transform::from_xyz(x, y, 1.0),
    //             &mut cmd,
    //             &mut observer_query,
    //             &asset_server,
    //             &cards_metadata,
    //             &mut meshes,
    //             &mut materials2,
    //         );
    //     }
    // }
    spawn_poker_card(
        PokerSuit::Diamonds,
        PokerPoint::Ace,
        Transform::from_xyz(0.0, 0.0, 1.0),
        &mut cmd,
        &mut observer_query,
        &asset_server,
        &cards_metadata,
        &mut meshes,
        &mut materials2,
        &mut materials3,
    );
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
        Shake::default(),
    ));
}
