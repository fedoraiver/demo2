use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::Srgba(Srgba {
    red: (0.0),
    green: (1.0),
    blue: (0.0),
    alpha: (0.5),
});

pub const NORMAL_BACKGROUND_COLOR: Color = Color::Srgba(Srgba {
    red: (0.0),
    green: (0.0),
    blue: (1.0),
    alpha: (0.5),
});

pub fn normal_button_style_node() -> Node {
    Node {
        width: Val::Px(200.0),
        height: Val::Px(80.0),
        ..Node::DEFAULT
    }
}
