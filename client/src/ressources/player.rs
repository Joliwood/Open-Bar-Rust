use bevy::prelude::*;
use bevy_animation::RepeatAnimation;
use std::time::Duration;
use crate::Movable;

#[derive(Resource)]
pub struct Animations(Vec<Handle<AnimationClip>>);

pub fn player(
  mut commands: Commands,
  asset_server: Res<AssetServer>
) {

  // Insert a resource with the current scene information
  commands.insert_resource(Animations(vec![
    asset_server.load("models/barman/fox.glb#Animation0"),
    // asset_server.load("models/barman/barman_v2.glb#Animation1"),
  ]));
  
  // Add the player barman
  let entity_spawn = Vec3::new(2.5, 0.0, -0.5);
  commands.spawn((
    SceneBundle {
      transform: Transform::from_translation(entity_spawn),
      scene: asset_server.load("models/barman/fox.glb#Scene0"),
      ..default()
    },
    Movable::new(entity_spawn),
    Name::new("Player"),
  ));
}

// Once the scene is loaded, start the animation
pub fn setup_scene_once_loaded(
  animations: Res<Animations>,
  mut players: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
  for mut player in &mut players {
      player.play(animations.0[0].clone_weak()).repeat();
  }
}

pub fn keyboard_animation_control(
  keyboard_input: Res<ButtonInput<KeyCode>>,
  mut animation_players: Query<&mut AnimationPlayer>,
  animations: Res<Animations>,
  mut current_animation: Local<usize>,
) {
  for mut player in &mut animation_players {
      if keyboard_input.just_pressed(KeyCode::Space) {
          if player.is_paused() {
              player.resume();
          } else {
              player.pause();
          }
      }

      if keyboard_input.just_pressed(KeyCode::ArrowUp) {
          let speed = player.speed();
          player.set_speed(speed * 1.2);
      }

      if keyboard_input.just_pressed(KeyCode::ArrowDown) {
          let speed = player.speed();
          player.set_speed(speed * 0.8);
      }

      if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
          let elapsed = player.seek_time();
          player.seek_to(elapsed - 0.1);
      }

      if keyboard_input.just_pressed(KeyCode::ArrowRight) {
          let elapsed = player.seek_time();
          player.seek_to(elapsed + 0.1);
      }

      if keyboard_input.just_pressed(KeyCode::Enter) {
          *current_animation = (*current_animation + 1) % animations.0.len();
          player
              .play_with_transition(
                  animations.0[*current_animation].clone_weak(),
                  Duration::from_millis(250),
              )
              .repeat();
      }

      if keyboard_input.just_pressed(KeyCode::Digit1) {
          player.set_repeat(RepeatAnimation::Count(1));
          player.replay();
      }

      if keyboard_input.just_pressed(KeyCode::Digit3) {
          player.set_repeat(RepeatAnimation::Count(3));
          player.replay();
      }

      if keyboard_input.just_pressed(KeyCode::Digit5) {
          player.set_repeat(RepeatAnimation::Count(5));
          player.replay();
      }

      if keyboard_input.just_pressed(KeyCode::KeyL) {
          player.set_repeat(RepeatAnimation::Forever);
      }
  }
}
