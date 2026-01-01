use bevy::{input_focus::InputFocus, prelude::*};

use crate::app::AppState;


#[derive(Component)]
pub struct Menu;

#[derive(Component)]
pub struct StartButton;

#[derive(Component)]
pub struct QuitButton;

#[derive(Component)]
pub struct ResumeButton;

#[derive(Component)]
pub struct BackToMenuButton;

/* ===================== COLORS ===================== */

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

/* ===================== BUTTON SYSTEM ===================== */

pub fn menu_button_system(
    mut input_focus: ResMut<InputFocus>,
    mut query: Query<
        (
            Entity,
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &mut Button,
        ),
        Changed<Interaction>,
    >,
) {
    for (entity, interaction, mut bg, mut border, mut button) in &mut query {
        match *interaction {
            Interaction::Pressed => {
                input_focus.set(entity);
                *bg = PRESSED_BUTTON.into();
                *border = BorderColor::all(Color::WHITE);
                button.set_changed();
            }
            Interaction::Hovered => {
                input_focus.set(entity);
                *bg = HOVERED_BUTTON.into();
                *border = BorderColor::all(Color::WHITE);
                button.set_changed();
            }
            Interaction::None => {
                *bg = NORMAL_BUTTON.into();
                *border = BorderColor::all(Color::BLACK);
            }
        }
    }
}

/* ===================== UI BUILDING BLOCKS ===================== */

fn menu_root() -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.03, 0.03, 0.03)),
        Menu,
    )
}

fn main_menu_panel() -> impl Bundle {
    (
        Node {
            width: Val::Px(420.0),
            padding: UiRect::all(Val::Px(24.0)),
            row_gap: Val::Px(16.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Stretch,
            ..default()
        },
        BackgroundColor(Color::srgb(0.1, 0.1, 0.3)),
        BorderRadius::all(Val::Px(16.0)),
    )
}

fn pause_menu_panel() -> impl Bundle {
    (
        Node {
            width: Val::Px(320.0),
            padding: UiRect::all(Val::Px(20.0)),
            row_gap: Val::Px(12.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Stretch,
            ..default()
        },
        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
        BorderRadius::all(Val::Px(12.0)),
    )
}

fn menu_button(text: &str, assets: &AssetServer) -> impl Bundle {
    (
        Button,
        Node {
            width: Val::Percent(100.),
            height: Val::Px(48.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(NORMAL_BUTTON),
        BorderRadius::all(Val::Px(8.0)),
        BorderColor::all(Color::BLACK),
        children![(
            Text::new(text),
            TextFont {
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::WHITE),
        )],
    )
}

/* ===================== SPAWN MAIN MENU ===================== */

pub fn spawn_main_menu(mut commands: Commands, assets: Res<AssetServer>) {
    commands
        .spawn(menu_root())
        .with_children(|root| {
            root.spawn(main_menu_panel()).with_children(|panel| {
                panel.spawn((menu_button("New Game", &assets), StartButton));
                panel.spawn((menu_button("Exit", &assets), QuitButton));
            });
        });
}

/* ===================== SPAWN PAUSE MENU ===================== */

pub fn spawn_pause_menu(mut commands: Commands, assets: Res<AssetServer>) {
    commands
        .spawn(menu_root())
        .with_children(|root| {
            root.spawn(pause_menu_panel()).with_children(|panel| {
                panel.spawn((menu_button("Resume", &assets), ResumeButton));
                panel.spawn((menu_button("Main Menu", &assets), BackToMenuButton));
                panel.spawn((menu_button("Quit", &assets), QuitButton));
            });
        });
}

/* ===================== BUTTON ACTIONS ===================== */

pub fn menu_buttons(
    mut query: Query<
        (
            &Interaction,
            Option<&StartButton>,
            Option<&QuitButton>,
            Option<&ResumeButton>,
            Option<&BackToMenuButton>,
        ),
        (With<Button>, Changed<Interaction>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, start, quit, resume, back) in &mut query {
        if *interaction == Interaction::Pressed {
            if start.is_some() {
                next_state.set(AppState::InGame);
            }
            if resume.is_some() {
                next_state.set(AppState::InGame);
            }
            if back.is_some() {
                next_state.set(AppState::MainMenu);
            }
            if quit.is_some() {
                exit.write_default();
            }
        }
    }
}

/* ===================== PAUSE INPUT ===================== */

pub fn pause_input(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        match state.get() {
            AppState::InGame => next_state.set(AppState::Paused),
            AppState::Paused => next_state.set(AppState::InGame),
            _ => {}
        }
    }
}

pub fn despawn_menu(mut commands: Commands, query: Query<Entity, With<Menu>>) {
    for e in &query {
        commands.entity(e).despawn();
    }
}
