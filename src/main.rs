use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(AssetPlugin {
                watch_for_changes_override: Some(true),
                ..Default::default()
            })
            .set(ImagePlugin::default_nearest()),
    );
    app.add_systems(Startup, setup_camera);
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
