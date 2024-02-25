use bevy::prelude::*;

use crate::Movable;

pub fn player_movements(
  mut cubes: Query<(&mut Transform, &mut Movable)>, 
  keyboard_input: Res<ButtonInput<KeyCode>>,
  timer: Res<Time>
) {

  if keyboard_input.pressed(KeyCode::ArrowUp) {
      for (mut transform, cube) in &mut cubes {
          let direction = transform.local_z();
          transform.translation -= direction * cube.speed * timer.delta_seconds();
      }
  }

  if keyboard_input.pressed(KeyCode::ArrowDown) {
      for (mut transform, cube) in &mut cubes {
          let direction = transform.local_z();
          transform.translation += direction * cube.speed * timer.delta_seconds();
      }
  }

  if keyboard_input.pressed(KeyCode::ArrowLeft) {
      for (mut transform, cube) in &mut cubes {
          let direction = transform.local_x();
          transform.translation -= direction * cube.speed * timer.delta_seconds();
      }
  }

  if keyboard_input.pressed(KeyCode::ArrowRight) {
      for (mut transform, cube) in &mut cubes {
          let direction = transform.local_x();
          transform.translation += direction * cube.speed * timer.delta_seconds();
      }
  }
  
}