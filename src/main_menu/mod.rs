mod components;
mod styles;
mod systems;

use crate::main_menu::systems::interactions::*;

use super::states::AppState;
use bevy::prelude::*;
use systems::layout::*;
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu);
        app.add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
        app.add_systems(
            Update,
            (interact_with_play_button, interact_with_quit_button)
                .run_if(in_state(AppState::MainMenu)),
        );
    }
}
