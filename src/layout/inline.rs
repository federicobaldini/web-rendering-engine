use crate::css;
use crate::style::StyledNode;
use super::{BoxType, Dimensions, LayoutBox};

impl<'a> LayoutBox<'a> {
  // Compute the dimensions of an inline-level element from its CSS properties.
  // Position (x, y) is not set here — the parent anonymous block is responsible for that.
  pub(super) fn layout_inline(&mut self, _containing_block: Dimensions) {
    let style: &StyledNode = self.get_style_node();
    let zero: css::Value = css::Value::Length(0.0, css::Unit::Px);

    self.dimensions.padding.left = style.lookup("padding-left", "padding", &zero).to_px();
    self.dimensions.padding.right = style.lookup("padding-right", "padding", &zero).to_px();
    self.dimensions.padding.top = style.lookup("padding-top", "padding", &zero).to_px();
    self.dimensions.padding.bottom = style.lookup("padding-bottom", "padding", &zero).to_px();

    self.dimensions.border.left = style.lookup("border-left-width", "border-width", &zero).to_px();
    self.dimensions.border.right = style.lookup("border-right-width", "border-width", &zero).to_px();
    self.dimensions.border.top = style.lookup("border-top-width", "border-width", &zero).to_px();
    self.dimensions.border.bottom = style.lookup("border-bottom-width", "border-width", &zero).to_px();

    self.dimensions.margin.left = style.lookup("margin-left", "margin", &zero).to_px();
    self.dimensions.margin.right = style.lookup("margin-right", "margin", &zero).to_px();
    self.dimensions.margin.top = style.lookup("margin-top", "margin", &zero).to_px();
    self.dimensions.margin.bottom = style.lookup("margin-bottom", "margin", &zero).to_px();

    // Use the explicit CSS width/height, or 0 if not specified (no text measurement yet)
    self.dimensions.content.width = style.value("width").map(|v: css::Value| v.to_px()).unwrap_or(0.0);
    self.dimensions.content.height = style.value("height").map(|v: css::Value| v.to_px()).unwrap_or(0.0);
  }

  // Place inline children left-to-right inside an anonymous block, wrapping to the next
  // line when a child no longer fits within the container width.
  pub(super) fn layout_anonymous_block(&mut self, containing_block: Dimensions) {
    self.dimensions.content.x = containing_block.content.x;
    self.dimensions.content.y = containing_block.content.y + containing_block.content.height;
    self.dimensions.content.width = containing_block.content.width;

    let mut cursor_x: f32 = 0.0;
    let mut cursor_y: f32 = 0.0;
    let mut line_height: f32 = 0.0;

    for child in &mut self.children {
      child.layout(self.dimensions);

      let child_margin_width: f32 = child.dimensions.margin_box().width();
      let child_margin_height: f32 = child.dimensions.margin_box().height();

      // Wrap only when there is already content on the current line
      if cursor_x + child_margin_width > self.dimensions.content.width && cursor_x > 0.0 {
        cursor_y += line_height;
        cursor_x = 0.0;
        line_height = 0.0;
      }

      child.dimensions.content.x = self.dimensions.content.x
        + cursor_x
        + child.dimensions.margin.left
        + child.dimensions.border.left
        + child.dimensions.padding.left;
      child.dimensions.content.y = self.dimensions.content.y
        + cursor_y
        + child.dimensions.margin.top
        + child.dimensions.border.top
        + child.dimensions.padding.top;

      // InlineBlockNode children were laid out with content.x/y = 0; now that their
      // final position is known, shift all their descendants by the same delta.
      if let BoxType::InlineBlockNode(_) = child.box_type {
        let dx = child.dimensions.content.x;
        let dy = child.dimensions.content.y;
        child.offset_descendants(dx, dy);
      }

      cursor_x += child_margin_width;
      if child_margin_height > line_height {
        line_height = child_margin_height;
      }
    }

    self.dimensions.content.height = cursor_y + line_height;
  }
}
