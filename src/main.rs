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
    app.run();
}
