use crate::{game_play::components::*, resources::*};

use bevy::prelude::*;

#[derive(Event)]
pub struct MockPointerOut;
#[derive(Event)]
pub struct MockPointerover;

pub fn cursor_over_on_hoverble_item(
    trigger: Trigger<Pointer<Over>>,
    mut query: Query<(Entity, &mut Transform), With<Hoverable>>,
    mut cmd: Commands,
    mut z_index_manager: ResMut<ZIndexManager>,
) {
    if let Ok((entity, mut transform)) = query.get_mut(trigger.target()) {
        cmd.entity(entity).insert((
            Hovering,
            BasePosition {
                position: transform.translation,
            },
        ));
        transform.translation.z = z_index_manager.next();
        debug!(
            "Hovering over entity at position: ({}, {})",
            transform.translation.x, transform.translation.y
        );
    }
}

pub fn mock_cursor_over_on_hoverble_item(
    trigger: Trigger<MockPointerover>,
    mut query: Query<(Entity, &mut Transform), With<Hoverable>>,
    mut cmd: Commands,
    mut z_index_manager: ResMut<ZIndexManager>,
) {
    if let Ok((entity, mut transform)) = query.get_mut(trigger.target()) {
        cmd.entity(entity).insert((
            Hovering,
            BasePosition {
                position: transform.translation,
            },
        ));
        transform.translation.z = z_index_manager.next();
        debug!(
            "Hovering over entity at position: ({}, {})",
            transform.translation.x, transform.translation.y
        );
    }
}

pub fn cursor_out_on_hoverable_item(
    trigger: Trigger<Pointer<Out>>,
    mut query: Query<(Entity, &mut Transform, &BasePosition), With<Hovering>>,
    mut cmd: Commands,
) {
    if let Ok((entity, mut transform, base_position)) = query.get_mut(trigger.target()) {
        transform.translation.x = base_position.position.x;
        transform.translation.y = base_position.position.y;
        cmd.entity(entity).remove::<Hovering>();
        cmd.entity(entity).remove::<BasePosition>();
        debug!(
            "Hover out entity, reset position to: ({}, {})",
            base_position.position.x, base_position.position.y
        );
    }
}

pub fn mock_cursor_out_on_hoverable_item(
    trigger: Trigger<MockPointerOut>,
    mut query: Query<(Entity, &mut Transform, &BasePosition), With<Hovering>>,
    mut cmd: Commands,
) {
    if let Ok((entity, mut transform, base_position)) = query.get_mut(trigger.target()) {
        transform.translation.x = base_position.position.x;
        transform.translation.y = base_position.position.y;
        cmd.entity(entity).remove::<Hovering>();
        cmd.entity(entity).remove::<BasePosition>();
        debug!(
            "Hover out entity, reset position to: ({}, {})",
            base_position.position.x, base_position.position.y
        );
    }
}

pub fn cursor_click_on_selectable_item(
    trigger: Trigger<Pointer<Click>>,
    mut query: Query<Entity, With<Selectable>>,
    mut cmd: Commands,
) {
    if let Ok(entity) = query.get_mut(trigger.target()) {
        cmd.entity(entity).insert(Selected);
        debug!("Entity selected: {:?}", entity);
    }
}

pub fn cursor_drag_start_on_movable_by_cursor_item(
    trigger: Trigger<Pointer<DragStart>>,
    mut query: Query<(Entity, &Transform), With<MovableByCursor>>,
    mut cmd: Commands,
) {
    if let Ok((entity, transform)) = query.get_mut(trigger.target()) {
        cmd.trigger_targets(MockPointerOut, entity);
        cmd.entity(entity).insert(IsMoving::new(transform.clone()));
        debug!("Started dragging entity: {:?}", entity);
    }
}

pub fn cursor_drag_end_on_movable_by_cursor_item(
    trigger: Trigger<Pointer<DragEnd>>,
    mut query: Query<Entity, With<IsMoving>>,
    mut cmd: Commands,
) {
    if let Ok(entity) = query.get_mut(trigger.target()) {
        cmd.entity(entity).remove::<IsMoving>();
        cmd.trigger_targets(MockPointerover, entity);
        debug!("Stopped dragging entity: {:?}", entity);
    }
}

pub fn cursor_move_on_movable_by_cursor_item(
    trigger: Trigger<Pointer<Move>>,
    mut query: Query<&mut IsMoving>,
) {
    if let Ok(mut is_moving) = query.get_mut(trigger.target()) {
        is_moving.target_transform =
            Transform::from_translation(trigger.event.hit.position.unwrap());
    }
    debug!("move event: {:?}", trigger.event);
}
