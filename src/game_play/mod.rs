pub mod components;
pub mod event;
pub mod systems;

use crate::game_play::components::BackgroundMaterial;
use crate::game_play::components::GambleTextMaterial;
use crate::game_play::event::*;
use crate::game_play::systems::hovering::*;
use crate::game_play::systems::movement::*;
use crate::game_play::systems::selection::*;
use crate::game_play::systems::util::*;
use crate::resources::*;
use crate::states::AppState;

use bevy::prelude::*;
use bevy::sprite::Material2dPlugin;

pub struct GamePlayPlugin;

impl Plugin for GamePlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<GambleTextMaterial>::default());
        app.add_plugins(Material2dPlugin::<BackgroundMaterial>::default());
        app.add_systems(Startup, setup_camera);
        app.add_systems(OnEnter(AppState::InGame), setup_background);
        app.add_event::<SelectItem>();
        app.add_event::<UnSelectItem>();
        app.init_resource::<CursorPressedAtItem>();

        app.add_systems(
            Update,
            (move_card, hover_card, select_card).run_if(in_state(AppState::InGame)),
        );
    }
}
