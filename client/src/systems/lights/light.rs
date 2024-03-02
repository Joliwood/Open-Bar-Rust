use bevy::prelude::*;

pub fn light(mut commands: Commands,) {
  commands.spawn((
    PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            color: Color::ORANGE,
            ..default()
        },
        transform: Transform::from_xyz(3.1, 4.8, -6.5),
        ..default()
    },
    Name::new("Light")
));
}