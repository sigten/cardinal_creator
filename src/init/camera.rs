use bevy::prelude::*;
use bevy_pancam::PanCam;

pub fn create_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default()).insert(PanCam {
        grab_buttons: vec![MouseButton::Left],
        enabled: true,
        zoom_to_cursor: true,
        min_scale: 1.,
        max_scale: Some(10.),
    });
    debug!("Creating camera")
}
