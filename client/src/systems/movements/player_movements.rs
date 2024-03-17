use bevy::prelude::*;
use crate::Movable;
use std::f32::consts::PI;

pub fn player_movements(
  mut cubes: Query<(&mut Transform, &mut Movable)>, 
  keyboard_input: Res<ButtonInput<KeyCode>>,
  timer: Res<Time>
) {
    // On loop sur les cubes reçus par la query
    for (mut transform, movable) in cubes.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowUp) {
            direction.z -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            direction.z += 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }

        if direction != Vec3::ZERO {
            let direction = direction.normalize();
            transform.translation += direction * movable.speed * timer.delta_seconds();

            // Calculate the angle to rotate
            let angle = direction.y.atan2(direction.x);
            transform.rotation = Quat::from_rotation_y(angle - PI / 2.0);
        }
    }
}