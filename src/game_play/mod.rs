pub mod components;
pub mod event;
pub mod systems;

use crate::game_play::components::*;
use crate::game_play::event::*;
use crate::game_play::systems::hovering::*;
use crate::game_play::systems::movement::*;
use crate::game_play::systems::selection::*;
use crate::game_play::systems::shaking::*;
use crate::game_play::systems::util::*;
use crate::resources::*;
use crate::states::AppState;

use bevy::prelude::*;
use bevy::sprite::Material2dPlugin;

pub struct GamePlayPlugin;

impl Plugin for GamePlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MeshPickingPlugin);
        app.add_plugins(Material2dPlugin::<BackgroundMaterial>::default());
        app.add_plugins(Material2dPlugin::<MyTextureAtlasMaterial>::default());
        app.add_systems(Startup, setup_camera);
        app.add_systems(OnEnter(AppState::InGame), setup_background);
        app.add_event::<SelectItem>();
        app.add_event::<UnSelectItem>();
        app.init_resource::<CursorPressedAtItem>();

        app.add_systems(
            Update,
            (move_card, hover_card, select_card, camera_shake).run_if(in_state(AppState::InGame)),
        );
    }
}
