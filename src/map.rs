use bevy::prelude::*;

#[derive(Resource)]
pub struct MapData {
    pub width: f32,
    pub height: f32,
    
    pub year: i32,
    
    pub camera_margin: f32,
}

#[derive(Component)]
pub struct Map {
    pub width: f32,
    pub height: f32,
    pub camera_margin: f32,
}

pub fn clamp_camera_system(
    map: Res<MapData>,
    mut q_camera: Query<&mut Transform, With<Camera>>
) {
    let mut transform = q_camera.single_mut().unwrap();
    
    let half_w = map.width / 2.0;
    let half_h = map.height / 2.0;
    let m = map.camera_margin;
    
    transform.translation.x = transform.translation.x
        .clamp(-half_w - m, half_w + m);
    transform.translation.y = transform.translation.y
        .clamp(-half_h - m, half_h + m);
}