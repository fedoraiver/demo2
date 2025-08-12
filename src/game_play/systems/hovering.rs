use crate::game_play::components::*;

use bevy::prelude::*;

pub fn hover_card_offset(time: f32) -> (f32, f32) {
    let amplitude = 2.0;
    let speed = 2.0;
    (
        (time * speed).sin() as f32 * amplitude,
        (time * speed).cos() as f32 * amplitude * 0.4,
    )
}

pub fn hover_card(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &HoverBasePosition), (With<Card>, With<IsHovering>)>,
) {
    // TODO: hovering 防抖
    let (offset_x, offset_y) = hover_card_offset(time.elapsed_secs());
    for (mut transform, base_position) in query.iter_mut() {
        transform.translation.x = base_position.position.x + offset_x;
        transform.translation.y = base_position.position.y + offset_y;
    }
}
