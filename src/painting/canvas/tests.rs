use crate::css;
use crate::dom;
use crate::hashmap;
use crate::layout;
use crate::style;
use super::Canvas;
use crate::painting::display_list::{render_background, DisplayCommand, DisplayList};

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
