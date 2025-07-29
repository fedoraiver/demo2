use crate::game_play::components::*;
use crate::game_play::systems::util::*;

use bevy::ecs::system::*;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

#[derive(Resource, Default)]
pub struct AsepriteHandle {
    pub cards: Handle<Aseprite>,
    pub background: Handle<Aseprite>,
    pub other: Handle<Aseprite>,
}

struct ResetZIndexCommand {
    z_base: f32,
}

impl Command for ResetZIndexCommand {
    fn apply(self, world: &mut World) {
        let transforms: Vec<(Entity, f32)> = {
            let mut query = world.query_filtered::<(Entity, &Transform), With<CardMarker>>();
            let mut entries: Vec<_> = query
                .iter(world)
                .map(|(e, t)| (e, t.translation.z))
                .collect();

            entries.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            entries
        };

        let mut current_z = self.z_base;
        let mut last_original_z = f32::NAN;

        for (entity, original_z) in transforms {
            let is_different = (original_z - last_original_z).abs() > f32::EPSILON;

            if let Some(mut transform) = world.entity_mut(entity).get_mut::<Transform>() {
                transform.translation.z = current_z;
            }

            if is_different {
                current_z += 1.0;
                last_original_z = original_z;
            }
        }

        let mut z_index_manager = world.resource_mut::<ZIndexManager>();
        z_index_manager.z_index = current_z;

        debug!(
            "[ZIndexManager] Z reset complete. z_index = {}",
            z_index_manager.z_index
        );
    }
}

#[derive(Resource, Default)]
pub struct ZIndexManager {
    pub z_index: f32,
}

impl ZIndexManager {
    pub fn next(&mut self, cmd: &mut Commands) -> f32 {
        self.z_index += 1.0;

        if self.z_index >= Z_INDEX_MAX {
            cmd.queue(ResetZIndexCommand { z_base: 1.0 });
            self.z_index += 1.0;
        }

        self.z_index
    }
}

#[derive(Resource, Default)]
pub struct CursorPressedAtItem {
    pub position: Vec2,
}
