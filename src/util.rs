use std::collections::HashMap;

use bevy::prelude::*;

use crate::{component::*, resources::*};

pub fn load_aseprite_metadata_from_json(json_path: &str) -> AsePriteMetadata {
    let file = std::fs::File::open(json_path).expect("Failed to open Aseprite JSON");
    let json: AsepriteJson = serde_json::from_reader(file).expect("Failed to parse JSON");

    let mut map = HashMap::new();

    for slice in json.meta.slices {
        if let Some(key) = slice.keys.first() {
            map.insert(slice.name.clone(), key.bounds.clone());
        }
    }

    let texture_size = json.meta.size;

    AsePriteMetadata {
        hashmap: map,
        texture_size: vec2(texture_size.w, texture_size.h),
    }
}
