use bevy::{input::mouse::AccumulatedMouseScroll, prelude::*, window::PrimaryWindow};

#[derive(Component)]
pub struct CameraControl;

//
// ===== RESOURCES =====
//

#[derive(Resource)]
pub struct ZoomSettings {
    min_scale: f32,
    max_scale: f32,
    zoom_speed: f32,
    pan_speed: f32,
    edge_scroll_margin: f32,
}

impl Default for ZoomSettings {
    fn default() -> Self {
        Self {
            min_scale: 0.3,
            max_scale: 3.0,
            zoom_speed: 0.15,
            pan_speed: 600.0,
            edge_scroll_margin: 5.0,
        }
    }
}

#[derive(Resource)]
pub struct CameraState {
    target_pos: Vec3,
    target_zoom: f32,
}

impl Default for CameraState {
    fn default() -> Self {
        Self {
            target_pos: Vec3::ZERO,
            target_zoom: 1.0,
        }
    }
}

//
// ===== INPUT → TARGET =====
//

pub fn camera_input(
    mut state: ResMut<CameraState>,
    settings: Res<ZoomSettings>,
    keys: Res<ButtonInput<KeyCode>>,
    scroll: Res<AccumulatedMouseScroll>,
    time: Res<Time>,
    windows: Query<&Window, With<PrimaryWindow>>
) {
    // ===== PAN =====
    let mut dir = Vec2::ZERO;

    if keys.pressed(KeyCode::ArrowUp) {
        dir.y += 1.0;
    }
    if keys.pressed(KeyCode::ArrowDown) {
        dir.y -= 1.0;
    }
    if keys.pressed(KeyCode::ArrowLeft) {
        dir.x -= 1.0;
    }
    if keys.pressed(KeyCode::ArrowRight) {
        dir.x += 1.0;
    }
    
    let Ok(window) = windows.single() else {return };
    
    if let Some(cursor) = window.cursor_position() {
        let w = window.height();
        let h = window.height();
        let m = settings.edge_scroll_margin;
        
        if cursor.x <= m {
            dir.x -= 1.0;
        } else if cursor.x >= w - m {
            dir.x += 1.0;
        }

        // Góra / dół
        if cursor.y <= m {
            dir.y += 1.0;
        } else if cursor.y >= h - m {
            dir.y -= 1.0;
        }
    }

    if dir.length_squared() > 0.0 {
        state.target_pos +=
            (dir.normalize() * settings.pan_speed * time.delta_secs()).extend(0.0);
    }

    // ===== ZOOM =====
    if scroll.delta.y != 0.0 {
        state.target_zoom *= 1.0 - scroll.delta.y * settings.zoom_speed;
        state.target_zoom = state
            .target_zoom
            .clamp(settings.min_scale, settings.max_scale);
    }
}

//
// ===== SMOOTH CAMERA =====
//

pub fn smooth_camera(
    mut query: Query<(&mut Transform, &mut Projection), With<CameraControl>>,
    state: Res<CameraState>,
    time: Res<Time>,
) {
    let Ok((mut transform, mut projection)) = query.single_mut() else {
        return;
    };

    // FPS-independent smoothing
    let smooth = 1.0 - (-12.0 * time.delta_secs()).exp();

    // Smooth pan
    transform.translation = transform.translation.lerp(state.target_pos, smooth);

    // Smooth zoom (tylko jeśli ortho)
    if let Projection::Orthographic(ref mut ortho) = *projection {
        ortho.scale = ortho.scale.lerp(state.target_zoom, smooth);
    }
}