use bevy::{prelude::*, diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}};

#[derive(Component)]
pub struct FpsText;

pub fn fps_counter_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    let Ok(mut text) = query.single_mut() else {
        return;
    };

    if let Some(fps) = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|d| d.smoothed())
    {
        text.0 = format!("FPS: {:.0}", fps);
    }
}