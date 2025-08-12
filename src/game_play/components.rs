use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::*;
use bevy::sprite::*;
use strum_macros::*;

#[derive(Component, Reflect)]
pub struct Card;

#[derive(Component, Default, Reflect)]
pub struct Hoverable;

#[derive(Component, Reflect)]
#[require(Hoverable)]
pub struct IsHovering;

#[derive(Component, Default, Reflect)]
pub struct Tiltable;

// TODO: IsTilting 解耦
#[derive(Component, Default, Reflect)]
#[require(Tiltable)]
pub struct IsTilting {
    pub cursor_from_item_position: Vec2,
}

#[derive(Component, Default, Reflect)]
pub struct Selectable;

#[derive(Component, Reflect)]
#[require(Selectable)]
pub struct IsSelected;

#[derive(Component, Default, Reflect)]
pub struct Movable;

#[derive(Component, Reflect)]
#[require(Movable)]
pub struct MovableByCursor;

#[derive(Component, Default, Reflect)]
#[require(Movable)]
pub struct IsMoving;

#[derive(Component, Debug, Clone, Copy, Reflect)]
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

#[derive(Component, Debug, Clone, Copy, Reflect)]
pub struct HoverBasePosition {
    pub position: Vec3,
}

#[derive(Component, Reflect)]
pub struct CardShadow;

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
    Ace,
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
            PokerPoint::Ace => "ace",
        }
        .to_string()
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct BackgroundMaterial {
    #[uniform(0)]
    pub random: Vec3,
}

impl Material2d for BackgroundMaterial {
    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/background.wgsl".into()
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct MyTextureAtlasMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub texture: Handle<Image>,
    #[uniform(2)]
    pub offset: Vec2,
    #[uniform(2)]
    pub size: Vec2,
    #[uniform(2)]
    pub texture_size: Vec2,
}

impl Material2d for MyTextureAtlasMaterial {
    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/my_texture_atlas.wgsl".into()
    }
}
