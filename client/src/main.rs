mod systems;
mod ressources;
mod ui;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::new()
        ))
        // Background color
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_systems(Startup, (
            ui::ui::ui,
            ressources::floor::floor,
            ressources::player::player,
            systems::lights::light::light,
            systems::camera::camera::camera
        ))
        .add_systems(Update, (
            systems::movements::player_movements::player_movements, 
            ui::menu_button::menu_button, 
            bevy::window::close_on_esc, systems::inputs::mouse_click_system::mouse_click_system
        ))
        .run();
}

// Define a struct to keep some information about our entity.
// Here it's an arbitrary movement speed, the spawn location, and a maximum distance from it.
#[derive(Component)]

pub struct Movable {
    spawn: Vec3,
    max_distance: f32,
    speed: f32,
}

// Implement a utility function for easier Movable struct creation.
impl Movable {
    fn new(spawn: Vec3) -> Self {
        Movable {
            spawn,
            max_distance: 5.0,
            speed: 2.0,
        }
    }
}
