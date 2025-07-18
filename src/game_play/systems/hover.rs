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
    mut cmd: Commands,
    time: Res<Time>,
    mut query1: Query<
        (&mut Transform, &BasePosition),
        (With<CardMarker>, Without<Selected>, With<Hovering>),
    >,
    mut query2: Query<
        (&mut Transform, &BasePosition),
        (With<CardMarker>, Without<Selected>, Without<Hovering>),
    >,
    query3: Query<
        (Entity, &mut Transform),
        (
            With<CardMarker>,
            Without<Selected>,
            With<Hovering>,
            Without<BasePosition>,
        ),
    >,
    mut removed_hovering: RemovedComponents<Hovering>,
) {
    for (mut transform, base_posotion) in query1.iter_mut() {
        trace!(
            "Hovering over card at position: ({}, {})",
            transform.translation.x, transform.translation.y
        );

        transform.translation.z = 2.0;

        let (offset_x, offset_y) = hover_card_offset(time.elapsed_secs());
        transform.translation.x = base_posotion.position.x + offset_x;
        transform.translation.y = base_posotion.position.y + offset_y;
    }
    for entity in removed_hovering.read() {
        if let Ok((mut transform, base_position)) = query2.get_mut(entity) {
            transform.translation.x = base_position.position.x;
            transform.translation.y = base_position.position.y;
            transform.translation.z = base_position.position.z;
            cmd.entity(entity).remove::<BasePosition>();
            debug!("hover over,reset position");
        }
    }
    for (entity, transform) in query3.iter() {
        cmd.entity(entity).insert(BasePosition {
            position: transform.translation,
        });
    }
}
