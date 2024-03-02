use bevy::prelude::*;
use crate::Movable;


pub fn player(
  mut commands: Commands,
  asset_server: Res<AssetServer>
) {
  
  // Add the player barman
  let entity_spawn = Vec3::new(2.5, 0.0, -0.5);
  commands.spawn((
    SceneBundle {
      transform: Transform::from_translation(entity_spawn),
      scene: asset_server.load("models/barman/barman_v2.glb#Scene0"),
      ..default()
    },
    Movable::new(entity_spawn),
    Name::new("Player"),
  ));
}
