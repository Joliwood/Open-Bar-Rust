use bevy::prelude::*;
use crate::Movable;

pub fn player(
  mut commands: Commands,
  asset_server: Res<AssetServer>
) {

  // Add a player to visualize translation.
  // TODO - Use SceneBundle for futur use of 3D models
  let entity_spawn = Vec3::ZERO;
  commands.spawn((
      SceneBundle {
          transform: Transform::from_translation(entity_spawn),
          scene: asset_server.load("models/alien/alien.glb#Scene0"),
          ..default()
      },
      Movable::new(entity_spawn),
      Name::new("Player"),
  ));
}