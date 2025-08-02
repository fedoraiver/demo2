use bevy::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AsepriteSliceKey {
    pub bounds: AsepriteSliceRect,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AsepriteSliceRect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[derive(Debug, Deserialize)]
pub struct AsepriteSlice {
    pub name: String,
    pub keys: Vec<AsepriteSliceKey>,
}

#[derive(Debug, Deserialize)]
pub struct AsepriteMeta {
    pub slices: Vec<AsepriteSlice>,
}

#[derive(Debug, Deserialize)]
pub struct AsepriteJson {
    pub meta: AsepriteMeta,
}
