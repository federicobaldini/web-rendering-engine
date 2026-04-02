use crate::css;
use crate::style::StyledNode;
use super::{BoxType, Dimensions, LayoutBox};

fn sum<I>(iter: I) -> f32
where
  I: Iterator<Item = f32>,
{
  iter.fold(0., |a: f32, b: f32| a + b)
}

impl<'a> LayoutBox<'a> {
  // Calculate the width of a block-level non-replaced element in normal flow
  // http://www.w3.org/TR/CSS2/visudet.html#blockwidth
  // Sets the horizontal margin/padding/border dimensions, and the "width"
  pub(super) fn calculate_block_width(&mut self, containing_block: Dimensions) {
    let style: &StyledNode = self.get_style_node();

    // "width" has initial value "auto"
    let auto: css::Value = css::Value::Keyword("auto".to_string());
    let mut width: css::Value = style.value("width").unwrap_or(auto.clone());

    // margin, border, and padding have initial value 0
    let zero: css::Value = css::Value::Length(0.0, css::Unit::Px);

    let mut margin_left: css::Value = style.lookup("margin-left", "margin", &zero);
    let mut margin_right: css::Value = style.lookup("margin-right", "margin", &zero);

    let border_left: css::Value = style.lookup("border-left-width", "border-width", &zero);
    let border_right: css::Value = style.lookup("border-right-width", "border-width", &zero);

    let padding_left: css::Value = style.lookup("padding-left", "padding", &zero);
    let padding_right: css::Value = style.lookup("padding-right", "padding", &zero);

    let total: f32 = sum(
      [
        &margin_left,
        &margin_right,
        &border_left,
        &border_right,
        &padding_left,
        &padding_right,
        &width,
      ]
      .iter()
      .map(|v: &&css::Value| v.to_px()),
    );

    // If width is not auto and the total is wider than the container, treat auto margins as 0
    if width != auto && total > containing_block.content.width {
      if margin_left == auto {
        margin_left = css::Value::Length(0.0, css::Unit::Px);
      }
      if margin_right == auto {
        margin_right = css::Value::Length(0.0, css::Unit::Px);
      }
    }

    // Adjust used values so that the above sum equals "containing_block.width"
    // Each arm of the "match" should increase the total width by exactly "underflow",
    // and afterward all values should be absolute lengths in px
    let underflow: f32 = containing_block.content.width - total;

    match (width == auto, margin_left == auto, margin_right == auto) {
      // If the values are overconstrained, calculate margin_right.
      (false, false, false) => {
        margin_right = css::Value::Length(margin_right.to_px() + underflow, css::Unit::Px);
      }

      // If exactly one size is auto, its used value follows from the equality
      (false, false, true) => {
        margin_right = css::Value::Length(underflow, css::Unit::Px);
      }
      (false, true, false) => {
        margin_left = css::Value::Length(underflow, css::Unit::Px);
      }

      // If width is set to auto, any other auto values become 0
      (true, _, _) => {
        if margin_left == auto {
          margin_left = css::Value::Length(0.0, css::Unit::Px);
        }
        if margin_right == auto {
          margin_right = css::Value::Length(0.0, css::Unit::Px);
        }

        if underflow >= 0.0 {
          // Expand width to fill the underflow
          width = css::Value::Length(underflow, css::Unit::Px);
        } else {
          // Width can't be negative. Adjust the right margin instead
          width = css::Value::Length(0.0, css::Unit::Px);
          margin_right = css::Value::Length(margin_right.to_px() + underflow, css::Unit::Px);
        }
      }

      // If margin-left and margin-right are both auto, their used values are equal
      (false, true, true) => {
        margin_left = css::Value::Length(underflow / 2.0, css::Unit::Px);
        margin_right = css::Value::Length(underflow / 2.0, css::Unit::Px);
      }
    }

    self.dimensions.content.width = width.to_px();

    self.dimensions.padding.left = padding_left.to_px();
    self.dimensions.padding.right = padding_right.to_px();

    self.dimensions.border.left = border_left.to_px();
    self.dimensions.border.right = border_right.to_px();

    self.dimensions.margin.left = margin_left.to_px();
    self.dimensions.margin.right = margin_right.to_px();
  }

  // Finish calculating the block's edge sizes, and position it within its containing block
  // http://www.w3.org/TR/CSS2/visudet.html#normal-block
  // Sets the vertical margin/padding/border dimensions, and the "x", "y" values
  pub(super) fn calculate_block_position(&mut self, containing_block: Dimensions) {
    let style: &StyledNode = self.get_style_node();

    // margin, border, and padding have initial value 0
    let zero: css::Value = css::Value::Length(0.0, css::Unit::Px);

    // If margin-top or margin-bottom is "auto", the used value is zero
    self.dimensions.margin.top = style.lookup("margin-top", "margin", &zero).to_px();
    self.dimensions.margin.bottom = style.lookup("margin-bottom", "margin", &zero).to_px();

    self.dimensions.border.top = style
      .lookup("border-top-width", "border-width", &zero)
      .to_px();
    self.dimensions.border.bottom = style
      .lookup("border-bottom-width", "border-width", &zero)
      .to_px();

    self.dimensions.padding.top = style.lookup("padding-top", "padding", &zero).to_px();
    self.dimensions.padding.bottom = style.lookup("padding-bottom", "padding", &zero).to_px();

    self.dimensions.content.x = containing_block.content.x
      + self.dimensions.margin.left
      + self.dimensions.border.left
      + self.dimensions.padding.left;

    // Position the box below all the previous boxes in the container
    self.dimensions.content.y = containing_block.content.height
      + containing_block.content.y
      + self.dimensions.margin.top
      + self.dimensions.border.top
      + self.dimensions.padding.top;
  }

  // Lay out the block's children within its content area
  // Sets "self.dimensions.height" to the total content height
  pub(super) fn layout_block_children(&mut self) {
    for child in &mut self.children {
      child.layout(self.dimensions);
      // Increment the height so each child is laid out below the previous one
      self.dimensions.content.height =
        self.dimensions.content.height + child.dimensions.margin_box().height();
    }
  }

  // Height of a block-level non-replaced element in normal flow with overflow visible
  pub(super) fn calculate_block_height(&mut self) {
    // If the height is set to an explicit length, use that exact length
    // Otherwise, just keep the value set by "layout_block_children"
    if let Some(css::Value::Length(height, css::Unit::Px)) = self.get_style_node().value("height") {
      self.dimensions.content.height = height;
    }
  }

  // Lay out a block-level element and its descendants
  pub fn layout_block(&mut self, containing_block: Dimensions) {
    // Child width can depend on parent width, so we need to calculate this box's width before
    // laying out its children.
    self.calculate_block_width(containing_block);

    // Determine where the box is located within its container
    self.calculate_block_position(containing_block);

    // Recursively lay out the children of this box
    self.layout_block_children();

    // Parent height can depend on child height, so "calculate_height" must be called after the
    // children are laid out
    self.calculate_block_height();
  }
}
