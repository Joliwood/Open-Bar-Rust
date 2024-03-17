// Exemple of a clippy rule for all this file
// #![allow(clippy::type_complexity)]

mod systems;
mod ressources;
mod ui;
mod sounds;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_atmosphere::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::new(),
            AtmospherePlugin,
        ))
        // Background color
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_systems(Startup, (
            sounds::audio_source::audio_source,
            ui::ui::ui,
            ressources::bar::bar,
            ressources::player::player,
            systems::lights::light::light,
            systems::camera::camera::camera,
            setup,
            ressources::sun::sun,
        ))
        .add_systems(Update, (
            systems::inputs::draw_cursor::draw_cursor,
            systems::movements::player_movements::player_movements,
            ui::menu_button::menu_button,
            bevy::window::close_on_esc, systems::inputs::mouse_click_system::mouse_click_system,
            ressources::player::setup_scene_once_loaded,
            ressources::player::keyboard_animation_control,
        ))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(AtmosphereCamera::default());
}

#[allow(dead_code)]

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
            speed: 4.0,
        }
    }
}

// Marker for updating the position of the light, not needed unless we have multiple lights
#[derive(Component)]
pub struct Sun;
