use crate::css;
use crate::dom;
use crate::hashmap;
use crate::layout::*;
use crate::style;
use super::build_layout_tree;

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
