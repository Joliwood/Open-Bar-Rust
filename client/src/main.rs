mod systems;

use bevy::{ prelude::*, math::vec3 };
use systems::constants::*;
use systems::movements::paddle_movements::paddle_movements;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Background color
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_systems(Update, bevy::window::close_on_esc)
        // This startup will start only when the game starts
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, paddle_movements)
        .run();
}

#[derive(Component)]

pub struct Paddle;

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Paddle
    commands.spawn((SpriteBundle {
        transform: Transform {
            translation: vec3(0.0, PADDLE_START_Y, 0.0),
            ..default()
        },
        sprite: Sprite {
            custom_size: Some(PADDLE_SIZE),
            color: PADDLE_COLOR,
            ..default()
        },
        ..default()
        },
        Paddle,
    ));
}
