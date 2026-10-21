use bevy::ecs::resource::Resource;

#[derive(Resource, Default)]
pub struct Terrain(usize);