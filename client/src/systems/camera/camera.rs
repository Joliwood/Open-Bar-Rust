use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;
use bevy_atmosphere::prelude::*;

pub fn camera(
  mut commands: Commands,
) {
  commands.spawn((
    Camera3dBundle {
        transform: Transform::from_xyz(5., 0., 5.)
          .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
        },
    Name::new("Camera"),
    AtmosphereCamera::default(),
    PanOrbitCamera::default(),
  ),
);
}