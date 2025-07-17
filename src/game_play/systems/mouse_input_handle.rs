use crate::game_play::components::*;
use crate::resources::*;

use bevy::prelude::*;

pub fn cursor_hover(
    mut query: Query<(Entity, &Transform, &Shape), With<Hoverable>>,
    cursor_position: Res<CursorWorldPosition>,
    mut cmd: Commands,
) {
    for (entity, transform, shape) in query.iter_mut() {
        if shape.contains_point(&transform.translation.truncate(), &cursor_position.position) {
            cmd.entity(entity).insert(Hovering);
        } else {
            cmd.entity(entity).remove::<Hovering>();
        }
    }
}

pub fn cursor_select(
    mut query: Query<
        (
            Entity,
            &mut Transform,
            &Shape,
            Option<&MovableByCursor>,
            Option<&mut BasePosition>,
        ),
        With<Selectable>,
    >,
    cursor_position: Res<CursorWorldPosition>,
    mut click_position: ResMut<ClickWorldPosition>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut cmd: Commands,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        click_position.position = cursor_position.position.clone();
        debug!("mouse left press");

        for (entity, transform, shape, maybe_movable_by_cursor, _maybe_base_position) in
            query.iter_mut()
        {
            if shape.contains_point(&transform.translation.truncate(), &cursor_position.position) {
                cmd.entity(entity).insert(Selected);
                if let Some(mut _movable_by_cursor) = maybe_movable_by_cursor {
                    cmd.entity(entity).insert(IsMoving::default());
                }
                debug!(
                    "Card at position ({}, {}) is now selected",
                    transform.translation.x, transform.translation.y,
                );
            }
        }
    }
    if mouse_button_input.just_released(MouseButton::Left) {
        click_position.position = Vec2::ZERO;
        debug!("mouse left release");

        for (entity, transform, shape, maybe_movable_by_cursor, maybe_base_position) in
            query.iter_mut()
        {
            if shape.contains_point(&transform.translation.truncate(), &cursor_position.position) {
                cmd.entity(entity).remove::<Selected>();
                if let Some(mut _movable) = maybe_movable_by_cursor {
                    cmd.entity(entity).remove::<IsMoving>();
                    if let Some(mut base_position) = maybe_base_position {
                        base_position.position.x = transform.translation.x;
                        base_position.position.y = transform.translation.y;
                    }
                }
                debug!(
                    "Card at position ({}, {}) is now unselected",
                    transform.translation.x, transform.translation.y
                )
            }
        }
    }
}

pub fn cursor_movement(
    mut query: Query<&mut IsMoving>,
    cursor_position: Res<CursorWorldPosition>,
    click_position: ResMut<ClickWorldPosition>,
) {
    for mut is_moving in query.iter_mut() {
        is_moving.delta.x = cursor_position.position.x - click_position.position.x;
        is_moving.delta.y = cursor_position.position.y - click_position.position.y;
    }
}
