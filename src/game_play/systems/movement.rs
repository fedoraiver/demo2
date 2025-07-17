use crate::game_play::components::*;

use bevy::prelude::*;

pub fn movement_card(
    mut query: Query<(&IsMoving, &mut Transform, &mut BasePosition), (With<CardMarker>)>,
    time: Res<Time>,
) {
    for (is_moving, mut transform, mut base_position) in query.iter_mut() {
        transform.translation += is_moving.target_transform.translation;
        base_position.position = transform.translation;
    }
}
