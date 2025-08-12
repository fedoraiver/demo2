use crate::game_play::{components::*, event::*};

use bevy::prelude::*;
pub fn move_card(
    mut query: Query<&mut Transform, (With<Card>, With<IsMoving>)>,
    mut move_event_reader: EventReader<MoveItem>,
) {
    // TODO: movement 平移光滑 + 旋转 + cursor 隐藏
    for move_event in move_event_reader.read() {
        if let Ok(mut transform) = query.get_mut(move_event.entity) {
            transform.translation += move_event.delta_transform.translation;
        }
    }
}
