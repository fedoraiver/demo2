use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::Srgba(Srgba {
    red: (0.15),
    green: (0.15),
    blue: (0.15),
    alpha: (0.6),
});

pub const HOVERED_BUTTON_COLOR: Color = Color::Srgba(Srgba {
    red: (0.25),
    green: (0.25),
    blue: (0.25),
    alpha: (0.7),
});

pub const PRESSED_BUTTON_COLOR: Color = Color::Srgba(Srgba {
    red: (0.35),
    green: (0.35),
    blue: (0.35),
    alpha: (0.8),
});

pub const NORMAL_BACKGROUND_COLOR: Color = Color::Srgba(Srgba {
    red: (0.15),
    green: (0.5),
    blue: (0.15),
    alpha: (0.2),
});

pub fn normal_button_style_bundle() -> Node {
    Node {
        width: Val::Px(200.0),
        height: Val::Px(60.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Node::DEFAULT
    }
}

pub fn normal_text_style(text: String) -> (Text, TextLayout) {
    (
        Text(text),
        TextLayout {
            justify: JustifyText::Center,
            ..Default::default()
        },
    )
}
