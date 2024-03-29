// CSS box model. All sizes are in px
/**
 * Features to add:
 * - Collapsing vertical margins;
 * - Relative positioning; (https://www.w3.org/TR/CSS2/visuren.html#relative-positioning)
 * - Parallelize the layout process, and measure the effect on performance;
 */
use crate::css;
use crate::style;
use crate::style::StyledNode;
use std::default::Default;

fn sum<I>(iter: I) -> f32
where
  I: Iterator<Item = f32>,
{
  iter.fold(0., |a: f32, b: f32| a + b)
}

#[derive(Copy, Clone, Default, Debug)]
pub struct Rectangle {
  x: f32,
  y: f32,
  width: f32,
  height: f32,
}

impl PartialEq for Rectangle {
  fn eq(&self, other: &Self) -> bool {
    self.x == other.x
      && self.y == other.y
      && self.width == other.width
      && self.height == other.height
  }
}

impl Rectangle {
  pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
    Self {
      x,
      y,
      width,
      height,
    }
  }

  pub fn expanded_by(self, edge: EdgeSizes) -> Self {
    Self {
      x: self.x - edge.left,
      y: self.y - edge.top,
      width: self.width + edge.left + edge.right,
      height: self.height + edge.top + edge.bottom,
    }
  }

  pub fn x(&self) -> f32 {
    self.x
  }

  pub fn y(&self) -> f32 {
    self.y
  }

  pub fn width(&self) -> f32 {
    self.width
  }

  pub fn set_width(&mut self, width: f32) {
    self.width = width;
  }

  pub fn height(&self) -> f32 {
    self.height
  }

  pub fn set_height(&mut self, height: f32) {
    self.height = height;
  }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct EdgeSizes {
  top: f32,
  right: f32,
  bottom: f32,
  left: f32,
}

impl PartialEq for EdgeSizes {
  fn eq(&self, other: &Self) -> bool {
    self.left == other.left
      && self.right == other.right
      && self.top == other.top
      && self.bottom == other.bottom
  }
}

impl EdgeSizes {
  pub fn new(top: f32, right: f32, bottom: f32, left: f32) -> Self {
    Self {
      top,
      right,
      bottom,
      left,
    }
  }

  pub fn top(&self) -> f32 {
    self.top
  }

  pub fn right(&self) -> f32 {
    self.right
  }

  pub fn bottom(&self) -> f32 {
    self.bottom
  }

  pub fn left(&self) -> f32 {
    self.left
  }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct Dimensions {
  // Position of the content area relative to the document origin:
  content: Rectangle,
  // Surrounding edges:
  padding: EdgeSizes,
  border: EdgeSizes,
  margin: EdgeSizes,
}

impl PartialEq for Dimensions {
  fn eq(&self, other: &Self) -> bool {
    self.content == other.content
      && self.padding == other.padding
      && self.border == other.border
      && self.margin == other.margin
  }
}

impl Dimensions {
  pub fn new(content: Rectangle, padding: EdgeSizes, border: EdgeSizes, margin: EdgeSizes) -> Self {
    Self {
      content,
      padding,
      border,
      margin,
    }
  }

  pub fn content(&self) -> &Rectangle {
    &self.content
  }

  pub fn set_content(&mut self) -> &mut Rectangle {
    &mut self.content
  }

  pub fn padding(&self) -> &EdgeSizes {
    &self.padding
  }

  pub fn border(&self) -> &EdgeSizes {
    &self.border
  }

  pub fn margin(&self) -> &EdgeSizes {
    &self.margin
  }

  // The area covered by the content area plus its padding
  pub fn padding_box(self) -> Rectangle {
    self.content.expanded_by(self.padding)
  }

  // The area covered by the content area plus padding and borders
  pub fn border_box(self) -> Rectangle {
    self.padding_box().expanded_by(self.border)
  }

  // The area covered by the content area plus padding, borders, and margin
  pub fn margin_box(self) -> Rectangle {
    self.border_box().expanded_by(self.margin)
  }
}

#[derive(Clone, Debug)]
pub enum BoxType<'a> {
  BlockNode(&'a style::StyledNode<'a>),
  InlineNode(&'a style::StyledNode<'a>),
  AnonymousBlock,
}

impl<'a> PartialEq for BoxType<'a> {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (BoxType::BlockNode(a), BoxType::BlockNode(b)) => a == b,
      (BoxType::InlineNode(a), BoxType::InlineNode(b)) => a == b,
      (BoxType::AnonymousBlock, BoxType::AnonymousBlock) => true,
      _ => false,
    }
  }
}

// A node in the layout tree
#[derive(Clone, Debug)]
pub struct LayoutBox<'a> {
  dimensions: Dimensions,
  box_type: BoxType<'a>,
  children: Vec<LayoutBox<'a>>,
}

impl<'a> PartialEq for LayoutBox<'a> {
  fn eq(&self, other: &Self) -> bool {
    self.dimensions == other.dimensions
      && self.box_type == other.box_type
      && self.children == other.children
  }
}

impl<'a> LayoutBox<'a> {
  pub fn new(box_type: BoxType<'a>) -> Self {
    Self {
      box_type,
      dimensions: Default::default(),
      children: Vec::new(),
    }
  }

  pub fn dimensions(&self) -> &Dimensions {
    &self.dimensions
  }

  pub fn box_type(&self) -> &BoxType<'a> {
    &self.box_type
  }

  pub fn children(&self) -> &Vec<LayoutBox<'a>> {
    &self.children
  }

  pub fn add_child(&mut self, new_layout_box: LayoutBox<'a>) {
    self.children.push(new_layout_box);
  }

  fn get_style_node(&self) -> &'a style::StyledNode<'a> {
    match &self.box_type {
      BoxType::BlockNode(node) | BoxType::InlineNode(node) => node,
      BoxType::AnonymousBlock => panic!("Anonymous block box has no style node"),
    }
  }

  // Where a new inline child should go
  pub fn get_inline_container(&mut self) -> &mut LayoutBox<'a> {
    match self.box_type {
      BoxType::InlineNode(_) | BoxType::AnonymousBlock => self,
      BoxType::BlockNode(_) => {
        // If we've just generated an anonymous block box, keep using it
        // Otherwise, create a new one
        match self.children.last() {
          Some(&LayoutBox {
            box_type: BoxType::AnonymousBlock,
            ..
          }) => {}
          _ => self.children.push(LayoutBox::new(BoxType::AnonymousBlock)),
        }
        self.children.last_mut().unwrap()
      }
    }
  }

  // Calculate the width of a block-level non-replaced element in normal flow
  // http://www.w3.org/TR/CSS2/visudet.html#blockwidth
  // Sets the horizontal margin/padding/border dimensions, and the "width"
  fn calculate_block_width(&mut self, containing_block: Dimensions) {
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
  fn calculate_block_position(&mut self, containing_block: Dimensions) {
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
  fn layout_block_children(&mut self) {
    for child in &mut self.children {
      child.layout(self.dimensions);
      // Increment the height so each child is laid out below the previous one
      self.dimensions.content.height =
        self.dimensions.content.height + child.dimensions.margin_box().height();
    }
  }

  // Height of a block-level non-replaced element in normal flow with overflow visible
  fn calculate_block_height(&mut self) {
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

  // Lay out a box and its descendants
  fn layout(&mut self, containing_block: Dimensions) {
    match &self.box_type {
      BoxType::BlockNode(_) => self.layout_block(containing_block),
      BoxType::InlineNode(_) => {}  // TODO
      BoxType::AnonymousBlock => {} // TODO
    }
  }
}

// Build the tree of LayoutBoxes, but don't perform any layout calculations yet
fn build_layout_tree<'a>(style_node: &'a style::StyledNode<'a>) -> LayoutBox<'a> {
  // Create the root box
  let mut root: LayoutBox = LayoutBox::new(match style_node.display() {
    style::Display::Block => BoxType::BlockNode(style_node),
    style::Display::Inline => BoxType::InlineNode(style_node),
    style::Display::None => panic!("Root node has display: none."),
  });

  // Create the descendant boxes
  for child in style_node.children() {
    match child.display() {
      style::Display::Block => root.children.push(build_layout_tree(child)),
      style::Display::Inline => root
        .get_inline_container()
        .children
        .push(build_layout_tree(child)),
      style::Display::None => {} // Skip nodes with "display: none;"
    }
  }
  return root;
}

// Transform a style tree into a layout tree
pub fn layout_tree<'a>(
  node: &'a style::StyledNode<'a>,
  mut containing_block: Dimensions,
) -> LayoutBox<'a> {
  // The layout algorithm expects the container height to start at 0
  // TODO: Save the initial containing block height, for calculating percent heights
  containing_block.content.height = 0.0;

  let mut root_box = build_layout_tree(node);
  root_box.layout(containing_block);
  root_box
}

#[cfg(test)]
mod tests {
  use crate::dom;
  use crate::hashmap;
  use crate::layout::*;
  use crate::style;

  // Test the method expanded_by of the Rectangle struct implementation
  #[test]
  fn test_expanded_by() {
    let rectangle: Rectangle = Rectangle::new(10.0, 20.0, 30.0, 40.0);
    let edge: EdgeSizes = EdgeSizes::new(15.0, 10.0, 20.0, 5.0);

    // Assert that the expanded_by method correctly expands the rectangle by the given edge sizes and
    // returns a new rectangle with the expected x, y, width, and height
    let result: Rectangle = rectangle.expanded_by(edge);
    // Assert that the resulting x coordinate is as expected
    assert_eq!(result.x(), 5.0);
    // Assert that the resulting y coordinate is as expected
    assert_eq!(result.y(), 5.0);
    // Assert that the resulting width is as expected
    assert_eq!(result.width(), 45.0);
    // Assert that the resulting height is as expected
    assert_eq!(result.height(), 75.0);
  }

  // Test the method get_inline_container of the LayoutBox struct implementation
  #[test]
  fn test_get_inline_container() {
    // Text node
    let text_node: dom::Node = dom::Node::text("Hello World!".to_string());
    // Node 2 (child): <div class='container-2'>
    let tag_name_2: String = String::from("div");
    let attributes_2: dom::AttributeMap =
      hashmap![String::from("class") => String::from("container-2")];
    let node_2: dom::Node = dom::Node::element(tag_name_2, attributes_2, vec![text_node.clone()]);
    // Node 1 (father): <div class='container-1'>
    let tag_name_1: String = String::from("div");
    let attributes_1: dom::AttributeMap =
      hashmap![String::from("class") => String::from("container-1")];
    let node_1: dom::Node = dom::Node::element(tag_name_1, attributes_1, vec![node_2.clone()]);
    // Stylesheet
    let stylesheet: css::Stylesheet = css::Stylesheet::new(vec![]);
    // Values
    let mut values_2: style::PropertyMap = hashmap![];
    let mut values_1: style::PropertyMap = hashmap![];

    match node_2.node_type() {
      dom::NodeType::Element(element) => {
        values_2 = style::specified_values(&element, &stylesheet);
      }
      _ => {}
    }
    match node_1.node_type() {
      dom::NodeType::Element(element) => {
        values_1 = style::specified_values(&element, &stylesheet);
      }
      _ => {}
    }
    // Style nodes
    let style_node_3: style::StyledNode = style::StyledNode::new(&text_node, hashmap![], vec![]);
    let style_node_2: style::StyledNode =
      style::StyledNode::new(&node_2, values_2, vec![style_node_3.clone()]);
    let style_node_1: style::StyledNode =
      style::StyledNode::new(&node_1, values_1, vec![style_node_2.clone()]);
    // Layout boxes
    let mut root_box: LayoutBox = LayoutBox::new(BoxType::BlockNode(&style_node_1));
    let mut child_box: LayoutBox = LayoutBox::new(BoxType::InlineNode(&style_node_2));
    let text_box: LayoutBox = LayoutBox::new(BoxType::InlineNode(&style_node_3));

    child_box.add_child(text_box.clone());
    root_box.add_child(child_box.clone());

    // Assert that calling get_inline_container on the child box (InlineNode) returns the child box itself
    // and doens't add any other boxes to the child box children
    let result: &mut LayoutBox = &mut child_box.get_inline_container().clone();
    assert_eq!(result, &mut child_box);
    assert_eq!(child_box.children().len(), 1);

    // Assert that calling get_inline_container on the root box (BlockNode) returns a new anonymous block box
    // and add it to the root box children
    assert_eq!(root_box.children().len(), 1);
    let result: &mut LayoutBox = root_box.get_inline_container();
    assert_eq!(result.box_type, BoxType::AnonymousBlock);
    assert_eq!(result.children().len(), 0);
    assert_eq!(root_box.children().len(), 2);
  }

  // Test the method calculate_block_width of the LayoutBox struct implementation
  #[test]
  fn test_calculate_block_width() {
    // Node: <div class='container-1'>
    let tag_name: String = String::from("div");
    let attributes: dom::AttributeMap =
      hashmap![String::from("class") => String::from("container-1")];
    let node: dom::Node = dom::Node::element(tag_name, attributes, vec![]);
    // Selector
    let simple_selector: css::SimpleSelector =
      css::SimpleSelector::new(None, None, vec!["container-1".to_string()]);
    let selector: css::Selector = css::Selector::Simple(simple_selector);
    // Declarations
    let width_unit: css::Value = css::Value::Length(50.0, css::Unit::Px);
    let width_declaration: css::Declaration =
      css::Declaration::new("width".to_string(), width_unit);

    let height_unit: css::Value = css::Value::Length(50.0, css::Unit::Px);
    let height_declaration: css::Declaration =
      css::Declaration::new("height".to_string(), height_unit);

    let padding_left_unit: css::Value = css::Value::Length(5.0, css::Unit::Px);
    let padding_left_declaration: css::Declaration =
      css::Declaration::new("padding-left".to_string(), padding_left_unit);

    let padding_right_unit: css::Value = css::Value::Length(5.0, css::Unit::Px);
    let padding_right_declaration: css::Declaration =
      css::Declaration::new("padding-right".to_string(), padding_right_unit);

    let border_left_width_unit: css::Value = css::Value::Length(1.0, css::Unit::Px);
    let border_left_width_declaration: css::Declaration =
      css::Declaration::new("border-left-width".to_string(), border_left_width_unit);

    let border_right_width_unit: css::Value = css::Value::Length(1.0, css::Unit::Px);
    let border_right_width_declaration: css::Declaration =
      css::Declaration::new("border-right-width".to_string(), border_right_width_unit);

    let margin_left_unit: css::Value = css::Value::Length(10.0, css::Unit::Px);
    let margin_left_declaration: css::Declaration =
      css::Declaration::new("margin-left".to_string(), margin_left_unit);

    let margin_right_unit: css::Value = css::Value::Length(10.0, css::Unit::Px);
    let margin_right_declaration: css::Declaration =
      css::Declaration::new("margin-right".to_string(), margin_right_unit);
    // Rule
    let rule: css::Rule = css::Rule::new(
      vec![selector],
      vec![
        width_declaration,
        height_declaration,
        padding_left_declaration,
        padding_right_declaration,
        border_left_width_declaration,
        border_right_width_declaration,
        margin_left_declaration,
        margin_right_declaration,
      ],
    );
    // Stylesheet
    let stylesheet: css::Stylesheet = css::Stylesheet::new(vec![rule.clone()]);
    // Value
    let mut values: style::PropertyMap = hashmap![];

    match node.node_type() {
      dom::NodeType::Element(element) => {
        values = style::specified_values(&element, &stylesheet);
      }
      _ => {}
    }
    // StyleNode
    let style_node: StyledNode = StyledNode::new(&node, values, vec![]);
    // LayoutBox
    let mut layout_box: LayoutBox = LayoutBox::new(BoxType::BlockNode(&style_node));
    // Containing block
    let content: Rectangle = Rectangle::new(0.0, 0.0, 100.0, 100.0);
    let padding: EdgeSizes = EdgeSizes::new(0.0, 0.0, 0.0, 0.0);
    let border: EdgeSizes = EdgeSizes::new(0.0, 0.0, 0.0, 0.0);
    let margin: EdgeSizes = EdgeSizes::new(0.0, 0.0, 0.0, 0.0);

    let containing_block: Dimensions = Dimensions::new(content, padding, border, margin);

    // Assert that the calculate_block_width method correctly calculates the width of the layout box given the containing block
    layout_box.calculate_block_width(containing_block);

    // Assert that the resulting content width is as expected
    assert_eq!(layout_box.dimensions().content().width(), 50.0);

    // Assert that the resulting left padding is as expected
    assert_eq!(layout_box.dimensions().padding().left(), 5.0);
    // Assert that the resulting right padding is as expected
    assert_eq!(layout_box.dimensions().padding().right(), 5.0);

    // Assert that the resulting left border is as expected
    assert_eq!(layout_box.dimensions().border().left(), 1.0);
    // Assert that the resulting right border is as expected
    assert_eq!(layout_box.dimensions().border().right(), 1.0);

    // Assert that the resulting left margin is as expected
    assert_eq!(layout_box.dimensions().margin().left(), 10.0);
    // The margin-right from the stylesheet is 10.0px, but the total margin from
    // the right side of the containing block is 28.0px
    // Assert that the resulting right margin is as expected
    assert_eq!(layout_box.dimensions().margin().right(), 28.0);
  }

  // Test the method calculate_block_position of the LayoutBox struct implementation
  #[test]
  fn test_calculate_block_position() {
    // Node: <div class='container-1'>
    let tag_name: String = String::from("div");
    let attributes: dom::AttributeMap =
      hashmap![String::from("class") => String::from("container-1")];
    let node: dom::Node = dom::Node::element(tag_name, attributes, vec![]);
    // Selector
    let simple_selector: css::SimpleSelector =
      css::SimpleSelector::new(None, None, vec!["container-1".to_string()]);
    let selector: css::Selector = css::Selector::Simple(simple_selector);
    // Declarations
    let width_unit: css::Value = css::Value::Length(50.0, css::Unit::Px);
    let width_declaration: css::Declaration =
      css::Declaration::new("width".to_string(), width_unit);

    let height_unit: css::Value = css::Value::Length(50.0, css::Unit::Px);
    let height_declaration: css::Declaration =
      css::Declaration::new("height".to_string(), height_unit);

    let padding_top_unit: css::Value = css::Value::Length(5.0, css::Unit::Px);
    let padding_top_declaration: css::Declaration =
      css::Declaration::new("padding-top".to_string(), padding_top_unit);

    let padding_bottom_unit: css::Value = css::Value::Length(5.0, css::Unit::Px);
    let padding_bottom_declaration: css::Declaration =
      css::Declaration::new("padding-bottom".to_string(), padding_bottom_unit);

    let padding_left_unit: css::Value = css::Value::Length(5.0, css::Unit::Px);
    let padding_left_declaration: css::Declaration =
      css::Declaration::new("padding-left".to_string(), padding_left_unit);

    let border_top_width_unit: css::Value = css::Value::Length(1.0, css::Unit::Px);
    let border_top_width_declaration: css::Declaration =
      css::Declaration::new("border-top-width".to_string(), border_top_width_unit);

    let border_bottom_width_unit: css::Value = css::Value::Length(1.0, css::Unit::Px);
    let border_bottom_width_declaration: css::Declaration =
      css::Declaration::new("border-bottom-width".to_string(), border_bottom_width_unit);

    let border_left_width_unit: css::Value = css::Value::Length(1.0, css::Unit::Px);
    let border_left_width_declaration: css::Declaration =
      css::Declaration::new("border-left-width".to_string(), border_left_width_unit);

    let margin_top_unit: css::Value = css::Value::Length(10.0, css::Unit::Px);
    let margin_top_declaration: css::Declaration =
      css::Declaration::new("margin-top".to_string(), margin_top_unit);

    let margin_bottom_unit: css::Value = css::Value::Length(10.0, css::Unit::Px);
    let margin_bottom_declaration: css::Declaration =
      css::Declaration::new("margin-bottom".to_string(), margin_bottom_unit);

    let margin_left_unit: css::Value = css::Value::Length(10.0, css::Unit::Px);
    let margin_left_declaration: css::Declaration =
      css::Declaration::new("margin-left".to_string(), margin_left_unit);
    // Rule
    let rule: css::Rule = css::Rule::new(
      vec![selector],
      vec![
        width_declaration,
        height_declaration,
        padding_top_declaration,
        padding_bottom_declaration,
        padding_left_declaration,
        border_top_width_declaration,
        border_bottom_width_declaration,
        border_left_width_declaration,
        margin_top_declaration,
        margin_bottom_declaration,
        margin_left_declaration,
      ],
    );
    // Stylesheet
    let stylesheet: css::Stylesheet = css::Stylesheet::new(vec![rule.clone()]);
    // Value
    let mut values: style::PropertyMap = hashmap![];

    match node.node_type() {
      dom::NodeType::Element(element) => {
        values = style::specified_values(&element, &stylesheet);
      }
      _ => {}
    }
    // StyleNode
    let style_node: StyledNode = StyledNode::new(&node, values, vec![]);
    // LayoutBox
    let mut layout_box: LayoutBox = LayoutBox::new(BoxType::BlockNode(&style_node));
    // Containing block
    let content: Rectangle = Rectangle::new(0.0, 0.0, 100.0, 100.0);
    let padding: EdgeSizes = EdgeSizes::new(0.0, 0.0, 0.0, 0.0);
    let border: EdgeSizes = EdgeSizes::new(0.0, 0.0, 0.0, 0.0);
    let margin: EdgeSizes = EdgeSizes::new(0.0, 0.0, 0.0, 0.0);

    let containing_block: Dimensions = Dimensions::new(content, padding, border, margin);

    layout_box.calculate_block_width(containing_block);
    // Assert that the calculate_block_position method correctly calculates the position of the layout box given the containing block
    layout_box.calculate_block_position(containing_block);

    // Assert that the resulting content x position is as expected
    assert_eq!(layout_box.dimensions().content().x(), 16.0);
    // Assert that the resulting content y position is as expected
    assert_eq!(layout_box.dimensions().content().y(), 116.0);

    // Assert that the resulting top padding is as expected
    assert_eq!(layout_box.dimensions().padding().top(), 5.0);
    // Assert that the resulting bottom padding is as expected
    assert_eq!(layout_box.dimensions().padding().bottom(), 5.0);
    // Assert that the resulting left padding is as expected
    assert_eq!(layout_box.dimensions().padding().left(), 5.0);

    // Assert that the resulting top border is as expected
    assert_eq!(layout_box.dimensions().border().top(), 1.0);
    // Assert that the resulting bottom border is as expected
    assert_eq!(layout_box.dimensions().border().bottom(), 1.0);
    // Assert that the resulting left border is as expected
    assert_eq!(layout_box.dimensions().border().left(), 1.0);

    // Assert that the resulting top margin is as expected
    assert_eq!(layout_box.dimensions().margin().top(), 10.0);
    // Assert that the resulting bottom margin is as expected
    assert_eq!(layout_box.dimensions().margin().bottom(), 10.0);
    // Assert that the resulting left margin is as expected
    assert_eq!(layout_box.dimensions().margin().left(), 10.0);
  }

  // Test the method layout_block_children of the LayoutBox struct implementation
  #[test]
  fn test_layout_block_children() {
    // Text node
    let text_node: dom::Node = dom::Node::text("Hello World!".to_string());
    // Node 2 (child): <div class='container-2'>
    let tag_name_2: String = String::from("div");
    let attributes_2: dom::AttributeMap =
      hashmap![String::from("class") => String::from("container-2")];
    let node_2: dom::Node = dom::Node::element(tag_name_2, attributes_2, vec![text_node.clone()]);
    // Node 1 (father): <div class='container-1'>
    let tag_name_1: String = String::from("div");
    let attributes_1: dom::AttributeMap =
      hashmap![String::from("class") => String::from("container-1")];
    let node_1: dom::Node = dom::Node::element(tag_name_1, attributes_1, vec![node_2.clone()]);
    // SelectorS
    let simple_selector_1: css::SimpleSelector =
      css::SimpleSelector::new(None, None, vec!["container-1".to_string()]);
    let selector_1: css::Selector = css::Selector::Simple(simple_selector_1);
    let simple_selector_2: css::SimpleSelector =
      css::SimpleSelector::new(None, None, vec!["container-2".to_string()]);
    let selector_2: css::Selector = css::Selector::Simple(simple_selector_2);
    // Declarations
    let display_declaraion: css::Declaration = css::Declaration::new(
      "display".to_string(),
      css::Value::Keyword("block".to_string()),
    );

    let width_unit_1: css::Value = css::Value::Length(100.0, css::Unit::Px);
    let width_declaration_1: css::Declaration =
      css::Declaration::new("width".to_string(), width_unit_1);

    let height_unit_1: css::Value = css::Value::Length(100.0, css::Unit::Px);
    let height_declaration_1: css::Declaration =
      css::Declaration::new("height".to_string(), height_unit_1);

    let width_unit_2: css::Value = css::Value::Length(50.0, css::Unit::Px);
    let width_declaration_2: css::Declaration =
      css::Declaration::new("width".to_string(), width_unit_2);

    let height_unit_2: css::Value = css::Value::Length(50.0, css::Unit::Px);
    let height_declaration_2: css::Declaration =
      css::Declaration::new("height".to_string(), height_unit_2);

    let padding_top_unit: css::Value = css::Value::Length(5.0, css::Unit::Px);
    let padding_top_declaration: css::Declaration =
      css::Declaration::new("padding-top".to_string(), padding_top_unit);

    let padding_bottom_unit: css::Value = css::Value::Length(5.0, css::Unit::Px);
    let padding_bottom_declaration: css::Declaration =
      css::Declaration::new("padding-bottom".to_string(), padding_bottom_unit);

    let border_top_width_unit: css::Value = css::Value::Length(1.0, css::Unit::Px);
    let border_top_width_declaration: css::Declaration =
      css::Declaration::new("border-top-width".to_string(), border_top_width_unit);

    let border_bottom_width_unit: css::Value = css::Value::Length(1.0, css::Unit::Px);
    let border_bottom_width_declaration: css::Declaration =
      css::Declaration::new("border-bottom-width".to_string(), border_bottom_width_unit);

    let margin_top_unit: css::Value = css::Value::Length(10.0, css::Unit::Px);
    let margin_top_declaration: css::Declaration =
      css::Declaration::new("margin-top".to_string(), margin_top_unit);

    let margin_bottom_unit: css::Value = css::Value::Length(10.0, css::Unit::Px);
    let margin_bottom_declaration: css::Declaration =
      css::Declaration::new("margin-bottom".to_string(), margin_bottom_unit);
    // Rules
    let rule_1: css::Rule = css::Rule::new(
      vec![selector_1],
      vec![
        display_declaraion,
        width_declaration_1,
        height_declaration_1,
      ],
    );
    let rule_2: css::Rule = css::Rule::new(
      vec![selector_2],
      vec![
        width_declaration_2,
        height_declaration_2,
        padding_top_declaration,
        padding_bottom_declaration,
        border_top_width_declaration,
        border_bottom_width_declaration,
        margin_top_declaration,
        margin_bottom_declaration,
      ],
    );
    // Stylesheet
    let stylesheet: css::Stylesheet = css::Stylesheet::new(vec![rule_1, rule_2]);
    // Values
    let mut values_2: style::PropertyMap = hashmap![];
    let mut values_1: style::PropertyMap = hashmap![];

    match node_2.node_type() {
      dom::NodeType::Element(element) => {
        values_2 = style::specified_values(&element, &stylesheet);
      }
      _ => {}
    }
    match node_1.node_type() {
      dom::NodeType::Element(element) => {
        values_1 = style::specified_values(&element, &stylesheet);
      }
      _ => {}
    }
    // Style nodes
    let style_node_3: style::StyledNode = style::StyledNode::new(&text_node, hashmap![], vec![]);
    let style_node_2: style::StyledNode =
      style::StyledNode::new(&node_2, values_2, vec![style_node_3.clone()]);
    let style_node_1: style::StyledNode =
      style::StyledNode::new(&node_1, values_1, vec![style_node_2.clone()]);
    // Layout boxes
    let mut root_box: LayoutBox = LayoutBox::new(BoxType::BlockNode(&style_node_1));
    let mut child_box: LayoutBox = LayoutBox::new(BoxType::InlineNode(&style_node_2));
    let text_box: LayoutBox = LayoutBox::new(BoxType::InlineNode(&style_node_3));

    child_box.add_child(text_box.clone());
    child_box.calculate_block_width(*root_box.dimensions());
    child_box.calculate_block_position(*root_box.dimensions());
    child_box.calculate_block_height();
    root_box.add_child(child_box);

    // Containing block
    let content: Rectangle = Rectangle::new(0.0, 0.0, 200.0, 200.0);
    let padding: EdgeSizes = EdgeSizes::new(0.0, 0.0, 0.0, 0.0);
    let border: EdgeSizes = EdgeSizes::new(0.0, 0.0, 0.0, 0.0);
    let margin: EdgeSizes = EdgeSizes::new(0.0, 0.0, 0.0, 0.0);

    let containing_block: Dimensions = Dimensions::new(content, padding, border, margin);

    root_box.calculate_block_width(containing_block);
    root_box.calculate_block_position(containing_block);
    // Assert that the layout_block_children method correctly calculates the height of the layout box by its children height
    root_box.layout_block_children();

    // Assert that the resulting content height is as expected
    assert_eq!(root_box.dimensions().content().height(), 82.0);
  }

  // Test the function build_layout_tree
  #[test]
  fn test_build_layout_tree() {
    // Text node
    let text_node: dom::Node = dom::Node::text("Hello World!".to_string());
    // Node 2 (child): <div class='container-2'>
    let tag_name_2: String = String::from("div");
    let attributes_2: dom::AttributeMap =
      hashmap![String::from("class") => String::from("container-2")];
    let node_2: dom::Node = dom::Node::element(tag_name_2, attributes_2, vec![text_node.clone()]);
    // Node 1 (father): <div class='container-1'>
    let tag_name_1: String = String::from("div");
    let attributes_1: dom::AttributeMap =
      hashmap![String::from("class") => String::from("container-1")];
    let node_1: dom::Node = dom::Node::element(tag_name_1, attributes_1, vec![node_2.clone()]);
    // SelectorS
    let simple_selector_1: css::SimpleSelector =
      css::SimpleSelector::new(None, None, vec!["container-1".to_string()]);
    let selector_1: css::Selector = css::Selector::Simple(simple_selector_1);
    let simple_selector_2: css::SimpleSelector =
      css::SimpleSelector::new(None, None, vec!["container-2".to_string()]);
    let selector_2: css::Selector = css::Selector::Simple(simple_selector_2);
    // Declarations
    let display_declaraion: css::Declaration = css::Declaration::new(
      "display".to_string(),
      css::Value::Keyword("block".to_string()),
    );

    let width_unit_1: css::Value = css::Value::Length(100.0, css::Unit::Px);
    let width_declaration_1: css::Declaration =
      css::Declaration::new("width".to_string(), width_unit_1);

    let height_unit_1: css::Value = css::Value::Length(100.0, css::Unit::Px);
    let height_declaration_1: css::Declaration =
      css::Declaration::new("height".to_string(), height_unit_1);

    let width_unit_2: css::Value = css::Value::Length(50.0, css::Unit::Px);
    let width_declaration_2: css::Declaration =
      css::Declaration::new("width".to_string(), width_unit_2);

    let height_unit_2: css::Value = css::Value::Length(50.0, css::Unit::Px);
    let height_declaration_2: css::Declaration =
      css::Declaration::new("height".to_string(), height_unit_2);
    // Rules
    let rule_1: css::Rule = css::Rule::new(
      vec![selector_1],
      vec![
        display_declaraion,
        width_declaration_1,
        height_declaration_1,
      ],
    );
    let rule_2: css::Rule = css::Rule::new(
      vec![selector_2],
      vec![width_declaration_2, height_declaration_2],
    );
    // Stylesheet
    let stylesheet: css::Stylesheet = css::Stylesheet::new(vec![rule_1, rule_2]);
    // Values
    let mut values_2: style::PropertyMap = hashmap![];
    let mut values_1: style::PropertyMap = hashmap![];

    match node_2.node_type() {
      dom::NodeType::Element(element) => {
        values_2 = style::specified_values(&element, &stylesheet);
      }
      _ => {}
    }
    match node_1.node_type() {
      dom::NodeType::Element(element) => {
        values_1 = style::specified_values(&element, &stylesheet);
      }
      _ => {}
    }
    // Style nodes
    let style_node_3: style::StyledNode = style::StyledNode::new(&text_node, hashmap![], vec![]);
    let style_node_2: style::StyledNode =
      style::StyledNode::new(&node_2, values_2, vec![style_node_3.clone()]);
    let style_node_1: style::StyledNode =
      style::StyledNode::new(&node_1, values_1, vec![style_node_2.clone()]);
    // Layout boxes
    let mut root_box: LayoutBox = LayoutBox::new(BoxType::BlockNode(&style_node_1));
    let mut anonymous_box: LayoutBox = LayoutBox::new(BoxType::AnonymousBlock);
    let mut child_box: LayoutBox = LayoutBox::new(BoxType::InlineNode(&style_node_2));
    let text_box: LayoutBox = LayoutBox::new(BoxType::InlineNode(&style_node_3));

    child_box.add_child(text_box.clone());
    anonymous_box.add_child(child_box.clone());
    root_box.add_child(anonymous_box.clone());

    // Assert that the build_layout_tree function correctly matches the layout tree with the right layout boxes
    let layout_tree_root: LayoutBox = build_layout_tree(&style_node_1);

    // Assert that the resulting root_box is as expected
    assert_eq!(layout_tree_root, root_box);
    // Assert that the resulting anonymous_box is as expected
    assert_eq!(layout_tree_root.children()[0], anonymous_box);
    // Assert that the resulting child_box is as expected
    assert_eq!(layout_tree_root.children()[0].children()[0], child_box);
    // Assert that the resulting text_box is as expected
    assert_eq!(
      layout_tree_root.children()[0].children()[0].children()[0],
      text_box
    );
  }
}
