use crate::css;
use crate::dom;
use crate::hashmap;
use crate::layout::*;
use crate::style;

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
  let style_node: style::StyledNode = style::StyledNode::new(&node, values, vec![]);
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
  let style_node: style::StyledNode = style::StyledNode::new(&node, values, vec![]);
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
