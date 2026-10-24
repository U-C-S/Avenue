mod terrain;

use bevy::{
    DefaultPlugins,
    app::{App, Startup},
    camera::Camera3d,
    camera_controller::free_camera::{FreeCamera, FreeCameraPlugin},
    ecs::system::Commands,
    math::Vec3,
    transform::components::Transform,
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FreeCameraPlugin))
        .add_systems(Startup, (setup_cam, terrain::setup_terrain))
        .run();
}

fn setup_cam(mut commands: Commands) {
    // cam
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-12.5, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        FreeCamera::default(),
    ));
}
