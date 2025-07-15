use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

// 花色和点数定义
const SUITS: [&str; 4] = ["spades", "hearts", "clubs", "diamonds"];
const VALUES: [&str; 13] = [
    "ace", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "jack", "queen",
    "king",
];

fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(AssetPlugin {
                watch_for_changes_override: Some(true),
                mode: AssetMode::Processed,
                ..Default::default()
            })
            .set(ImagePlugin::default_nearest()),
    );
    app.add_plugins(AsepriteUltraPlugin);
    app.add_systems(Startup, setup_camera);
    app.add_systems(Startup, setup_card);
    app.run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: bevy::render::camera::ScalingMode::AutoMin {
                min_width: (1024.0),
                min_height: (1024.0),
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}

fn setup_card(mut commands: Commands, asset_server: Res<AssetServer>) {
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
