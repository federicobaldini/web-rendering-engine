use crate::css;
use crate::dom;
use crate::hashmap;
use crate::style::*;
use super::matching::{match_rule, matching_rules, matches_simple_selector};

// Test the function matches_simple_selector
#[test]
fn test_matches_simple_selector() {
  let tag_name: String = String::from("div");
  let attributes: dom::AttributeMap = hashmap![String::from("id") => String::from("container-id"), String::from("class") => String::from("container-class")];
  let element: dom::ElementData = dom::ElementData::new(tag_name, attributes);
  let simple_selector_1: css::SimpleSelector = css::SimpleSelector::new(
    Some("div".to_string()),
    Some("container-id".to_string()),
    vec!["container-class".to_string()],
  );
  let simple_selector_2: css::SimpleSelector = css::SimpleSelector::new(
    Some("p".to_string()),
    Some("container-id".to_string()),
    vec!["container-class".to_string()],
  );
  let simple_selector_3: css::SimpleSelector = css::SimpleSelector::new(
    Some("div".to_string()),
    Some("different-id".to_string()),
    vec!["container-class".to_string()],
  );
  let simple_selector_4: css::SimpleSelector = css::SimpleSelector::new(
    Some("div".to_string()),
    Some("container-id".to_string()),
    vec!["different-class".to_string()],
  );

  // Assert that the matches_simple_selector function correctly match the element "<div id='container-id' class='container-class'></div>"
  // with the simple selector "div#container-id.container-class"
  assert_eq!(matches_simple_selector(&element, &simple_selector_1), true);

  // Assert that the matches_simple_selector function does not match the element "<div id='container-id' class='container-class'></div>"
  // with the simple selector "p#container-id.container-class"
  assert_eq!(matches_simple_selector(&element, &simple_selector_2), false);

  // Assert that the matches_simple_selector function does not match the element "<div id='container-id' class='container-class'></div>"
  // with the simple selector "div#different-id.container-class"
  assert_eq!(matches_simple_selector(&element, &simple_selector_3), false);

  // Assert that the matches_simple_selector function does not match the element "<div id='container-id' class='container-class'></div>"
  // with the simple selector "div#container-id.different-class"
  assert_eq!(matches_simple_selector(&element, &simple_selector_4), false);
}

// Test the function match_rule
#[test]
fn test_match_rule() {
  // Element
  let tag_name: String = String::from("div");
  let attributes: dom::AttributeMap = hashmap![String::from("id") => String::from("container-id"), String::from("class") => String::from("container-class")];
  let element: dom::ElementData = dom::ElementData::new(tag_name, attributes);
  // Selector
  let simple_selector: css::SimpleSelector = css::SimpleSelector::new(
    Some("div".to_string()),
    Some("container-id".to_string()),
    vec!["container-class".to_string()],
  );
  let selector: css::Selector = css::Selector::Simple(simple_selector);
  // Declaration
  let unit: css::Value = css::Value::Length(100.0, css::Unit::Px);
  let declaration: css::Declaration = css::Declaration::new("width".to_string(), unit);
  // Rule
  let rule: css::Rule = css::Rule::new(vec![selector], vec![declaration]);
  // Specificity
  let specificity: css::Specificity = (1, 1, 1);

  // Assert that the match_rule function correctly matches the given selector and declaration with the element data
  // and returns the expected specificity and rule
  assert_eq!(match_rule(&element, &rule), Some((specificity, &rule)));
}

// Test the function matching_rules
#[test]
fn test_matching_rules() {
  // Element
  let tag_name: String = String::from("div");
  let attributes: dom::AttributeMap = hashmap![String::from("id") => String::from("container-id"), String::from("class") => String::from("container-class")];
  let element: dom::ElementData = dom::ElementData::new(tag_name, attributes);
  // Selectors
  let simple_selector_1: css::SimpleSelector = css::SimpleSelector::new(
    Some("div".to_string()),
    Some("container-id".to_string()),
    vec!["container-class".to_string()],
  );
  let simple_selector_2: css::SimpleSelector =
    css::SimpleSelector::new(None, None, vec!["container-class".to_string()]);
  let selector_1: css::Selector = css::Selector::Simple(simple_selector_1);
  let selector_2: css::Selector = css::Selector::Simple(simple_selector_2);
  // Declarations
  let unit: css::Value = css::Value::Length(100.0, css::Unit::Px);
  let declaration_1: css::Declaration = css::Declaration::new("width".to_string(), unit);
  let color: css::Value = css::Value::ColorValue(css::Color::new(163, 228, 215, 255));
  let declaration_2: css::Declaration = css::Declaration::new("background".to_string(), color);
  // Rules
  let rule_1: css::Rule = css::Rule::new(vec![selector_1], vec![declaration_1]);
  let rule_2: css::Rule = css::Rule::new(vec![selector_2], vec![declaration_2]);
  // Specificities
  let specificity_1: css::Specificity = (1, 1, 1);
  let specificity_2: css::Specificity = (0, 1, 0);
  // Stylesheet
  let stylesheet: css::Stylesheet = css::Stylesheet::new(vec![rule_1.clone(), rule_2.clone()]);

  // Assert that the matching_rules function correctly matches the given selectors and declarations with the element data
  // and returns the expected specificities and rules
  assert_eq!(
    matching_rules(&element, &stylesheet),
    vec![(specificity_1, &rule_1), (specificity_2, &rule_2)]
  );
}

// Test the function specified_values
#[test]
fn test_specified_values() {
  // Element
  let tag_name: String = String::from("div");
  let attributes: dom::AttributeMap = hashmap![String::from("id") => String::from("container-id"), String::from("class") => String::from("container-class")];
  let element: dom::ElementData = dom::ElementData::new(tag_name, attributes);
  // Selector
  let simple_selector: css::SimpleSelector = css::SimpleSelector::new(
    Some("div".to_string()),
    Some("container-id".to_string()),
    vec!["container-class".to_string()],
  );
  let selector: css::Selector = css::Selector::Simple(simple_selector);
  // Declaration
  let unit: css::Value = css::Value::Length(100.0, css::Unit::Px);
  let declaration: css::Declaration = css::Declaration::new("width".to_string(), unit);
  // Rule
  let rule: css::Rule = css::Rule::new(vec![selector], vec![declaration]);
  // Stylesheet
  let stylesheet: css::Stylesheet = css::Stylesheet::new(vec![rule.clone()]);
  // Values
  let values: PropertyMap = specified_values(&element, &stylesheet);

  // Assert that the values returned by the specified_values function, correctly have a "width" property set to 100px
  assert_eq!(
    values.get("width"),
    Some(&css::Value::Length(100.0, css::Unit::Px))
  );
}

// Test the function style_tree
#[test]
fn test_style_tree() {
  // Node 2: <div class='container-1'>
  let tag_name_2: String = String::from("div");
  let attributes_2: dom::AttributeMap =
    hashmap![String::from("class") => String::from("container-1")];
  let children_2: Vec<dom::Node> = vec![];
  let node_2: dom::Node = dom::Node::element(tag_name_2, attributes_2, children_2);
  // Node 4: <p class='paragraph'>
  let tag_name_4: String = String::from("p");
  let attributes_4: dom::AttributeMap =
    hashmap![String::from("class") => String::from("paragraph")];
  let children_4: Vec<dom::Node> = vec![dom::Node::text("Hello World!".to_string())];
  let node_4: dom::Node = dom::Node::element(tag_name_4, attributes_4, children_4);
  // Node 3: <div class='container-2'>
  let tag_name_3: String = String::from("div");
  let attributes_3: dom::AttributeMap =
    hashmap![String::from("class") => String::from("container-2")];
  let children_3: Vec<dom::Node> = vec![node_4.clone()];
  let node_3: dom::Node = dom::Node::element(tag_name_3, attributes_3, children_3);
  // Node 1: <html>
  let tag_name_1: String = String::from("html");
  let attributes_1: dom::AttributeMap = hashmap![];
  let children_1: Vec<dom::Node> = vec![node_2.clone(), node_3.clone()];
  let node_1: dom::Node = dom::Node::element(tag_name_1, attributes_1, children_1);
  // Selectors
  let simple_selector_1: css::SimpleSelector =
    css::SimpleSelector::new(Some("p".to_string()), None, vec!["paragraph".to_string()]);
  let simple_selector_2: css::SimpleSelector =
    css::SimpleSelector::new(None, None, vec!["container-2".to_string()]);
  let selector_1: css::Selector = css::Selector::Simple(simple_selector_1);
  let selector_2: css::Selector = css::Selector::Simple(simple_selector_2);
  // Declarations
  let unit: css::Value = css::Value::Length(100.0, css::Unit::Px);
  let declaration_1: css::Declaration = css::Declaration::new("width".to_string(), unit);
  let color: css::Value = css::Value::ColorValue(css::Color::new(163, 228, 215, 255));
  let declaration_2: css::Declaration = css::Declaration::new("background".to_string(), color);
  // Rules
  let rule_1: css::Rule = css::Rule::new(vec![selector_1], vec![declaration_1]);
  let rule_2: css::Rule = css::Rule::new(vec![selector_2], vec![declaration_2]);
  // Stylesheet
  let stylesheet: css::Stylesheet = css::Stylesheet::new(vec![rule_1.clone(), rule_2.clone()]);
  // Values
  let mut values_2: PropertyMap = hashmap![];
  let mut values_4: PropertyMap = hashmap![];
  let mut values_3: PropertyMap = hashmap![];
  let mut values_1: PropertyMap = hashmap![];

  match node_2.node_type() {
    dom::NodeType::Element(element) => {
      values_2 = specified_values(&element, &stylesheet);
    }
    _ => {}
  }

  match node_4.node_type() {
    dom::NodeType::Element(element) => {
      values_4 = specified_values(&element, &stylesheet);
    }
    _ => {}
  }

  match node_3.node_type() {
    dom::NodeType::Element(element) => {
      values_3 = specified_values(&element, &stylesheet);
    }
    _ => {}
  }

  match node_1.node_type() {
    dom::NodeType::Element(element) => {
      values_1 = specified_values(&element, &stylesheet);
    }
    _ => {}
  }

  // Assert that the style_tree function correctly matches the style tree with the right style nodes
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
