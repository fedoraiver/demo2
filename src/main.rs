// mod main_menu;
mod components;
mod resources;
mod systems;

// use crate::main_menu::MainMenuPlugin;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use resources::*;
use systems::*;

// #[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
// enum AppState {
//     #[default]
//     MainMenu,
//     InGame,
//     Paused,
// }

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
    // app.init_state::<AppState>();
    // app.add_plugins(MainMenuPlugin);
    app.add_plugins(AsepriteUltraPlugin);
    app.insert_resource(Time::<Fixed>::from_hz(1.0));
    app.init_resource::<CursorWorldPosition>();
    app.add_systems(Startup, setup_camera);
    app.add_systems(Startup, setup_background);
    app.add_systems(FixedUpdate, get_cursor_world_position_system);
    app.add_systems(FixedUpdate, cursor_hover_system);
    app.add_systems(FixedUpdate, card_hover_system);
    app.add_systems(Update, exit_game);
    app.run();
}
