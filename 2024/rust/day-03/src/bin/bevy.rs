use bevy::prelude::*;

#[derive(Component, Debug)]
struct TextChar;

#[derive(Component, Debug)]
struct TextVisualizer {
    content: String,
    chars: Vec<Entity>,  // References to individual TextChar entities
    current_position: usize,
}

// // Resource to hold the original text
// #[derive(Resource)]
// struct VisualizerText(String);

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_systems(Update, hello_world).run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let input_text = "your long text string here".to_string();

    // Spawn the visualizer parent
    let visualizer = commands
        .spawn((
            TextVisualizer {
                content: input_text.clone(),
                chars: Vec::new(),
                current_position: 0,
            },
            // Spatial bundle for positioning the whole text block
            SpatialBundle::default(),
        ))
        .id();
}

fn hello_world() {
    println!("hello world!");
}