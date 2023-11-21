use bevy::prelude::*;
use bevy_flycam::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.add_plugins(NoCameraPlayerPlugin);
    }
}


fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 2.0, 0.5),
            ..default()
        },
        FlyCam
    ));
}
