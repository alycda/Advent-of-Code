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
    let left = vec![3, 4, 2, 1, 3, 3];
    let right = vec![4, 3, 5, 3, 9, 3];

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
        -40.0
    } else {
        40.0
    };
    let y_position = 195.0 - (index as f32 * 85.0);

    // println!("Spawning number {} at position ({}, {})", value, x_offset, y_position);

    commands.spawn((
        Number::<T>::new(value, index),
        Sprite {
            color: if std::any::TypeId::of::<T>() == std::any::TypeId::of::<LeftList>() {
                Color::srgb(0.35, 0.75, 0.35)
            } else {
                Color::srgb(0.75, 0.35, 0.75)
            },
            custom_size: Some(Vec2::new(20.0, 20.0)),
            ..default()
        },
        Transform::from_xyz(x_offset, y_position, 0.0),
    ))
        .with_children(|parent| {
            // Spawn Text2d as child
            parent.spawn((
                Text2d::new(value.to_string()),
                TextFont {
                    font_size: 20.0,
                    // color: Color::BLACK,
                    ..default()
                },
                // Color::BLACK,
                // Center both horizontally and vertically
                TextLayout::new_with_justify(JustifyText::Center),
                // Text will appear centered above the sprite
                Transform::from_xyz(0.0, 5.0, 0.5), // keeping Z at 0 causes rendering issues where they're on the same layer and results in a flicker
            ));
        });
}