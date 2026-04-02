use crate::css;
use crate::dom;
use crate::hashmap;
use crate::layout;
use crate::style;
use super::canvas::Canvas;
use super::display_list::{
  build_display_list, get_color, render_background, render_borders, render_text, DisplayCommand,
  DisplayList,
};

// Test the function get_color
#[test]
fn test_get_color() {
  // Node: <div class='container-1'>
  let tag_name: String = String::from("div");
  let attributes: dom::AttributeMap =
    hashmap![String::from("class") => String::from("container-1")];
  let node: dom::Node = dom::Node::element(tag_name, attributes, vec![]);
  // Selector
  let simple_selector: css::SimpleSelector =
    css::SimpleSelector::new(None, None, vec!["container-1".to_string()]);
  let selector: css::Selector = css::Selector::Simple(simple_selector);
  // Declaration
  let background_unit: css::Value = css::Value::ColorValue(css::Color::new(255, 0, 0, 255));
  let background_declaration: css::Declaration =
    css::Declaration::new("background".to_string(), background_unit);
  // Rule
  let rule: css::Rule = css::Rule::new(vec![selector], vec![background_declaration]);
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
  let style_node: style::StyledNode = style::StyledNode::new(&node, values, vec![]);
  // LayoutBox
  let layout_box: layout::LayoutBox =
    layout::LayoutBox::new(layout::BoxType::BlockNode(&style_node));

  assert_eq!(
    get_color(&layout_box, "background"),
    Some(css::Color::new(255, 0, 0, 255))
  );
}

// Test the function render_background
#[test]
fn test_render_background() {
  // Node: <div class='container-1'>
  let tag_name: String = String::from("div");
  let attributes: dom::AttributeMap =
    hashmap![String::from("class") => String::from("container-1")];
  let node: dom::Node = dom::Node::element(tag_name, attributes, vec![]);
  // Selector
  let simple_selector: css::SimpleSelector =
    css::SimpleSelector::new(None, None, vec!["container-1".to_string()]);
  let selector: css::Selector = css::Selector::Simple(simple_selector);
  // Declaration
  let background_unit: css::Value = css::Value::ColorValue(css::Color::new(255, 0, 0, 255));
  let background_declaration: css::Declaration =
    css::Declaration::new("background".to_string(), background_unit);
  // Rule
  let rule: css::Rule = css::Rule::new(vec![selector], vec![background_declaration]);
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
  let style_node: style::StyledNode = style::StyledNode::new(&node, values, vec![]);
  // LayoutBox
  let layout_box: layout::LayoutBox =
    layout::LayoutBox::new(layout::BoxType::BlockNode(&style_node));
  // Display list
  let mut display_list: DisplayList = vec![];

  // Assert that the render_background function correctly add the layout_box background and border box to the display_list
  render_background(&mut display_list, &layout_box);

  // Assert that the first element of the display list is a SolidColor DisplayCommand with the right color and border box
  assert_eq!(
    display_list[0],
    DisplayCommand::SolidColor(
      css::Color::new(255, 0, 0, 255),
      layout_box.dimensions().border_box()
    )
  );
}

// Test the function render_borders
#[test]
fn test_render_borders() {
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
  let border_color_unit: css::Value = css::Value::ColorValue(css::Color::new(255, 0, 0, 255));
  let border_color_declaration: css::Declaration =
    css::Declaration::new("border-color".to_string(), border_color_unit);

  let border_top_width_unit: css::Value = css::Value::Length(1.0, css::Unit::Px);
  let border_top_width_declaration: css::Declaration =
    css::Declaration::new("border-top-width".to_string(), border_top_width_unit);

  let border_right_width_unit: css::Value = css::Value::Length(1.0, css::Unit::Px);
  let border_right_width_declaration: css::Declaration =
    css::Declaration::new("border-right-width".to_string(), border_right_width_unit);

  let border_bottom_width_unit: css::Value = css::Value::Length(1.0, css::Unit::Px);
  let border_bottom_width_declaration: css::Declaration =
    css::Declaration::new("border-bottom-width".to_string(), border_bottom_width_unit);

  let border_left_width_unit: css::Value = css::Value::Length(1.0, css::Unit::Px);
  let border_left_width_declaration: css::Declaration =
    css::Declaration::new("border-left-width".to_string(), border_left_width_unit);
  // Rule
  let rule: css::Rule = css::Rule::new(
    vec![selector],
    vec![
      border_color_declaration,
      border_top_width_declaration,
      border_right_width_declaration,
      border_bottom_width_declaration,
      border_left_width_declaration,
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
  let style_node: style::StyledNode = style::StyledNode::new(&node, values, vec![]);
  // LayoutBox
  let layout_box: layout::LayoutBox =
    layout::LayoutBox::new(layout::BoxType::BlockNode(&style_node));
  // Display list
  let mut display_list: DisplayList = vec![];

  // Assert that the render_borders function correctly add the layout_box borders
  render_borders(&mut display_list, &layout_box);

  // Assert that the first element of the display list is a SolidColor DisplayCommand with the right color and top border data
  assert_eq!(
    display_list[0],
    DisplayCommand::SolidColor(
      css::Color::new(255, 0, 0, 255),
      layout::Rectangle::new(
        layout_box.dimensions().border_box().x(),
        layout_box.dimensions().border_box().y(),
        layout_box.dimensions().border_box().width(),
        layout_box.dimensions().border().top()
      )
    )
  );
  // Assert that the first element of the display list is a SolidColor DisplayCommand with the right color and right border data
  assert_eq!(
    display_list[1],
    DisplayCommand::SolidColor(
      css::Color::new(255, 0, 0, 255),
      layout::Rectangle::new(
        layout_box.dimensions().border_box().x()
          + layout_box.dimensions().border_box().width()
          - layout_box.dimensions().border().right(),
        layout_box.dimensions().border_box().y(),
        layout_box.dimensions().border().right(),
        layout_box.dimensions().border_box().height()
      )
    )
  );
  // Assert that the first element of the display list is a SolidColor DisplayCommand with the right color and bottom border data
  assert_eq!(
    display_list[2],
    DisplayCommand::SolidColor(
      css::Color::new(255, 0, 0, 255),
      layout::Rectangle::new(
        layout_box.dimensions().border_box().x(),
        layout_box.dimensions().border_box().y() + layout_box.dimensions().border_box().height()
          - layout_box.dimensions().border().bottom(),
        layout_box.dimensions().border_box().width(),
        layout_box.dimensions().border().bottom()
      )
    )
  );
  // Assert that the first element of the display list is a SolidColor DisplayCommand with the right color and left border data
  assert_eq!(
    display_list[3],
    DisplayCommand::SolidColor(
      css::Color::new(255, 0, 0, 255),
      layout::Rectangle::new(
        layout_box.dimensions().border_box().x(),
        layout_box.dimensions().border_box().y(),
        layout_box.dimensions().border().left(),
        layout_box.dimensions().border_box().height()
      )
    )
  );
}

// Test that render_text adds a DrawText command for an inline text node.
#[test]
fn test_render_text() {
  let text_node: dom::Node = dom::Node::text("Hello".to_string());
  let style_node: style::StyledNode = style::StyledNode::new(&text_node, hashmap![], vec![]);
  let layout_box: layout::LayoutBox =
    layout::LayoutBox::new(layout::BoxType::InlineNode(&style_node));
  let mut display_list: DisplayList = vec![];

  render_text(&mut display_list, &layout_box);

  assert_eq!(display_list.len(), 1);
  assert_eq!(
    display_list[0],
    DisplayCommand::DrawText(
      css::Color::new(0, 0, 0, 255),
      *layout_box.dimensions().content(),
      "Hello".to_string(),
      16.0,
    )
  );
}

// Test that render_text skips whitespace-only text nodes.
#[test]
fn test_render_text_skips_whitespace() {
  let text_node: dom::Node = dom::Node::text("   \n  ".to_string());
  let style_node: style::StyledNode = style::StyledNode::new(&text_node, hashmap![], vec![]);
  let layout_box: layout::LayoutBox =
    layout::LayoutBox::new(layout::BoxType::InlineNode(&style_node));
  let mut display_list: DisplayList = vec![];

  render_text(&mut display_list, &layout_box);

  assert_eq!(display_list.len(), 0);
}

// Test the method paint_item of the Canvas struct implementation
#[test]
fn test_paint_item() {
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
  let display_declaraion: css::Declaration = css::Declaration::new(
    "display".to_string(),
    css::Value::Keyword("block".to_string()),
  );
  let background_unit: css::Value = css::Value::ColorValue(css::Color::new(255, 0, 0, 255));
  let background_declaration: css::Declaration =
    css::Declaration::new("background".to_string(), background_unit);
  let width_unit: css::Value = css::Value::Length(50.0, css::Unit::Px);
  let width_declaration: css::Declaration =
    css::Declaration::new("width".to_string(), width_unit);
  let height_unit: css::Value = css::Value::Length(50.0, css::Unit::Px);
  let height_declaration: css::Declaration =
    css::Declaration::new("height".to_string(), height_unit);
  // Rule
  let rule: css::Rule = css::Rule::new(
    vec![selector],
    vec![
      display_declaraion,
      width_declaration,
      height_declaration,
      background_declaration,
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
  let style_node: style::StyledNode = style::StyledNode::new(&node, values, vec![]);
  // Containing block
  let content: layout::Rectangle = layout::Rectangle::new(0.0, 0.0, 100.0, 100.0);
  let padding: layout::EdgeSizes = layout::EdgeSizes::new(0.0, 0.0, 0.0, 0.0);
  let border: layout::EdgeSizes = layout::EdgeSizes::new(0.0, 0.0, 0.0, 0.0);
  let margin: layout::EdgeSizes = layout::EdgeSizes::new(0.0, 0.0, 0.0, 0.0);
  let containing_block: layout::Dimensions =
    layout::Dimensions::new(content, padding, border, margin);
  // LayoutBox
  let mut layout_box: layout::LayoutBox =
    layout::LayoutBox::new(layout::BoxType::BlockNode(&style_node));
  layout_box.layout_block(containing_block);
  // Display list
  let mut display_list: DisplayList = vec![];
  render_background(&mut display_list, &layout_box);
  // Canvas
  let mut canvas: Canvas = Canvas::new(200, 200);

  canvas.paint_item(&display_list[0]);

  let mut layout_box_pixels_count: i32 = 0;
  let mut layout_box_pixels_right_position: bool = true;

  for pixel in canvas.pixels() {
    if *pixel == css::Color::new(255, 0, 0, 255) {
      layout_box_pixels_count += 1;
    }
  }

  for row in (20000..30000).step_by(200) {
    for column in 0..49 {
      if canvas.pixels()[row + column] != css::Color::new(255, 0, 0, 255) {
        layout_box_pixels_right_position = false;
      }
    }
  }

  assert_eq!(layout_box_pixels_count, 2500);
  assert_eq!(layout_box_pixels_right_position, true);
}

// Test that painting a semi-transparent color over a white background alpha-blends correctly.
#[test]
fn test_paint_item_alpha_compositing() {
  // 50% transparent red (a = 128 ≈ 0.502) over white (255, 255, 255, 255).
  // out_r = 255 * (128/255) + 255 * (1 - 128/255) = 255
  // out_g = 0   * (128/255) + 255 * (1 - 128/255) ≈ 127
  // out_b = 0   * (128/255) + 255 * (1 - 128/255) ≈ 127
  // out_a = (128/255 + 1.0 * (1 - 128/255)) * 255 = 255
  let src_a: f32 = 128.0 / 255.0;
  let inv_a: f32 = 1.0 - src_a;
  let expected_r: u8 = (255.0 * src_a + 255.0 * inv_a) as u8;
  let expected_g: u8 = (0.0 * src_a + 255.0 * inv_a) as u8;
  let expected_b: u8 = (0.0 * src_a + 255.0 * inv_a) as u8;
  let expected_a: u8 = ((src_a + 1.0 * inv_a) * 255.0) as u8;

  let mut canvas: Canvas = Canvas::new(2, 2);
  let item: DisplayCommand = DisplayCommand::SolidColor(
    css::Color::new(255, 0, 0, 128),
    layout::Rectangle::new(0.0, 0.0, 2.0, 2.0),
  );
  canvas.paint_item(&item);

  for pixel in canvas.pixels() {
    assert_eq!(*pixel, css::Color::new(expected_r, expected_g, expected_b, expected_a));
  }
}
