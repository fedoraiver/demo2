pub mod components;
pub mod event;
pub mod systems;
pub mod util;

use crate::game_play::components::*;
use crate::game_play::event::*;
use crate::game_play::systems::hovering::*;
use crate::game_play::systems::movement::*;
use crate::game_play::systems::selection::*;
use crate::game_play::systems::setup::*;
use crate::game_play::systems::shaking::*;
use crate::resources::*;
use crate::states::AppState;

use bevy::prelude::*;
use bevy::sprite::Material2dPlugin;

pub struct GamePlayPlugin;

impl Plugin for GamePlayPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CardMarker>();
        app.register_type::<Hoverable>();
        app.register_type::<IsHovering>();
        app.register_type::<Tiltable>();
        app.register_type::<IsTilting>();
        app.register_type::<Selectable>();
        app.register_type::<IsSelected>();
        app.register_type::<Movable>();
        app.register_type::<MovableByCursor>();
        app.register_type::<IsMoving>();
        app.register_type::<MoveBasePosition>();
        app.register_type::<HoverBasePosition>();
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
