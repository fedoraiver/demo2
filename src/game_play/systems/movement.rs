use crate::game_play::components::*;

use bevy::prelude::*;

pub fn movement_item(mut query: Query<(&IsMoving, &mut Transform, &BasePosition)>) {
    for (is_moving, mut transform, base_position) in query.iter_mut() {
        transform.translation.x = base_position.position.x + is_moving.delta.x;
        transform.translation.y = base_position.position.y + is_moving.delta.y;
    }
}
