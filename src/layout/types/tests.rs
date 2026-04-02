use crate::css;
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
