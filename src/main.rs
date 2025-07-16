// mod main_menu;
mod components;
mod resources;
mod systems;

// use crate::main_menu::MainMenuPlugin;
use resources::*;
use systems::*;

use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_hanabi::prelude::*;

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
    app.add_plugins(HanabiPlugin);
    app.insert_resource(Time::<Fixed>::from_hz(60.0));
    app.init_resource::<CursorWorldPosition>();
    app.init_resource::<ClickWorldPosition>();
    app.add_systems(Startup, setup_particle);
    app.add_systems(Startup, setup_camera);
    app.add_systems(Startup, setup_background);
    app.add_systems(Update, get_cursor_world_position);
    app.add_systems(Update, cursor_hover);
    app.add_systems(Update, card_hover);
    app.add_systems(Update, card_move_by_cursor);
    app.add_systems(Update, (item_move, cursor_select).chain());
    app.add_systems(Update, exit_game);
    app.run();
}
