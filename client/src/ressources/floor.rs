use bevy::prelude::*;

// Floor creation
pub fn floor (
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
  commands.spawn((
    SceneBundle {
        transform: Transform::from_translation(Vec3::new(0.0, -0.5, 0.0)),
        scene: asset_server.load("models/grass/grass.glb#Scene0"),
        ..default()
    },
    Name::new("Floor"))
);
}
