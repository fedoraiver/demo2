use crate::{
    game_play::{components::*, systems::util::*},
    resources::*,
};

use bevy::prelude::*;

#[derive(Event)]
pub struct MockPointerOut;
#[derive(Event)]
pub struct MockPointerOver;
#[derive(Event)]
pub struct MockPointerClick;

pub fn cursor_over_on_hoverble_item(
    trigger: Trigger<Pointer<Over>>,
    mut query: Query<(Entity, &mut Transform), With<Hoverable>>,
    mut cmd: Commands,
    mut z_index_manager: ResMut<ZIndexManager>,
    mouse_button: Res<ButtonInput<MouseButton>>,
) {
    if mouse_button.pressed(MouseButton::Left) {
        debug!("Mouse button is pressed, ignoring hover event.");
        return;
    }
    if let Ok((entity, mut transform)) = query.get_mut(trigger.target()) {
        cmd.entity(entity).insert((
            Hovering,
            HoverBasePosition {
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
    trigger: Trigger<MockPointerOver>,
    mut query: Query<(Entity, &mut Transform), With<Hoverable>>,
    mut cmd: Commands,
    mut z_index_manager: ResMut<ZIndexManager>,
) {
    if let Ok((entity, mut transform)) = query.get_mut(trigger.target()) {
        cmd.entity(entity).insert((
            Hovering,
            HoverBasePosition {
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
    mut query: Query<(Entity, &mut Transform, &HoverBasePosition), With<Hovering>>,
    mut cmd: Commands,
) {
    if let Ok((entity, mut transform, base_position)) = query.get_mut(trigger.target()) {
        transform.translation.x = base_position.position.x;
        transform.translation.y = base_position.position.y;
        cmd.entity(entity).remove::<Hovering>();
        cmd.entity(entity).remove::<HoverBasePosition>();
        debug!(
            "Hover out entity, reset position to: ({}, {})",
            base_position.position.x, base_position.position.y
        );
    }
}

pub fn mock_cursor_out_on_hoverable_item(
    trigger: Trigger<MockPointerOut>,
    mut query: Query<(Entity, &mut Transform, &HoverBasePosition), With<Hovering>>,
    mut cmd: Commands,
) {
    if let Ok((entity, mut transform, base_position)) = query.get_mut(trigger.target()) {
        transform.translation.x = base_position.position.x;
        transform.translation.y = base_position.position.y;
        cmd.entity(entity).remove::<Hovering>();
        cmd.entity(entity).remove::<HoverBasePosition>();
        debug!(
            "Hover out entity, reset position to: ({}, {})",
            base_position.position.x, base_position.position.y
        );
    }
}

pub fn cursor_click_on_selectable_item(
    trigger: Trigger<Pointer<Click>>,
    query: Query<Entity, With<Selectable>>,
    selected_query: Query<&Selected>,
    mut cmd: Commands,
) {
    if let Ok(entity) = query.get(trigger.target()) {
        if selected_query.get(entity).is_ok() {
            cmd.entity(entity).remove::<Selected>();
            info!("Entity deselected: {:?}", entity);
        } else {
            cmd.entity(entity).insert(Selected);
            info!("Entity selected: {:?}", entity);
        }
    }
}

pub fn mock_cursor_click_on_selectable_item(
    trigger: Trigger<MockPointerClick>,
    query: Query<Entity, With<Selectable>>,
    selected_query: Query<&Selected>,
    mut cmd: Commands,
) {
    if let Ok(entity) = query.get(trigger.target()) {
        if selected_query.get(entity).is_ok() {
            cmd.entity(entity).remove::<Selected>();
            info!("Entity deselected: {:?}", entity);
        } else {
            cmd.entity(entity).insert(Selected);
            info!("Entity selected: {:?}", entity);
        }
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
        cmd.entity(entity)
            .insert(MoveBasePosition::new(Vec3::from(transform.translation)));
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
        // cmd.entity(entity).remove::<MoveBasePosition>();
        cmd.trigger_targets(MockPointerOver, entity);
        cmd.trigger_targets(MockPointerClick, entity);
        debug!("Stopped dragging entity: {:?}", entity);
    }
}

pub fn cursor_drag_on_movable_by_cursor_item(
    trigger: Trigger<Pointer<Drag>>,
    mut query: Query<&mut IsMoving>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    q_window: Query<&Window>,
) {
    if let Ok(mut is_moving) = query.get_mut(trigger.target()) {
        is_moving.target_transform =
            is_moving
                .target_transform
                .mul_transform(Transform::from_translation(
                    window_to_world_position(trigger.event().delta, &q_camera, &q_window)
                        .extend(0.0),
                ));
        debug!("move event: {:?}", trigger.event);
    }
}
