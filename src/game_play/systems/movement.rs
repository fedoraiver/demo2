use crate::game_play::components::*;

use bevy::prelude::*;
pub fn move_card(mut query: Query<(&IsMoving, &mut Transform), With<Card>>) {
    for (is_moving, mut transform) in query.iter_mut() {
        *transform = is_moving.target_transform;
    }
}
