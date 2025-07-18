use crate::game_play::components::*;
use crate::resources::*;

use bevy::prelude::*;

pub fn cursor_hover(
    mut query: Query<(Entity, &Transform, &Sprite), With<Hoverable>>,
    cursor_position: Res<CursorWorldPosition>,
    mut cmd: Commands,
) {
    let mut z_max_entity_opt: Option<Entity> = None;
    let mut z_max = -10.0;
    for (entity, transform, sprite) in query.iter_mut() {
        if (sprite, transform).contains_point(cursor_position.position) {
            if transform.translation.z >= z_max {
                if let Some(z_max_entity) = z_max_entity_opt {
                    cmd.entity(z_max_entity).remove::<Hovering>();
                }
                cmd.entity(entity).insert(Hovering);
                z_max = transform.translation.z;
                z_max_entity_opt = Some(entity);
            }
        } else {
            cmd.entity(entity).remove::<Hovering>();
        }
    }
}

pub fn cursor_select(
    mut query: Query<(Entity, &Transform, &Sprite, Option<&MovableByCursor>), With<Selectable>>,
    cursor_position: Res<CursorWorldPosition>,
    mut click_position: ResMut<ClickWorldPosition>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut cmd: Commands,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let mut z_max_entity_opt: Option<Entity> = None;
        let mut z_max = -10.0;

        click_position.position = cursor_position.position;
        debug!("mouse left press");

        for (entity, transform, sprite, maybe_movable_by_cursor) in query.iter_mut() {
            if (sprite, transform).contains_point(cursor_position.position) {
                if transform.translation.z >= z_max {
                    if let Some(z_max_entity) = z_max_entity_opt {
                        cmd.entity(z_max_entity).remove::<Selected>();
                        cmd.entity(z_max_entity).remove::<IsMoving>();
                    }
                    cmd.entity(entity).insert(Selected);
                    if let Some(mut _movable_by_cursor) = maybe_movable_by_cursor {
                        cmd.entity(entity).insert(IsMoving::new(*transform));
                    }
                    z_max = transform.translation.z;
                    z_max_entity_opt = Some(entity);
                }
            }
        }
        if let Some(z_max_entity) = z_max_entity_opt {
            if let Ok((_entity, transform, _sprite, _movable_by_cursor)) = query.get(z_max_entity) {
                debug!(
                    "Card at position ({}, {}) is now selected",
                    transform.translation.x, transform.translation.y,
                );
            }
        }
    }
}

pub fn cursor_unselect(
    mut query: Query<(Entity, Option<&MovableByCursor>), With<Selected>>,
    mut click_position: ResMut<ClickWorldPosition>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut cmd: Commands,
) {
    if mouse_button_input.just_released(MouseButton::Left) {
        click_position.position = Vec2::ZERO;
        debug!("mouse left release");

        for (entity, maybe_movable_by_cursor) in query.iter_mut() {
            cmd.entity(entity).remove::<Selected>();
            cmd.entity(entity).remove::<BasePosition>();
            if let Some(mut _movable) = maybe_movable_by_cursor {
                cmd.entity(entity).remove::<IsMoving>();
            }
        }
    }
}

pub fn cursor_movement(
    mut query: Query<&mut IsMoving>,
    cursor_position: Res<CursorWorldPosition>,
    cursor_world_posision_last_frame: Res<CursorWorldPositionLastFrame>,
) {
    for mut is_moving in query.iter_mut() {
        is_moving.target_transform = Transform::from_translation(
            (cursor_position.position - cursor_world_posision_last_frame.position).extend(0.0),
        )
        .mul_transform(is_moving.target_transform);
    }
}
