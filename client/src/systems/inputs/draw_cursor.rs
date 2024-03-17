use bevy::prelude::*;
use crate::Movable;

// Constants for easier adjustments
const CURSOR_ABOVE_GROUND: f32 = 0.01;
const CURSOR_RADIUS: f32 = 0.2;

pub fn draw_cursor(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    ground_query: Query<&GlobalTransform, With<Movable>>,
    windows: Query<&Window>,
    mut gizmos: Gizmos,
) {
    // Assuming the first camera and window if multiple exist; adjust based on your game's design
    let (camera, camera_transform) = match camera_query.get_single() {
        Ok(value) => value,
        Err(_) => {
            // Handle error or log
            return;
        }
    };
    
    let ground = match ground_query.get_single() {
        Ok(value) => value,
        Err(_) => {
            // Handle error or log
            return;
        }
    };

    let window = match windows.get_single() {
        Ok(value) => value,
        Err(_) => {
            // Handle error or log
            return;
        }
    };

    let cursor_position = match window.cursor_position() {
        Some(position) => position,
        None => return, // Early return if cursor position is not available
    };

    let ray = match camera.viewport_to_world(camera_transform, cursor_position) {
        Some(ray) => ray,
        None => return, // Early return if ray cannot be calculated
    };

    let distance = match ray.intersect_plane(ground.translation(), Plane3d::new(ground.up())) {
        Some(distance) => distance,
        None => return, // Early return if ray does not intersect the ground plane
    };
    let point = ray.get_point(distance);

    gizmos.circle(
        point + ground.up() * CURSOR_ABOVE_GROUND,
        Direction3d::new_unchecked(ground.up()), // Assuming the up vector is normalized
        CURSOR_RADIUS,
        Color::WHITE,
    );
}
