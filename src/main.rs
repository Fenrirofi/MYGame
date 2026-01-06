use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin, input_focus::InputFocus, prelude::*, window::WindowResolution
};

mod nation;
mod app;
mod menu;
mod time;
mod camera;
mod fps;
mod map;
mod player;

use camera::{CameraControl, ZoomSettings, CameraState, camera_input, smooth_camera};
use time::{GameTime, DateText, game_time_system, update_date_ui, time_controls};
use fps::{FpsText, fps_counter_system};
use app::{AppState};
use menu::{spawn_main_menu, menu_buttons, spawn_pause_menu, pause_input, despawn_menu};
use crate::player::Player;
use crate::{map::{MapData, clamp_camera_system}, menu::menu_button_system};

fn main() {
    App::new()
        // =========================
        // PLUGINS
        // =========================
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    mode: bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                    resolution: WindowResolution::new(1280, 720),
                    title: "My Game".into(),
                    ..default()
                }),
                ..default()
            }),
        )
        .add_plugins(FrameTimeDiagnosticsPlugin::default())

        // =========================
        // STATE
        // =========================
        .init_state::<AppState>()

        // =========================
        // RESOURCES
        // =========================
        .init_resource::<ZoomSettings>()
        .init_resource::<CameraState>()
        .init_resource::<GameTime>()
        .init_resource::<InputFocus>()
        
        .insert_resource(MapData {
            width: 4000.,
            height: 4000.,
            year: 1918,
            camera_margin: 200.,
        })

        // =========================
        // STARTUP
        // =========================
        .add_systems(Startup, setup)

        // =========================
        // IN-GAME SYSTEMS
        // =========================
        .add_systems(
            Update,
            (
                camera_input,
                smooth_camera,
                fps_counter_system,
                clamp_camera_system,
                game_time_system,
                time_controls,
                update_date_ui,
            )
                .run_if(in_state(AppState::InGame)),
        )

        // =========================
        // MENU SYSTEMS
        // =========================
        .add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
        .add_systems(OnExit(AppState::MainMenu), despawn_menu)

        .add_systems(OnEnter(AppState::Paused), spawn_pause_menu)
        .add_systems(OnExit(AppState::Paused), despawn_menu)

        .add_systems(
            Update,
            (
                menu_button_system,
                menu_buttons,
            )
                .run_if(in_menu_states),
        )
        .add_systems(Update, menu_button_system)
        .add_systems(Update, pause_input)

        // =========================
        // RUN
        // =========================
        .run();
}

fn in_menu_states(state: Res<State<AppState>>) -> bool {
    matches!(state.get(), AppState::MainMenu | AppState::Paused)
}

fn setup(mut commands: Commands) {
    // Testowy obiekt
    commands.spawn((
        Sprite::from_color(Color::WHITE, Vec2::new(400.0, 400.0)),
        Transform::default(),
    ));

    // Kamera 2D
    commands.spawn((Camera2d::default(), Player, CameraControl));

    commands.spawn((
        Text::new("FPS: "),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        },
        FpsText,
    ));

    commands.spawn((
        Text::new(""),
        TextFont {
            font_size: 22.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(30.0),
            left: Val::Px(5.0),
            ..default()
        },
        DateText,
    ));
}




