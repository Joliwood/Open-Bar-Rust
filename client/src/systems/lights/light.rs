use bevy::prelude::*;

pub fn light(mut commands: Commands,) {
  commands.spawn((
    PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    },
    Name::new("Light")
));
}