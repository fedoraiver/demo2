use bevy::prelude::*;

#[derive(Event)]
pub struct SelectItem {
    pub entity: Entity,
}

impl SelectItem {
    pub fn new(entity: Entity) -> Self {
        Self { entity }
    }
}

#[derive(Event)]
pub struct UnSelectItem {
    pub entity: Entity,
}

impl UnSelectItem {
    pub fn new(entity: Entity) -> Self {
        Self { entity }
    }
}
