use bevy::{
    DefaultPlugins,
    app::{App, Update},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, hello)
        .run();
}

fn hello() {
    println!("Hello, Avenue!");
}
