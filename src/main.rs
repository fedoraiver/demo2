mod components;
mod resources;
mod systems;

use resources::*;
use systems::*;

use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};
use bevy_aseprite_ultra::prelude::*;
use bevy_hanabi::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(AssetPlugin {
                watch_for_changes_override: Some(true),
                mode: AssetMode::Processed,
                ..Default::default()
            })
            .set(ImagePlugin::default_nearest())
            .set(LogPlugin {
                level: Level::INFO,
                ..Default::default()
            }),
    );
    app.add_plugins(AsepriteUltraPlugin);
    app.add_plugins(HanabiPlugin);
    app.insert_resource(Time::<Fixed>::from_hz(60.0));
    app.init_resource::<CursorWorldPosition>();
    app.init_resource::<ClickWorldPosition>();
    app.add_systems(Startup, setup_particle);
    app.add_systems(Startup, setup_camera);
    app.add_systems(Startup, setup_background);
    app.add_systems(Update, get_cursor_world_position);
    app.add_systems(Update, card_move_by_cursor);
    app.add_systems(Update, (cursor_hover, card_hover).chain());
    app.add_systems(Update, (item_move, cursor_select).chain());
    app.add_systems(Update, exit_game);
    app.run();
}
