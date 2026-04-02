use crate::css;
use crate::dom;
use crate::hashmap;
use crate::style::{PropertyMap, StyledNode};
use crate::style::cascade::specified_values;
use super::style_tree;

// Test the function style_tree
#[test]
fn test_style_tree() {
  let tag_name_2: String = String::from("div");
  let attributes_2: dom::AttributeMap =
    hashmap![String::from("class") => String::from("container-1")];
  let node_2: dom::Node = dom::Node::element(tag_name_2, attributes_2, vec![]);
  let tag_name_4: String = String::from("p");
  let attributes_4: dom::AttributeMap =
    hashmap![String::from("class") => String::from("paragraph")];
  let node_4: dom::Node = dom::Node::element(tag_name_4, attributes_4, vec![dom::Node::text("Hello World!".to_string())]);
  let tag_name_3: String = String::from("div");
  let attributes_3: dom::AttributeMap =
    hashmap![String::from("class") => String::from("container-2")];
  let node_3: dom::Node = dom::Node::element(tag_name_3, attributes_3, vec![node_4.clone()]);
  let tag_name_1: String = String::from("html");
  let node_1: dom::Node = dom::Node::element(tag_name_1, hashmap![], vec![node_2.clone(), node_3.clone()]);
  let simple_selector_1: css::SimpleSelector =
    css::SimpleSelector::new(Some("p".to_string()), None, vec!["paragraph".to_string()]);
  let simple_selector_2: css::SimpleSelector =
    css::SimpleSelector::new(None, None, vec!["container-2".to_string()]);
  let selector_1: css::Selector = css::Selector::Simple(simple_selector_1);
  let selector_2: css::Selector = css::Selector::Simple(simple_selector_2);
  let unit: css::Value = css::Value::Length(100.0, css::Unit::Px);
  let declaration_1: css::Declaration = css::Declaration::new("width".to_string(), unit);
  let color: css::Value = css::Value::ColorValue(css::Color::new(163, 228, 215, 255));
  let declaration_2: css::Declaration = css::Declaration::new("background".to_string(), color);
  let rule_1: css::Rule = css::Rule::new(vec![selector_1], vec![declaration_1]);
  let rule_2: css::Rule = css::Rule::new(vec![selector_2], vec![declaration_2]);
  let stylesheet: css::Stylesheet = css::Stylesheet::new(vec![rule_1.clone(), rule_2.clone()]);
  let mut values_2: PropertyMap = hashmap![];
  let mut values_4: PropertyMap = hashmap![];
  let mut values_3: PropertyMap = hashmap![];
  let mut values_1: PropertyMap = hashmap![];

  match node_2.node_type() {
    dom::NodeType::Element(element) => { values_2 = specified_values(&element, &stylesheet); }
    _ => {}
  }
  match node_4.node_type() {
    dom::NodeType::Element(element) => { values_4 = specified_values(&element, &stylesheet); }
    _ => {}
  }
  match node_3.node_type() {
    dom::NodeType::Element(element) => { values_3 = specified_values(&element, &stylesheet); }
    _ => {}
  }
  match node_1.node_type() {
    dom::NodeType::Element(element) => { values_1 = specified_values(&element, &stylesheet); }
    _ => {}
  }

  assert_eq!(
    style_tree(&node_1, &stylesheet),
    StyledNode::new(
      &node_1,
      values_1,
      vec![
        StyledNode::new(&node_2, values_2, vec![]),
        StyledNode::new(
          &node_3,
          values_3,
          vec![StyledNode::new(
            &node_4,
            values_4,
            vec![StyledNode::new(
              &dom::Node::text("Hello World!".to_string()),
              hashmap![],
              vec![]
            )]
          )]
        )
      ]
    )
  );
}
