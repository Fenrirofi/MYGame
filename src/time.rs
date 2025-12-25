use bevy::prelude::*;

#[derive(Component)]
pub struct DateText;

#[derive(Resource)]
pub struct GameTime {
    year: i32,
    month: u32,
    day: u32,

    accumulator: f32, // zbiera sekundy
    speed: f32,       // 0.0 = pause, 1.0 = normal, 5.0 = fast
}

impl Default for GameTime {
    fn default() -> Self {
        Self {
            year: 1960,
            month: 1,
            day: 12,
            accumulator: 0.0,
            speed: 1.0,
        }
    }
}

pub fn game_time_system(
    mut game_time: ResMut<GameTime>,
    time: Res<Time>,
) {
    if game_time.speed <= 0.0 {
        return; // pauza
    }

    game_time.accumulator += time.delta_secs() * game_time.speed;

    while game_time.accumulator >= 1.0 {
        game_time.accumulator -= 1.0;
        advance_day(&mut game_time);
    }
}

pub fn update_date_ui(
    game_time: Res<GameTime>,
    mut query: Query<&mut Text, With<DateText>>,
) {
    let Ok(mut text) = query.single_mut() else {
        return;
    };

    let month_name = match game_time.month {
        1 => "stycznia",
        2 => "lutego",
        3 => "marca",
        4 => "kwietnia",
        5 => "maja",
        6 => "czerwca",
        7 => "lipca",
        8 => "sierpnia",
        9 => "września",
        10 => "października",
        11 => "listopada",
        12 => "grudnia",
        _ => "",
    };

    text.0 = format!(
        "{} {} {}",
        game_time.day, month_name, game_time.year
    );
}

pub fn advance_day(time: &mut GameTime) {
    time.day += 1;

    let days_in_month = match time.month {
        1 => 31,
        2 => 28, // uproszczone, bez lat przestępnych
        3 => 31,
        4 => 30,
        5 => 31,
        6 => 30,
        7 => 31,
        8 => 31,
        9 => 30,
        10 => 31,
        11 => 30,
        12 => 31,
        _ => 30,
    };

    if time.day > days_in_month {
        time.day = 1;
        time.month += 1;

        if time.month > 12 {
            time.month = 1;
            time.year += 1;
        }
    }
}

pub fn time_controls(
    mut game_time: ResMut<GameTime>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        game_time.speed = if game_time.speed > 0.0 { 0.0 } else { 1.0 };
    }

    if keys.just_pressed(KeyCode::Numpad1) {
        game_time.speed = 1.0;
    }
    if keys.just_pressed(KeyCode::Numpad2) {
        game_time.speed = 3.0;
    }
    if keys.just_pressed(KeyCode::Numpad3) {
        game_time.speed = 10.0;
    }
}

