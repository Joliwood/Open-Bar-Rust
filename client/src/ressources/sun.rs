use crate::Sun;
use bevy::prelude::{Commands, DirectionalLightBundle};

// Simple sun environment
pub fn sun(
  mut commands: Commands,
) {
  // Our Sun
  commands.spawn((
      DirectionalLightBundle {
          ..Default::default()
      },
      Sun, // Marks the light as Sun
  ));
}
