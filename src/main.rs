use bevy:: prelude::*;
use bevy_xpbd_3d::{math::*, prelude::*};

mod world;
mod camera;


use world::WorldPlugin;
use camera::CameraPlugin;

fn main() {
        App::new()
            .add_plugins((
                DefaultPlugins,
                WorldPlugin,
                PhysicsPlugins::default(),
                PhysicsDebugPlugin::default(),
                CameraPlugin,
            ))
            .run();
}
