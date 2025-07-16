mod components;
mod styles;
mod systems;

use super::states::AppState;
use bevy::prelude::*;
use systems::layout::*;
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        // OnEnter state Systems
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu);
        // OnExit state Systems
        app.add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
    }
}
