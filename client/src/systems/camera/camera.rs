use bevy::prelude::*;
use bevy_atmosphere::prelude::*;

pub fn camera(
  mut commands: Commands,
) {

  commands.spawn((
    Camera3dBundle {
        transform: Transform::from_xyz(8.7, 11.3, 3.0)
          // .looking_at(Vec3::new(2.5, 0.0, -0.5), Vec3::Y),
          .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..default()
        },

    Name::new("Camera"),
    AtmosphereCamera::default(),

  ));
}
