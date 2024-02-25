use bevy::prelude::*;

pub fn camera(mut commands: Commands) {
  commands.spawn((
    Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 10.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
        },
    Name::new("Camera")
  ));
}