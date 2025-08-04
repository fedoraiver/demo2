use bevy::prelude::*;

use crate::game_play::{components::*, event::*};

pub fn select_card(
    mut query: Query<&mut Transform, With<Card>>,
    mut select_event_reader: EventReader<SelectItem>,
    mut unselect_event_reader: EventReader<UnSelectItem>,
) {
    for select_event in select_event_reader.read() {
        if let Ok(mut transform) = query.get_mut(select_event.entity) {
            transform.scale = Vec3::new(1.2, 1.2, 1.0);
        }
    }
    for unselect_event in unselect_event_reader.read() {
        if let Ok(mut transform) = query.get_mut(unselect_event.entity) {
            transform.scale = Vec3::new(1.0, 1.0, 1.0);
        }
    }
}
