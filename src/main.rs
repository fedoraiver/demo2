// mod main_menu;
mod systems;

// use crate::main_menu::MainMenuPlugin;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
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
    app.add_systems(Startup, setup_camera);
    app.add_systems(Startup, setup_background);
    // app.add_systems(Startup, setup_card);
    // app.add_systems(Startup, setup_board);
    app.add_systems(Update, exit_game);
    app.run();
}
