// use bevy::prelude::*;

// // Floor creation
// pub fn floor (
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     commands.spawn(PbrBundle {
//         mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
//         material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
//         transform: Transform::from_translation(Vec3::new(0.0, -2.0, 0.0)),
//         ..Default::default()
//     });
// }
