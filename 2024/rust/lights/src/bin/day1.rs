use bevy::prelude::*;

use std::marker::PhantomData;

// Marker components for our two lists
#[derive(Component, Default)]
struct LeftList;

#[derive(Component, Default)]
struct RightList;

// Generic number component that can be specialized
#[derive(Component)]
struct Number<T: Component> {
    value: i32,
    index: usize,
    _phantom: PhantomData<T>,
}

impl<T: Component> Number<T> {
    fn new(value: i32, index: usize) -> Self {
        Self {
            value,
            index,
            _phantom: PhantomData,
        }
    }
}

#[derive(Bundle)]
struct NumberBundle<T: Component> {
    number: Number<T>,
    text: Text,
    text_transform: Transform,
    global_transform: GlobalTransform,
    visibility: Visibility,
    computed_visibility: InheritedVisibility,
    text_2d: Text2d,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        // .add_systems(Update, hello_world)
        .run();
}

// fn hello_world() {
//     println!("hello Pluto!");
// }

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2d::default());

    // Example data
    let left = vec![1, 3, 5, 7];
    let right = vec![2, 4, 6, 8];

    // Spawn left numbers
    for (idx, &value) in left.iter().enumerate() {
        spawn_number::<LeftList>(&mut commands, value, idx);
    }

    // Spawn right numbers
    for (idx, &value) in right.iter().enumerate() {
        spawn_number::<RightList>(&mut commands, value, idx);
    }
}

fn spawn_number<T>(commands: &mut Commands, value: i32, index: usize) 
where T: Component
{
    let x_offset = if std::any::TypeId::of::<T>() == std::any::TypeId::of::<LeftList>() {
        -30.0
    } else {
        30.0
    };
    // let y_position = (index as f32) * -50.0 + 200.0;
    let y_position = 30.0 - (index as f32 * 25.0);

    // commands.spawn(SpriteBundle {
    //     sprite: Sprite {
    //         color: Color::RED,
    //         custom_size: Some(Vec2::new(5.0, 5.0)),
    //         ..default()
    //     },
    //     transform: Transform::from_xyz(x_offset, y_position, 0.0),
    //     ..default()
    // });

    commands.spawn((
        Sprite {
            color: Color::srgb(0.35, 0.75, 0.35),
            custom_size: Some(Vec2::new(5.0, 5.0)),
            ..default()
        },
        Transform::from_xyz(x_offset, y_position, 0.0)
    ));

    println!("Spawning number {} at position ({}, {})", value, x_offset, y_position);

    commands.spawn((
        NumberBundle {
            number: Number::<T>::new(value, index),
            text: Text::new(value.to_string()),
            text_transform: Transform::from_xyz(x_offset, y_position, 0.0),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::Visible,
            computed_visibility: InheritedVisibility::default(),
            text_2d: Text2d::default(),
        },
        // T::default(),
    ));
}