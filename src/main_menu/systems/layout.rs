use crate::main_menu::{self, components::*, styles::*};

use bevy::{ecs::query, prelude::*};

pub fn spawn_main_menu(mut cmd: Commands, asset_server: Res<AssetServer>) {
    let main_menu_entity = build_main_menu(&mut cmd, &asset_server);
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

pub fn build_main_menu(cmd: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let main_menu_entity = cmd
        .spawn((
            (
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(8.0),
                    column_gap: Val::Px(8.0),
                    ..Default::default()
                },
                BackgroundColor(NORMAL_BACKGROUND_COLOR),
                MainMenu,
            ),
            children![
                //Tile
                //Start button
                (
                    (
                        normal_button_style_node(),
                        BackgroundColor(NORMAL_BUTTON_COLOR),
                        Button,
                        PlayButton
                    ),
                    // Text
                    children![Text("Start".into())]
                ),
                //Quit button
                (
                    (
                        normal_button_style_node(),
                        BackgroundColor(NORMAL_BUTTON_COLOR),
                        Button,
                        QuitButton
                    ),
                    // Text
                    children![Text("Quit".into())]
                )
            ],
        ))
        .id();

    main_menu_entity
}
