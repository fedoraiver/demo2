use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_aseprite_ultra::prelude::*;
use bevy_hanabi::prelude::*;

use crate::components::*;
use crate::resources::*;

const SUITS: [&str; 4] = ["spades", "hearts", "clubs", "diamonds"];
const VALUES: [&str; 13] = [
    "ace", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "jack", "queen",
    "king",
];
const CARD_WIDTH: f32 = 64.0;
const CARD_HEIGHT: f32 = 96.0;
const CANVAS_WIDTH: f32 = 1024.0;
const CANVAS_HEIGHT: f32 = 576.0;

pub fn setup_camera(mut cmd: Commands) {
    cmd.spawn((
        Camera2d,
        MainCamera,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: bevy::render::camera::ScalingMode::AutoMin {
                min_width: (CANVAS_WIDTH),
                min_height: (CANVAS_HEIGHT),
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}

pub fn setup_background(mut cmd: Commands, asset_server: Res<AssetServer>) {
    let cards_handle: Handle<Aseprite> = asset_server.load("cards.aseprite");
    let board_handle: Handle<Aseprite> = asset_server.load("board.aseprite");
    let icon_pack_handle: Handle<Aseprite> = asset_server.load("icon_pack.aseprite");
    cmd.spawn((
        AseSlice {
            name: "board".into(),
            aseprite: board_handle.clone(),
        },
        Sprite {
            custom_size: Some(Vec2::new(CANVAS_WIDTH, CANVAS_HEIGHT)),
            ..Default::default()
        },
        Transform::from_xyz(0.0, 0.0, -1.0),
    ));
    for i in 0..SUITS.len() {
        for j in 0..VALUES.len() {
            let slice_name = format!("{}_{}", SUITS[i], VALUES[j]);
            let pos_x = j as f32 * 70.0 - 6.0 * 70.0;
            let pos_y = i as f32 * 100.0 - 1.0 * 100.0;
            cmd.spawn((
                AseSlice {
                    name: slice_name,
                    aseprite: cards_handle.clone(),
                },
                Sprite {
                    custom_size: Some(Vec2::new(CARD_WIDTH, CARD_HEIGHT)),
                    ..Default::default()
                },
                Transform::from_xyz(pos_x, pos_y, 0.0),
                CardMarker,
                Shape::Rect {
                    width: CARD_WIDTH,
                    height: CARD_HEIGHT,
                },
                Hoverable::default(),
                Selectable::default(),
                Movable {
                    movable_by_cursor: true,
                    ..Default::default()
                },
                BasePosition {
                    position: Vec3::new(pos_x, pos_y, 0.0),
                },
            ));
        }
    }
    cmd.spawn((
        AseSlice {
            name: "gamble_text".into(),
            aseprite: icon_pack_handle.clone(),
        },
        Sprite {
            custom_size: Some(Vec2::new(640.0, 256.0)),
            ..Default::default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));
}

pub fn cursor_hover(
    mut query: Query<(&Transform, &Shape, &mut Hoverable)>,
    cursor_position: Res<CursorWorldPosition>,
) {
    for (transform, shape, mut hoverable) in query.iter_mut() {
        if shape.contains_point(&transform.translation.truncate(), &cursor_position.position) {
            hoverable.is_hovering = true;
        } else {
            hoverable.is_hovering = false;
        }
    }
}

pub fn cursor_select(
    mut query: Query<(
        &mut Transform,
        &Shape,
        &mut Selectable,
        Option<&mut Movable>,
        Option<&mut BasePosition>,
    )>,
    cursor_position: Res<CursorWorldPosition>,
    mut click_position: ResMut<ClickWorldPosition>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        click_position.position = cursor_position.position.clone();
        // For Debug
        // info!("mouse left press");

        for (transform, shape, mut selectable, maybe_movable, _maybe_base_position) in
            query.iter_mut()
        {
            if shape.contains_point(&transform.translation.truncate(), &cursor_position.position) {
                selectable.is_selected = true;
                if let Some(mut movable) = maybe_movable {
                    if movable.movable_by_cursor {
                        movable.is_moving = true;
                    }
                }
                // For Debug
                // info!(
                //     "Card at position ({}, {}) is now {}",
                //     transform.translation.x,
                //     transform.translation.y,
                //     if selectable.is_selected {
                //         "selected"
                //     } else {
                //         "deselected"
                //     }
                // );
            }
        }
    }
    if mouse_button_input.just_released(MouseButton::Left) {
        click_position.position = Vec2::ZERO;
        // For Debug
        // info!("mouse left release");

        for (transform, shape, mut selectable, maybe_movable, maybe_base_position) in
            query.iter_mut()
        {
            if shape.contains_point(&transform.translation.truncate(), &cursor_position.position) {
                selectable.is_selected = false;
                if let Some(mut movable) = maybe_movable {
                    if movable.movable_by_cursor {
                        movable.is_moving = false;
                    }
                    if let Some(mut base_position) = maybe_base_position {
                        base_position.position.x = transform.translation.x;
                        base_position.position.y = transform.translation.y;
                    }
                }
                // For Debug
                // info!(
                //     "Card at position ({}, {}) is now {}",
                //     transform.translation.x,
                //     transform.translation.y,
                //     if selectable.is_selected {
                //         "selected"
                //     } else {
                //         "deselected"
                //     }
                // );
            }
        }
    }
}

pub fn get_cursor_world_position(
    mut cursor_world_position: ResMut<CursorWorldPosition>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = match q_camera.single() {
        Ok((camera, transform)) => (camera, transform),
        Err(_) => {
            error!("No main camera found!");
            return;
        }
    };

    let window = match q_window.single() {
        Ok(window) => window,
        Err(_) => {
            error!("No primary window found!");
            return;
        }
    };

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate())
    {
        cursor_world_position.position = world_position;
        // For Debug
        // info!("cursor position: {},{}", world_position.x, world_position.y);
    }
}

pub fn card_hover(
    time: Res<Time>,
    mut query: Query<
        (
            &Hoverable,
            &mut Transform,
            &BasePosition,
            Option<&Selectable>,
        ),
        (With<CardMarker>, Changed<Hoverable>),
    >,
) {
    for (hoverable, mut transform, base_posotion, maybe_selectable) in query.iter_mut() {
        if hoverable.is_hovering {
            // For Debug
            // info!(
            //     "Hovering over card at position: ({}, {})",
            //     transform.translation.x, transform.translation.y
            // );

            if let Some(selectable) = maybe_selectable {
                if selectable.is_selected {
                    return;
                }
            }
            transform.translation.z = 2.0;

            let t = time.elapsed_secs_f64();
            let amplitude = 2.0;
            let speed = 2.0;

            let offset_x = (t * speed).sin() as f32 * amplitude;
            let offset_y = (t * speed).cos() as f32 * amplitude * 0.4;

            transform.translation.x = base_posotion.position.x + offset_x;
            transform.translation.y = base_posotion.position.y + offset_y;
        } else {
            transform.translation.x = base_posotion.position.x;
            transform.translation.y = base_posotion.position.y;
            transform.translation.z = base_posotion.position.z;
        }
    }
}

pub fn card_move_by_cursor(
    mut query: Query<&mut Movable, With<CardMarker>>,
    cursor_position: Res<CursorWorldPosition>,
    click_position: ResMut<ClickWorldPosition>,
) {
    for mut movable in query.iter_mut() {
        if movable.movable_by_cursor && movable.is_moving {
            movable.delta.x = cursor_position.position.x - click_position.position.x;
            movable.delta.y = cursor_position.position.y - click_position.position.y;
            // For Debug
            // info!("Card is moving by cursor to position: ({}, {})", movable.delta.x, movable.delta.y);
        } else {
            movable.delta = Vec2::ZERO;
        }
    }
}

pub fn item_move(mut query: Query<(&Movable, &mut Transform, &BasePosition)>) {
    for (movable, mut transform, base_position) in query.iter_mut() {
        if movable.is_moving {
            transform.translation.x = base_position.position.x + movable.delta.x;
            transform.translation.y = base_position.position.y + movable.delta.y;
        }
    }
}

pub fn exit_game(keyboard_input: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}

pub fn setup_particle(mut cmd: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(1.0, 0.0, 0.0, 1.0));
    gradient.add_key(1.0, Vec4::new(0.0, 0.0, 1.0, 1.0));

    let mut module = Module::default();

    // On spawn, randomly initialize the position of the particle
    // to be over the surface of a sphere of radius 2 units.
    let init_pos = SetPositionSphereModifier {
        center: module.lit(Vec3::new(0.0, 0.0, 0.0)),
        radius: module.lit(2.0),
        dimension: ShapeDimension::Surface,
    };

    // Also initialize a radial initial velocity to 6 units/sec
    // away from the (same) sphere center.
    let init_vel = SetVelocitySphereModifier {
        center: module.lit(Vec3::new(0.0, 0.0, 0.0)),
        speed: module.lit(6.0),
    };

    // Initialize the total lifetime of the particle, that is
    // the time for which it's simulated and rendered. This modifier
    // is almost always required, otherwise the particles won't show.
    let lifetime = module.lit(5.0); // literal value "10.0"
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    // Every frame, add a gravity-like acceleration downward
    // let accel = module.lit(Vec3::new(0., -3.0, 0.0));
    // let update_accel = AccelModifier::new(accel);

    // Create the effect asset
    let effect = EffectAsset::new(
        // Maximum number of particles alive at a time
        32768,
        // Spawn at a rate of 5 particles per second
        SpawnerSettings::rate(50.0.into()),
        // Move the expression module into the asset
        module,
    )
    .with_name("ParticleBlow")
    .init(init_pos)
    .init(init_vel)
    .init(init_lifetime)
    // .update(update_accel)
    // Render the particles with a color gradient over their
    // lifetime. This maps the gradient key 0 to the particle spawn
    // time, and the gradient key 1 to the particle death.
    .render(ColorOverLifetimeModifier {
        gradient,
        ..default()
    });

    // Insert into the asset system
    let effect_handle = effects.add(effect);

    cmd.spawn((
        ParticleEffect::new(effect_handle),
        Transform::from_xyz(0.0, 0.0, 3.0),
    ));
}
