use bevy::{ecs::query, prelude::*};

use crate::main_menu::{self, components::MainMenu};

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
                    ..Default::default()
                },
                BackgroundColor(Color::Srgba(Srgba {
                    red: (0.0),
                    green: (0.0),
                    blue: (1.0),
                    alpha: (0.5),
                })),
                MainMenu,
            ),
            children![],
        ))
        .id();

    main_menu_entity
}
