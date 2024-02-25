use bevy::prelude::*;
use bevy_third_person_camera::*;

pub fn camera(mut commands: Commands) {
  commands.spawn((
    Camera3dBundle {
        transform: Transform::from_xyz(-1.5, 3.7, 3.6)
          .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
        },
    Name::new("Camera"),
    ThirdPersonCamera::default(),
  ),
);
}