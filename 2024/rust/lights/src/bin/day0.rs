use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, hello_world)
        .run();
}

fn hello_world() {
    println!("hello Pluto!");
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2d::default());
}