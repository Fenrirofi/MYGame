use std::{f32::consts::PI, ops::Range};
use bevy::{input::mouse::{AccumulatedMouseScroll, MouseWheel}, prelude::{OrthographicProjection, *}};

mod map;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<ZoomSettings>()
        .add_systems(Startup, setup)
        .add_systems(Update, zoom)
        .run();
}

#[derive(Resource)]
struct ZoomSettings {
    pub min_scale: f32,
    pub max_scale: f32,
    pub speed: f32,
}

impl Default for ZoomSettings {
    fn default() -> Self {
        Self {
            min_scale: 0.1, // Maksymalne przybliżenie
            max_scale: 3.0, // Maksymalne oddalenie
            speed: 0.5,     // Czułość scrolla
        }
    }
}

fn setup(
    mut commands: Commands,
    mut _meshes: ResMut<Assets<Mesh>>,
    mut _material: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Sprite::from_color(Color::WHITE, Vec2::new(200., 200.)),
        Transform {
            translation: Vec3::new(0., 0., 0.),
            scale: Vec3::ONE,
            ..Default::default()
        },
    ));

    commands.spawn((
        Camera2d::default(),
    ));
    
    commands.spawn((
        Text::new("Uzyj kolka myszy, aby przyblizac/oddalac"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
}

fn zoom(
    camera: Single<&mut Projection, With<Camera>>,
    camera_settings: Res<ZoomSettings>,
    mouse_wheel_input: Res<AccumulatedMouseScroll>,
) {
    // Usually, you won't need to handle both types of projection,
    // but doing so makes for a more complete example.
    match *camera.into_inner() {
        Projection::Orthographic(ref mut orthographic) => {
            // We want scrolling up to zoom in, decreasing the scale, so we negate the delta.
            let delta_zoom = -mouse_wheel_input.delta.y * camera_settings.speed;
            // When changing scales, logarithmic changes are more intuitive.
            // To get this effect, we add 1 to the delta, so that a delta of 0
            // results in no multiplicative effect, positive values result in a multiplicative increase,
            // and negative values result in multiplicative decreases.
            let multiplicative_zoom = 1. + delta_zoom;

            orthographic.scale = (orthographic.scale * multiplicative_zoom).clamp(
                camera_settings.min_scale,
                camera_settings.max_scale,
            );
        }
        _ => (),
    }
}