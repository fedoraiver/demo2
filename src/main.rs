use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

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
    commands.spawn((
        AseSlice {
            name: "diamonds_ace".into(),
            aseprite: asset_server.load("cards.aseprite"),
        },
        Sprite {
            ..Default::default()
        },
    ));
}
