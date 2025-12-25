use bevy::prelude::*;

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

pub fn spawn_main_menu(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.3)),
            Menu,
        ))
        .with_children(|parent| {
            parent
                .spawn((Button::default(), StartButton))
                .with_children(|b| {
                    b.spawn((Text::new("New Game"), TextColor::WHITE));
                });
            parent
                .spawn((Button::default(), QuitButton))
                .with_children(|b| {
                    b.spawn((Text::new("Exit"), TextColor::WHITE));
                });
        });
}

pub fn spawn_pause_menu(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0., 1., 0.)),
            Menu,
        ))
        .with_children(|parent| {
            parent
                .spawn((Button::default(), StartButton))
                .with_children(|b| {
                    b.spawn((Text::new("Start"), TextColor::WHITE));
                });
            parent
                .spawn((Button::default(), QuitButton))
                .with_children(|b| {
                    b.spawn((Text::new("Quit"), TextColor::WHITE));
                });
        });
}

pub fn menu_buttons(
    mut commands: Commands,
    mut query: Query<(
        &Interaction,
        Option<&StartButton>,
        Option<&QuitButton>,
        Option<&ResumeButton>,
        Option<&BackToMenuButton>,
    )>,
    mut next_state: ResMut<NextState<AppState>>,
    mut exit: EventWriter<AppExit>,
) {
    for (interactin, start, quit, resume, back) in &mut query {
        if *interactin == Interaction::Pressed {
            if start.is_some() {
                next_state.set(AppState::InGame);
            }
            if resume.is_some() {
                next_state.set(AppState::InGame);
            }
            if back.is_some() {
                next_state.set(AppState::MainMenu);
            }
        }
    }
}

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
