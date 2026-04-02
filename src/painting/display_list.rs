use crate::css;
use crate::dom;
use crate::layout;

pub(super) type DisplayList = Vec<DisplayCommand>;

#[derive(Debug)]
pub(super) enum DisplayCommand {
  SolidColor(css::Color, layout::Rectangle),
  // color, content bounds, text string, font size in px
  DrawText(css::Color, layout::Rectangle, String, f32),
}

impl PartialEq for DisplayCommand {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (DisplayCommand::SolidColor(a, b), DisplayCommand::SolidColor(c, d)) => a == c && b == d,
      (DisplayCommand::DrawText(a, b, c, d), DisplayCommand::DrawText(e, f, g, h)) => {
        a == e && b == f && c == g && d == h
      }
      _ => false,
    }
  }
}

// Return the specified color for CSS property "name", or None if no color was specified.
pub(super) fn get_color(layout_box: &layout::LayoutBox, name: &str) -> Option<css::Color> {
  match layout_box.box_type() {
    layout::BoxType::BlockNode(style)
    | layout::BoxType::InlineNode(style)
    | layout::BoxType::InlineBlockNode(style) => {
      match style.value(name) {
        Some(css::Value::ColorValue(color)) => Some(color),
        _ => None,
      }
    }
    layout::BoxType::AnonymousBlock => None,
  }
}

pub(super) fn render_background(list: &mut DisplayList, layout_box: &layout::LayoutBox) {
  get_color(layout_box, "background").map(|color: css::Color| {
    list.push(DisplayCommand::SolidColor(
      color,
      layout_box.dimensions().border_box(),
    ))
  });
}

pub(super) fn render_borders(list: &mut DisplayList, layout_box: &layout::LayoutBox) {
  let color: css::Color = match get_color(layout_box, "border-color") {
    Some(color) => color,
    _ => return, // fail out if no border-color is specified
  };

  let dimensions: &layout::Dimensions = layout_box.dimensions();
  let border_box: layout::Rectangle = dimensions.border_box();

  // Top border
  list.push(DisplayCommand::SolidColor(
    color,
    layout::Rectangle::new(
      border_box.x(),
      border_box.y(),
      border_box.width(),
      dimensions.border().top(),
    ),
  ));

  // Right border
  list.push(DisplayCommand::SolidColor(
    color,
    layout::Rectangle::new(
      border_box.x() + border_box.width() - dimensions.border().right(),
      border_box.y(),
      dimensions.border().right(),
      border_box.height(),
    ),
  ));

  // Bottom border
  list.push(DisplayCommand::SolidColor(
    color,
    layout::Rectangle::new(
      border_box.x(),
      border_box.y() + border_box.height() - dimensions.border().bottom(),
      border_box.width(),
      dimensions.border().bottom(),
    ),
  ));

  // Left border
  list.push(DisplayCommand::SolidColor(
    color,
    layout::Rectangle::new(
      border_box.x(),
      border_box.y(),
      dimensions.border().left(),
      border_box.height(),
    ),
  ));
}

// If this layout box is an inline node wrapping a DOM text node, add a DrawText command.
pub(super) fn render_text(list: &mut DisplayList, layout_box: &layout::LayoutBox) {
  if let layout::BoxType::InlineNode(style) = layout_box.box_type() {
    if let dom::NodeType::Text(text) = style.node().node_type() {
      // Skip whitespace-only text nodes produced by HTML indentation
      if text.trim().is_empty() {
        return;
      }
      // Text nodes have no specified values (inheritance is not yet implemented),
      // so default to black text at 16px.
      let color: css::Color = css::Color::new(0, 0, 0, 255);
      let font_size: f32 = style
        .value("font-size")
        .map(|v: css::Value| v.to_px())
        .unwrap_or(16.0);
      list.push(DisplayCommand::DrawText(
        color,
        *layout_box.dimensions().content(),
        text.clone(),
        font_size,
      ));
    }
  }
}

fn render_layout_box(list: &mut DisplayList, layout_box: &layout::LayoutBox) {
  render_background(list, layout_box);
  render_borders(list, layout_box);
  render_text(list, layout_box);

  for child in layout_box.children() {
    render_layout_box(list, child);
  }
}

pub(super) fn build_display_list(layout_root: &layout::LayoutBox) -> DisplayList {
  let mut list: Vec<DisplayCommand> = Vec::new();
  render_layout_box(&mut list, layout_root);
  return list;
}
