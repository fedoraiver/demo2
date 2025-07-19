use crate::game_play::components::*;

use bevy::prelude::*;
pub fn movement_card(mut query: Query<(&IsMoving, &mut Transform), With<CardMarker>>) {
    for (is_moving, mut transform) in query.iter_mut() {
        *transform = is_moving.target_transform;
    }
}
