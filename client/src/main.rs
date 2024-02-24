mod systems;

use bevy::{ 
    prelude::*, 
    // math::vec3 
};
// use systems::constants::*;
// use systems::movements::paddle_movements::paddle_movements;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Background color
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_systems(Update, bevy::window::close_on_esc)
        // This startup will start only when the game starts
        .add_systems(Startup, setup)
        // .add_systems(FixedUpdate, paddle_movements)
        .run();
}

// #[derive(Component)]

// pub struct Paddle;

// 2D basic setup
// fn setup(mut commands: Commands) {
//     // Camera
//     commands.spawn(Camera2dBundle::default());

//     // Paddle
//     commands.spawn((SpriteBundle {
//         transform: Transform {
//             translation: vec3(0.0, PADDLE_START_Y, 0.0),
//             ..default()
//         },
//         sprite: Sprite {
//             custom_size: Some(PADDLE_SIZE),
//             color: PADDLE_COLOR,
//             ..default()
//         },
//         ..default()
//         },
//         Paddle,
//     ));
// }

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(4.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::rgb_u8(124, 144, 255)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 10.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
