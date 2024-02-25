use bevy::prelude::*;
use crate::systems::constants::*;

pub fn menu_button(
  mut interaction_query: Query<
      (
          &Interaction,
          &mut BackgroundColor,
          &mut BorderColor,
          &Children,
      ),
      (Changed<Interaction>, With<Button>),
  >,
  mut text_query: Query<&mut Text>,
) {
  for (interaction, mut color, mut border_color, children) in &mut interaction_query {
      let mut text = text_query.get_mut(children[0]).unwrap();
      match *interaction {
          Interaction::Pressed => {
              text.sections[0].value = "Press".to_string();
              *color = PRESSED_BUTTON.into();
              border_color.0 = Color::RED;
          }
          Interaction::Hovered => {
              text.sections[0].value = "Hover".to_string();
              *color = HOVERED_BUTTON.into();
              border_color.0 = Color::WHITE;
          }
          Interaction::None => {
              text.sections[0].value = "Button".to_string();
              *color = NORMAL_BUTTON.into();
              border_color.0 = Color::BLACK;
          }
      }
  }
}