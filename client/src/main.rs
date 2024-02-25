mod systems;
// mod ressources;

use bevy::prelude::*;
// use systems::constants::*;
// use ressources::*;
use systems::movements::player_movements::player_movements;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Background color
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_systems(Update, bevy::window::close_on_esc)
        // This startup will start only when the game starts
        .add_systems(Startup, setup)
        // .add_systems(Startup, spawn_floor.run())
        .add_systems(Update, player_movements)
        .run();
}

// Define a struct to keep some information about our entity.
// Here it's an arbitrary movement speed, the spawn location, and a maximum distance from it.
#[derive(Component)]

struct Movable {
    spawn: Vec3,
    max_distance: f32,
    speed: f32,
}

// Implement a utility function for easier Movable struct creation.
impl Movable {
    fn new(spawn: Vec3) -> Self {
        Movable {
            spawn,
            max_distance: 5.0,
            speed: 2.0,
        }
    }
}

// pub struct Player {
//     entity: Option<Entity>,
//     i: usize,
//     j: usize,
// }

// struct Game {
//     board: Vec<Vec<Cell>>,
//     player: Player,
//     score: i32,
// }

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // TODO - Stuct Game to create to store all data (score, alcools, etc...)
    // mut game: ResMut<Game>
) {
    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(4.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });

    // floor
    // spawn_floor();

    // Add a cube / player to visualize translation.
    // TODO - Use SceneBundle for futur use of 3D models
    let entity_spawn = Vec3::ZERO;
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::default()),
            material: materials.add(Color::WHITE),
            transform: Transform::from_translation(entity_spawn),
            ..default()
        },
        Movable::new(entity_spawn),
    ));

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
