use crate::css;
use crate::dom;
use crate::hashmap;
use crate::layout::*;
use crate::style;
use super::tree::build_layout_tree;

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

// Test the method layout_inline of the LayoutBox struct implementation
#[test]
fn test_layout_inline() {
  let tag_name: String = String::from("span");
  let attributes: dom::AttributeMap =
    hashmap![String::from("class") => String::from("inline-1")];
  let node: dom::Node = dom::Node::element(tag_name, attributes, vec![]);
  let simple_selector: css::SimpleSelector =
    css::SimpleSelector::new(None, None, vec!["inline-1".to_string()]);
  let selector: css::Selector = css::Selector::Simple(simple_selector);
  let rule: css::Rule = css::Rule::new(
    vec![selector],
    vec![
      css::Declaration::new("width".to_string(), css::Value::Length(80.0, css::Unit::Px)),
      css::Declaration::new("height".to_string(), css::Value::Length(20.0, css::Unit::Px)),
      css::Declaration::new("padding".to_string(), css::Value::Length(4.0, css::Unit::Px)),
      css::Declaration::new("border-width".to_string(), css::Value::Length(2.0, css::Unit::Px)),
      css::Declaration::new("margin".to_string(), css::Value::Length(3.0, css::Unit::Px)),
    ],
  );
  let stylesheet: css::Stylesheet = css::Stylesheet::new(vec![rule]);
  let mut values: style::PropertyMap = hashmap![];
  match node.node_type() {
    dom::NodeType::Element(element) => {
      values = style::specified_values(&element, &stylesheet);
    }
    _ => {}
  }
  let style_node: style::StyledNode = style::StyledNode::new(&node, values, vec![]);
  let mut layout_box: LayoutBox = LayoutBox::new(BoxType::InlineNode(&style_node));
  let containing_block: Dimensions = Dimensions::new(
    Rectangle::new(0.0, 0.0, 200.0, 100.0),
    EdgeSizes::new(0.0, 0.0, 0.0, 0.0),
    EdgeSizes::new(0.0, 0.0, 0.0, 0.0),
    EdgeSizes::new(0.0, 0.0, 0.0, 0.0),
  );

  layout_box.layout_inline(containing_block);

  assert_eq!(layout_box.dimensions().content().width(), 80.0);
  assert_eq!(layout_box.dimensions().content().height(), 20.0);
  assert_eq!(layout_box.dimensions().padding().top(), 4.0);
  assert_eq!(layout_box.dimensions().padding().left(), 4.0);
  assert_eq!(layout_box.dimensions().border().top(), 2.0);
  assert_eq!(layout_box.dimensions().border().left(), 2.0);
  assert_eq!(layout_box.dimensions().margin().top(), 3.0);
  assert_eq!(layout_box.dimensions().margin().left(), 3.0);
}

// Test the method layout_anonymous_block of the LayoutBox struct implementation.
// Two inline children fit side by side on a single line.
#[test]
fn test_layout_anonymous_block_single_line() {
  let node_1: dom::Node = dom::Node::element(
    "span".to_string(),
    hashmap![String::from("class") => String::from("a")],
    vec![],
  );
  let node_2: dom::Node = dom::Node::element(
    "span".to_string(),
    hashmap![String::from("class") => String::from("b")],
    vec![],
  );
  let stylesheet: css::Stylesheet = css::Stylesheet::new(vec![
    css::Rule::new(
      vec![css::Selector::Simple(css::SimpleSelector::new(None, None, vec!["a".to_string()]))],
      vec![
        css::Declaration::new("width".to_string(), css::Value::Length(50.0, css::Unit::Px)),
        css::Declaration::new("height".to_string(), css::Value::Length(30.0, css::Unit::Px)),
      ],
    ),
    css::Rule::new(
      vec![css::Selector::Simple(css::SimpleSelector::new(None, None, vec!["b".to_string()]))],
      vec![
        css::Declaration::new("width".to_string(), css::Value::Length(60.0, css::Unit::Px)),
        css::Declaration::new("height".to_string(), css::Value::Length(20.0, css::Unit::Px)),
      ],
    ),
  ]);
  let mut values_1: style::PropertyMap = hashmap![];
  let mut values_2: style::PropertyMap = hashmap![];
  match node_1.node_type() {
    dom::NodeType::Element(element) => values_1 = style::specified_values(&element, &stylesheet),
    _ => {}
  }
  match node_2.node_type() {
    dom::NodeType::Element(element) => values_2 = style::specified_values(&element, &stylesheet),
    _ => {}
  }
  let style_node_1: style::StyledNode = style::StyledNode::new(&node_1, values_1, vec![]);
  let style_node_2: style::StyledNode = style::StyledNode::new(&node_2, values_2, vec![]);
  let child_box_1: LayoutBox = LayoutBox::new(BoxType::InlineNode(&style_node_1));
  let child_box_2: LayoutBox = LayoutBox::new(BoxType::InlineNode(&style_node_2));
  let mut anon_box: LayoutBox = LayoutBox::new(BoxType::AnonymousBlock);
  anon_box.add_child(child_box_1);
  anon_box.add_child(child_box_2);

  let containing_block: Dimensions = Dimensions::new(
    Rectangle::new(0.0, 0.0, 200.0, 0.0),
    EdgeSizes::new(0.0, 0.0, 0.0, 0.0),
    EdgeSizes::new(0.0, 0.0, 0.0, 0.0),
    EdgeSizes::new(0.0, 0.0, 0.0, 0.0),
  );

  anon_box.layout_anonymous_block(containing_block);

  // Both children fit on one line: child 1 at x=0, child 2 at x=50
  assert_eq!(anon_box.children()[0].dimensions().content().x(), 0.0);
  assert_eq!(anon_box.children()[0].dimensions().content().y(), 0.0);
  assert_eq!(anon_box.children()[1].dimensions().content().x(), 50.0);
  assert_eq!(anon_box.children()[1].dimensions().content().y(), 0.0);
  // Height equals the tallest child on the line
  assert_eq!(anon_box.dimensions().content().height(), 30.0);
}

// Test that layout_anonymous_block wraps to a new line when a child no longer fits.
#[test]
fn test_layout_anonymous_block_wrapping() {
  let node_1: dom::Node = dom::Node::element(
    "span".to_string(),
    hashmap![String::from("class") => String::from("a")],
    vec![],
  );
  let node_2: dom::Node = dom::Node::element(
    "span".to_string(),
    hashmap![String::from("class") => String::from("b")],
    vec![],
  );
  let stylesheet: css::Stylesheet = css::Stylesheet::new(vec![
    css::Rule::new(
      vec![css::Selector::Simple(css::SimpleSelector::new(None, None, vec!["a".to_string()]))],
      vec![
        css::Declaration::new("width".to_string(), css::Value::Length(50.0, css::Unit::Px)),
        css::Declaration::new("height".to_string(), css::Value::Length(30.0, css::Unit::Px)),
      ],
    ),
    css::Rule::new(
      vec![css::Selector::Simple(css::SimpleSelector::new(None, None, vec!["b".to_string()]))],
      vec![
        css::Declaration::new("width".to_string(), css::Value::Length(60.0, css::Unit::Px)),
        css::Declaration::new("height".to_string(), css::Value::Length(20.0, css::Unit::Px)),
      ],
    ),
  ]);
  let mut values_1: style::PropertyMap = hashmap![];
  let mut values_2: style::PropertyMap = hashmap![];
  match node_1.node_type() {
    dom::NodeType::Element(element) => values_1 = style::specified_values(&element, &stylesheet),
    _ => {}
  }
  match node_2.node_type() {
    dom::NodeType::Element(element) => values_2 = style::specified_values(&element, &stylesheet),
    _ => {}
  }
  let style_node_1: style::StyledNode = style::StyledNode::new(&node_1, values_1, vec![]);
  let style_node_2: style::StyledNode = style::StyledNode::new(&node_2, values_2, vec![]);
  let child_box_1: LayoutBox = LayoutBox::new(BoxType::InlineNode(&style_node_1));
  let child_box_2: LayoutBox = LayoutBox::new(BoxType::InlineNode(&style_node_2));
  let mut anon_box: LayoutBox = LayoutBox::new(BoxType::AnonymousBlock);
  anon_box.add_child(child_box_1);
  anon_box.add_child(child_box_2);

  // Container is only 60px wide: child 1 (50px) fits, child 2 (60px) wraps
  let containing_block: Dimensions = Dimensions::new(
    Rectangle::new(0.0, 0.0, 60.0, 0.0),
    EdgeSizes::new(0.0, 0.0, 0.0, 0.0),
    EdgeSizes::new(0.0, 0.0, 0.0, 0.0),
    EdgeSizes::new(0.0, 0.0, 0.0, 0.0),
  );

  anon_box.layout_anonymous_block(containing_block);

  // Child 1 stays on line 0
  assert_eq!(anon_box.children()[0].dimensions().content().x(), 0.0);
  assert_eq!(anon_box.children()[0].dimensions().content().y(), 0.0);
  // Child 2 wraps to line 1 (y = height of line 0 = 30)
  assert_eq!(anon_box.children()[1].dimensions().content().x(), 0.0);
  assert_eq!(anon_box.children()[1].dimensions().content().y(), 30.0);
  // Total height = line 0 (30) + line 1 (20)
  assert_eq!(anon_box.dimensions().content().height(), 50.0);
}
