use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_world);
        app.insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0,
        });

          }
}
fn spawn_world(mut commands: Commands, mut asset_server: Res<AssetServer>) {
    let cenário1 = SceneBundle {
        scene: asset_server.load("models/terreno/terreno.gltf#Scene0"),
        ..default()
    };    

    commands.spawn(cenário1);
}    
