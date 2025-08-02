use std::collections::HashMap;

use crate::component::*;

pub fn load_card_aseprite_metadata_from_json(
    json_path: &str,
) -> HashMap<String, AsepriteSliceRect> {
    let file = std::fs::File::open(json_path).expect("Failed to open Aseprite JSON");
    let json: AsepriteJson = serde_json::from_reader(file).expect("Failed to parse JSON");

    let mut map = HashMap::new();

    for slice in json.meta.slices {
        if let Some(key) = slice.keys.first() {
            map.insert(slice.name.clone(), key.bounds.clone());
        }
    }

    map
}
