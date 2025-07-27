mod game_play;
mod main_menu;
mod resources;
mod states;
mod systems;
mod visual_effect;
#[cfg(feature = "bevy_screen_diagnostics_plugin")]
mod diagnostics {
    pub use bevy_screen_diagnostics::*;

    #[cfg(feature = "sysinfo_plugin")]
    pub use bevy_screen_diagnostics::ScreenSystemInformationDiagnosticsPlugin;
}

use crate::game_play::*;
use crate::main_menu::*;
use crate::visual_effect::crt_post_processing::PostProcessPlugin;
use bevy_inspector_egui::bevy_egui::*;
use resources::*;
use states::*;
use systems::*;

use bevy::{log::*, prelude::*};
use bevy_aseprite_ultra::*;
use bevy_hanabi::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

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
    app.add_plugins(EguiPlugin::default());
    app.add_plugins(WorldInspectorPlugin::new());
    #[cfg(feature = "bevy_mod_debugdump_plugin")]
    {
        output_render_graph(&mut app);
        output_schedule_graph(&mut app, Update);
    }

    #[cfg(feature = "bevy_screen_diagnostics_plugin")]
    {
        app.add_plugins(diagnostics::ScreenDiagnosticsPlugin::default())
            .add_plugins(diagnostics::ScreenFrameDiagnosticsPlugin)
            .add_plugins(diagnostics::ScreenEntityDiagnosticsPlugin);

        #[cfg(feature = "sysinfo_plugin")]
        app.add_plugins(diagnostics::ScreenSystemInformationDiagnosticsPlugin);
    }

    app.init_resource::<AsepriteHandle>();
    app.init_resource::<ZIndexManager>();
    app.init_state::<AppState>();

    app.add_plugins(MainMenuPlugin);
    app.add_plugins(AsepriteUltraPlugin);
    app.add_plugins(HanabiPlugin);
    app.add_plugins(GamePlayPlugin);
    app.add_plugins(PostProcessPlugin);

    // app.add_systems(Startup, register_particle_effects);
    app.add_systems(Startup, register_aseprite_assets);
    app.add_systems(Startup, register_my_observers);
    app.add_systems(Update, toggle_pause_state);

    app.run();
}
