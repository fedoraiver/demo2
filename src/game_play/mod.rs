pub mod components;
pub mod systems;

use crate::game_play::systems::hover::*;
use crate::game_play::systems::mouse_input_handle::*;
use crate::game_play::systems::movement::*;
use crate::game_play::systems::util::*;
use crate::states::AppState;

use bevy::prelude::*;

pub struct GamePlayPlugin;

impl Plugin for GamePlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
        app.add_systems(OnEnter(AppState::InGame), setup_background);

        app.add_systems(
            Update,
            (
                get_cursor_world_position,
                cursor_movement,
                cursor_select,
                cursor_unselect,
                movement_card,
                cursor_hover,
                hover_card,
            )
                .run_if(in_state(AppState::InGame)),
        );
    }
}
