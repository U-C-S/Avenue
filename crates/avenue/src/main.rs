mod terrain;

use bevy::{
    DefaultPlugins,
    app::{App, Startup},
    asset::Assets,
    camera::Camera3d,
    camera_controller::free_camera::{FreeCamera, FreeCameraPlugin},
    color::{Color, palettes::css},
    ecs::system::{Commands, ResMut},
    light::DirectionalLight,
    math::Vec3,
    mesh::{Mesh, Mesh3d, Meshable},
    pbr::{MeshMaterial3d, StandardMaterial},
    shape::Plane3d,
    transform::components::Transform,
    utils::default,
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FreeCameraPlugin))
        .init_resource::<terrain::Terrain>()
        .add_systems(Startup, setup_terrain)
        .run();
}

fn setup_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // sun ig
    commands.spawn((
        DirectionalLight {
            illuminance: 10_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(20.0, 40.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // ground plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0).subdivisions(10))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::from(css::SANDY_BROWN),
            perceptual_roughness: 0.8,
            ..default()
        })),
    ));

    // cam
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-12.5, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        FreeCamera::default(),
    ));
}
