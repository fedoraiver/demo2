use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::*;
use bevy::sprite::*;
use strum_macros::*;

#[derive(Component)]
pub struct CardMarker;

#[derive(Component, Default)]
pub struct Hoverable;

#[derive(Component)]
#[require(Hoverable)]
pub struct Hovering;

#[derive(Component, Default)]
pub struct Selectable;

#[derive(Component)]
#[require(Selectable)]
pub struct Selected;

#[derive(Component, Default)]
pub struct Movable;

#[derive(Component)]
#[require(Movable)]
pub struct MovableByCursor;

#[derive(Component, Default)]
#[require(Movable)]
pub struct IsMoving {
    pub target_transform: Transform,
}
impl IsMoving {
    pub fn new(transform: Transform) -> Self {
        Self {
            target_transform: transform,
        }
    }
}
#[derive(Component, Debug, Clone, Copy)]
pub struct MoveBasePosition {
    pub position: Vec3,
}
impl MoveBasePosition {
    pub fn new(translation: Vec3) -> Self {
        Self {
            position: translation,
        }
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct HoverBasePosition {
    pub position: Vec3,
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum PokerSuit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum PokerPoint {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl ToString for PokerSuit {
    fn to_string(&self) -> String {
        match self {
            PokerSuit::Clubs => "clubs",
            PokerSuit::Spades => "spades",
            PokerSuit::Diamonds => "diamonds",
            PokerSuit::Hearts => "hearts",
        }
        .to_string()
    }
}

impl ToString for PokerPoint {
    fn to_string(&self) -> String {
        match self {
            PokerPoint::Ace => "ace",
            PokerPoint::Two => "two",
            PokerPoint::Three => "three",
            PokerPoint::Four => "four",
            PokerPoint::Five => "five",
            PokerPoint::Six => "six",
            PokerPoint::Seven => "seven",
            PokerPoint::Eight => "eight",
            PokerPoint::Nine => "nine",
            PokerPoint::Ten => "ten",
            PokerPoint::Jack => "jack",
            PokerPoint::Queen => "queen",
            PokerPoint::King => "king",
        }
        .to_string()
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct GambleTextMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub texture: Handle<Image>,
}

impl Material2d for GambleTextMaterial {
    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/gamble_text.wgsl".into()
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct BackgroundMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub texture: Handle<Image>,
}

impl Material2d for BackgroundMaterial {
    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/background.wgsl".into()
    }
}
