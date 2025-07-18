use crate::{game_play::components::*, resources::*};
const ROTATE_PARAMETER: f32 = 8e-6;

use bevy::prelude::*;
pub fn movement_card(
    mut query: Query<(&mut IsMoving, &mut Transform, &mut BasePosition), With<CardMarker>>,
    cursor_position: Res<CursorWorldPosition>,
    cursor_position_last_frame: Res<CursorWorldPositionLastFrame>,
) {
    for (mut is_moving, mut transform, mut base_position) in query.iter_mut() {
        let v = cursor_position.position - cursor_position_last_frame.position;
        let r = cursor_position.position - transform.translation.xy();
        let delta_angle = r.extend(0.0).cross(v.extend(0.0)).z * ROTATE_PARAMETER;

        is_moving.target_transform.rotate_z(delta_angle);

        *transform = is_moving.target_transform;
        // transform.translation += is_moving.target_transform.translation;
        base_position.position = transform.translation;
        // transform.rotate_z(delta_angle);
    }
}
