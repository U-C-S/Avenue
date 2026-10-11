use bevy::{
    DefaultPlugins, app::{App, Startup, Update}, asset::Assets, color::Color, ecs::system::{Commands, ResMut}, light::GlobalAmbientLight, mesh::Mesh3d, pbr::StandardMaterial, utils::default,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_terrain)
        .add_systems(Update, hello)
        .run();
}

fn hello() {
    println!("Hello, Avenue!");
}

fn setup_terrain(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.insert_resource(GlobalAmbientLight {
        color: Color::srgb(0.64, 0.72, 0.86),
        brightness: 3500.0,
        ..default()
    });

    let ground_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.10, 0.13, 0.12),
        perceptual_roughness: 1.0,
        ..default()
    });

    // commands.spawn((Name::new("terrain"), Mesh3d))
}
