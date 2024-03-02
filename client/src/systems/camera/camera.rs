use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;

pub fn camera(mut commands: Commands) {
  commands.spawn((
    Camera3dBundle {
        transform: Transform::from_xyz(-1.5, 3.7, 3.6)
          .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
        },
    Name::new("Camera"),
    PanOrbitCamera::default(),
  ),
);
}