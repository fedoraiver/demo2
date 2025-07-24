pub mod components;
pub mod systems;

use crate::game_play::components::GambleTextMaterial;
use crate::game_play::systems::hover::*;
use crate::game_play::systems::movement::*;
use crate::game_play::systems::util::*;
use crate::states::AppState;

use bevy::prelude::*;
use bevy::sprite::Material2dPlugin;

pub struct GamePlayPlugin;

impl Plugin for GamePlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<GambleTextMaterial>::default());
        app.add_systems(Startup, setup_camera);
        app.add_systems(OnEnter(AppState::InGame), setup_background);

        app.add_systems(
            Update,
            (movement_card, hover_card).run_if(in_state(AppState::InGame)),
        );
    }
}
