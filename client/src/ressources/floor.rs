use bevy::prelude::*;

// Floor creation
pub fn floor (
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let entity_spawn = {
        // Little hack to have same height position as the player
        Vec3::new(0.0, -0.1, 0.0)
    };

  commands.spawn((
    SceneBundle {
        transform: Transform::from_translation(entity_spawn),
        scene: asset_server.load("models/grass/grass.glb#Scene0"),
        ..default()
    },
    Name::new("Floor"))
);
}
