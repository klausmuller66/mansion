use bevy:: prelude::*;

mod world;
mod camera;
mod walk;


use world::WorldPlugin;
use camera::CameraPlugin;
use walk::WalkPlugin;

fn main() {
        App::new()
            .add_plugins((DefaultPlugins, WorldPlugin, CameraPlugin, WalkPlugin))
            .run();
}
