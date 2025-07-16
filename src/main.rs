mod components;
mod main_menu;
mod resources;
mod states;
mod systems;

use crate::main_menu::MainMenuPlugin;
use resources::*;
use states::*;
use systems::*;

use bevy::prelude::*;
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
            .set(ImagePlugin::default_nearest()),
    );
    app.init_state::<AppState>();
    app.add_plugins(MainMenuPlugin);
    app.add_plugins(AsepriteUltraPlugin);
    app.add_plugins(HanabiPlugin);
    app.insert_resource(Time::<Fixed>::from_hz(60.0));
    app.init_resource::<CursorWorldPosition>();
    app.init_resource::<ClickWorldPosition>();
    app.add_systems(Startup, register_particle_effect);
    app.add_systems(Startup, setup_camera);
    app.add_systems(OnEnter(AppState::InGame), setup_background);
    app.add_systems(Update, get_cursor_world_position);
    app.add_systems(Update, card_move_by_cursor);
    app.add_systems(Update, (cursor_hover, card_hover).chain());
    app.add_systems(Update, (item_move, cursor_select).chain());
    app.add_systems(Update, exit_game);
    app.run();
}
