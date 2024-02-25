use bevy::{
    ecs::system::Res,
    input::{mouse::MouseButton, ButtonInput},
    log::info,
};

// This system prints messages when you press or release the left mouse button:
pub fn mouse_click_system(mouse_button_input: Res<ButtonInput<MouseButton>>) {
    if mouse_button_input.pressed(MouseButton::Left) {
        info!("left mouse currently pressed");
    }

    if mouse_button_input.just_pressed(MouseButton::Left) {
        info!("left mouse just pressed");
    }

    if mouse_button_input.just_released(MouseButton::Left) {
        info!("left mouse just released");
    }

    if mouse_button_input.pressed(MouseButton::Right) {
        info!("right mouse currently released");
    }

    if mouse_button_input.just_pressed(MouseButton::Right) {
        info!("right mouse just pressed");
    }

    if mouse_button_input.just_released(MouseButton::Right) {
        info!("right mouse just released");
    }
}
