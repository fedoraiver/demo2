use bevy::prelude::*;

pub fn spawn_main_menu(mut cmd: Commands, asset_server: Res<AssetServer>) {
    let main_menu_entity = build_main_menu(&mut cmd, &asset_server);
}

pub fn despawn_main_menu() {}

pub fn build_main_menu(cmd: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let main_menu_entity = cmd
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            BackgroundColor(Color::Srgba(Srgba {
                red: (0.133),
                green: (0.545),
                blue: (0.133),
                alpha: (0.0),
            })),
        ))
        .id();

    main_menu_entity
}
