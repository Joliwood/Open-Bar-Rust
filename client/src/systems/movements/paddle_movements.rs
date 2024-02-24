use bevy::prelude::*;
use crate::Paddle;
use super::super::constants::*;

pub fn paddle_movements(
  keyboard_input: Res<ButtonInput<KeyCode>>,
  time: Res<Time>,
  mut query: Query<&mut Transform, With<Paddle>>,
) {
  let mut paddle_transform = query.single_mut();
  let mut horizontal_direction = 0.0;
  let mut vertical_direction = 0.0;

  // Left movement (Azerty only)
  if keyboard_input.pressed(KeyCode::KeyA) 
  || keyboard_input.pressed(KeyCode::ArrowLeft) {
    horizontal_direction -= 1.0;
  }
  // Right movement
  if keyboard_input.pressed(KeyCode::KeyD)
  || keyboard_input.pressed(KeyCode::ArrowRight) {
    horizontal_direction += 1.0;
  }

  // Up movement
  if keyboard_input.pressed(KeyCode::KeyW)
  || keyboard_input.pressed(KeyCode::ArrowUp) {
    vertical_direction += 1.0;
  }

  // Down movement
  if keyboard_input.pressed(KeyCode::KeyS)
  || keyboard_input.pressed(KeyCode::ArrowDown) {
    vertical_direction -= 1.0;
  }

  // Calculate the new horizontal paddle position based on player input
  let new_paddle_horizontal_position = 
    paddle_transform.translation.x + horizontal_direction * PADDLE_SPEED * time.delta_seconds();

  // Calculate the new vertical paddle position based on player input
  let new_paddle_vertical_position = 
    paddle_transform.translation.y + vertical_direction * PADDLE_SPEED * time.delta_seconds();

  paddle_transform.translation.x = new_paddle_horizontal_position;
  paddle_transform.translation.y = new_paddle_vertical_position;
}
