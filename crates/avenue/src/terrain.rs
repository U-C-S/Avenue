use bevy::{
    asset::Assets,
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

pub fn setup_terrain(
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
}
