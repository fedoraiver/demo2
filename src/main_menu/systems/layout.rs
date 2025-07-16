use crate::main_menu::{components::*, styles::*};

use bevy::prelude::*;

pub fn spawn_main_menu(mut cmd: Commands) {
    build_main_menu(&mut cmd);
}

pub fn despawn_main_menu(mut cmd: Commands, q_main_menu: Query<Entity, With<MainMenu>>) {
    let main_menu_entity = match q_main_menu.single() {
        Ok(main_menu_entity) => main_menu_entity,
        Err(_) => {
            error!("No main menu found!");
            return;
        }
    };
    cmd.entity(main_menu_entity).despawn();
}

pub fn build_main_menu(cmd: &mut Commands) {
    cmd.spawn((
        (
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(10.0),
                column_gap: Val::Px(10.0),
                ..Default::default()
            },
            BackgroundColor(NORMAL_BACKGROUND_COLOR),
            MainMenu,
        ),
        children![
            //Tile
            (
                Node {
                    width: Val::Px(300.0),
                    height: Val::Px(60.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                children![
                    // imgae
                    (),
                    // text
                    (
                        normal_text_style("Poker Automata".into()),
                        TextFont {
                            font_size: 32.0,
                            ..Default::default()
                        }
                    ),
                    // imgae
                    ()
                ]
            ),
            //Play button
            (
                (
                    normal_button_style_bundle(),
                    BackgroundColor(NORMAL_BUTTON_COLOR),
                    Button,
                    PlayButton
                ),
                // Text
                children![normal_text_style("Play".into())]
            ),
            //Quit button
            (
                (
                    normal_button_style_bundle(),
                    BackgroundColor(NORMAL_BUTTON_COLOR),
                    Button,
                    QuitButton
                ),
                // Text
                children![normal_text_style("Quit".into())]
            )
        ],
    ));
    // .id();

    // main_menu_entity
}
