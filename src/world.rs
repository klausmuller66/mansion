use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;


pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PhysicsPlugins::default(), PhysicsDebugPlugin::default()));
        app.add_systems(Startup, spawn_world);
        app.insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0,
        });

          }
    }


    fn spawn_world(
        mut commands: Commands,
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut meshes: ResMut<Assets<Mesh>>,
        assets: ResMut<AssetServer>,
    ) {
        commands.spawn((
            SceneBundle {
                scene: assets.load("models/terreno/terreno.gltf#Scene0"),
                ..default()
            },
            AsyncSceneCollider::new(Some(ComputedCollider::TriMesh)),
            RigidBody::Static,
        ));

}    
