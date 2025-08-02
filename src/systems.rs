use crate::{game_play::systems::mouse_input_handle::*, resources::*, util::*};

use bevy::prelude::*;
use bevy_hanabi::prelude::*;

use crate::states::AppState;

pub fn toggle_pause_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state_current_state: Res<State<AppState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match app_state_current_state.get() {
            AppState::InGame => app_state_next_state.set(AppState::Paused),
            AppState::Paused => app_state_next_state.set(AppState::InGame),
            _ => {}
        }
    }
}

pub fn register_particle_effects(mut effects: ResMut<Assets<EffectAsset>>) {
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
    let _effect_handle = effects.add(effect);

    // cmd.spawn((
    //     ParticleEffect::new(effect_handle),
    //     Transform::from_xyz(0.0, 0.0, 3.0),
    // ));
}

pub fn register_my_observers(mut cmd: Commands) {
    cmd.spawn((
        Observer::new(cursor_over_at_hoverble_item),
        Name::new("cursor_over_at_hoverble_item_observer"),
    ));

    cmd.spawn((
        Observer::new(mock_cursor_over_at_hoverble_item),
        Name::new("mock_cursor_over_at_hoverble_item_observer"),
    ));

    cmd.spawn((
        Observer::new(cursor_out_at_hoverable_item),
        Name::new("cursor_out_at_hoverable_item_observer"),
    ));

    cmd.spawn((
        Observer::new(cursor_pressed_at_item),
        Name::new("cursor_pressed_at_item"),
    ));

    cmd.spawn((
        Observer::new(mock_cursor_out_at_hoverable_item),
        Name::new("mock_cursor_out_at_hoverable_item_observer"),
    ));

    cmd.spawn((
        Observer::new(cursor_click_at_selectable_item),
        Name::new("cursor_click_at_selectable_item_observer"),
    ));

    cmd.spawn((
        Observer::new(mock_cursor_click_at_selectable_item),
        Name::new("mock_cursor_click_at_selectable_item_observer"),
    ));

    cmd.spawn((
        Observer::new(cursor_drag_start_at_movable_by_cursor_item),
        Name::new("cursor_drag_start_at_movable_by_cursor_item_observer"),
    ));

    cmd.spawn((
        Observer::new(cursor_drag_at_movable_by_cursor_item),
        Name::new("cursor_drag_at_movable_by_cursor_item_observer"),
    ));

    cmd.spawn((
        Observer::new(cursor_drag_end_at_movable_by_cursor_item),
        Name::new("cursor_drag_end_at_movable_by_cursor_item_observer"),
    ));
}

#[cfg(feature = "bevy_mod_debugdump_plugin")]
pub fn output_render_graph(app: &mut App) {
    use bevy_mod_debugdump::*;
    let dot = render_graph_dot(app, &render_graph::Settings::default());
    if let Err(err) = std::fs::write("graph/render_graph.dot", dot) {
        error!("Failed to write render graph: {}", err);
    } else {
        info!("Render graph written to render_graph.dot");
    }
}

#[cfg(feature = "bevy_mod_debugdump_plugin")]
pub fn output_schedule_graph<L: bevy::ecs::schedule::ScheduleLabel>(
    app: &mut App,
    schedule_label: L,
) {
    use bevy_mod_debugdump::*;
    let dot = schedule_graph_dot(app, schedule_label, &schedule_graph::Settings::default());
    if let Err(err) = std::fs::create_dir_all("graph")
        .and_then(|_| std::fs::write("graph/schedule_graph.dot", dot))
    {
        error!("Failed to write schedule graph: {}", err);
    } else {
        info!("Schedule graph written to graph/schedule_graph.dot");
    }
}

pub fn register_cards_aseprite_metadata(mut cmd: Commands) {
    cmd.insert_resource(CardsAsePriteMetadata::from(
        load_aseprite_metadata_from_json("assets/metadata/aseprite_cards.json"),
    ));
}
