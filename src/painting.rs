use crate::css;
use crate::layout;

type DisplayList = Vec<DisplayCommand>;

#[derive(Debug)]
enum DisplayCommand {
  SolidColor(css::Color, layout::Rectangle),
  // insert more commands here
}

impl PartialEq for DisplayCommand {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (DisplayCommand::SolidColor(a, b), DisplayCommand::SolidColor(c, d)) => a == c && b == d,
      _ => false,
    }
  }
}

// Return the specified color for CSS property "name", or None if no color was specified.
fn get_color(layout_box: &layout::LayoutBox, name: &str) -> Option<css::Color> {
  match layout_box.box_type() {
    layout::BoxType::BlockNode(style) | layout::BoxType::InlineNode(style) => {
      match style.value(name) {
        Some(css::Value::ColorValue(color)) => Some(color),
        _ => None,
      }
    }
    layout::BoxType::AnonymousBlock => None,
  }
}
