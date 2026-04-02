use crate::css;
use crate::dom;
use crate::hashmap;
use super::{match_rule, matching_rules, matches_simple_selector};

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

  assert_eq!(matches_simple_selector(&element, &simple_selector_1), true);
  assert_eq!(matches_simple_selector(&element, &simple_selector_2), false);
  assert_eq!(matches_simple_selector(&element, &simple_selector_3), false);
  assert_eq!(matches_simple_selector(&element, &simple_selector_4), false);
}

// Test the function match_rule
#[test]
fn test_match_rule() {
  let tag_name: String = String::from("div");
  let attributes: dom::AttributeMap = hashmap![String::from("id") => String::from("container-id"), String::from("class") => String::from("container-class")];
  let element: dom::ElementData = dom::ElementData::new(tag_name, attributes);
  let simple_selector: css::SimpleSelector = css::SimpleSelector::new(
    Some("div".to_string()),
    Some("container-id".to_string()),
    vec!["container-class".to_string()],
  );
  let selector: css::Selector = css::Selector::Simple(simple_selector);
  let unit: css::Value = css::Value::Length(100.0, css::Unit::Px);
  let declaration: css::Declaration = css::Declaration::new("width".to_string(), unit);
  let rule: css::Rule = css::Rule::new(vec![selector], vec![declaration]);
  let specificity: css::Specificity = (1, 1, 1);

  assert_eq!(match_rule(&element, &rule), Some((specificity, &rule)));
}

// Test the function matching_rules
#[test]
fn test_matching_rules() {
  let tag_name: String = String::from("div");
  let attributes: dom::AttributeMap = hashmap![String::from("id") => String::from("container-id"), String::from("class") => String::from("container-class")];
  let element: dom::ElementData = dom::ElementData::new(tag_name, attributes);
  let simple_selector_1: css::SimpleSelector = css::SimpleSelector::new(
    Some("div".to_string()),
    Some("container-id".to_string()),
    vec!["container-class".to_string()],
  );
  let simple_selector_2: css::SimpleSelector =
    css::SimpleSelector::new(None, None, vec!["container-class".to_string()]);
  let selector_1: css::Selector = css::Selector::Simple(simple_selector_1);
  let selector_2: css::Selector = css::Selector::Simple(simple_selector_2);
  let unit: css::Value = css::Value::Length(100.0, css::Unit::Px);
  let declaration_1: css::Declaration = css::Declaration::new("width".to_string(), unit);
  let color: css::Value = css::Value::ColorValue(css::Color::new(163, 228, 215, 255));
  let declaration_2: css::Declaration = css::Declaration::new("background".to_string(), color);
  let rule_1: css::Rule = css::Rule::new(vec![selector_1], vec![declaration_1]);
  let rule_2: css::Rule = css::Rule::new(vec![selector_2], vec![declaration_2]);
  let specificity_1: css::Specificity = (1, 1, 1);
  let specificity_2: css::Specificity = (0, 1, 0);
  let stylesheet: css::Stylesheet = css::Stylesheet::new(vec![rule_1.clone(), rule_2.clone()]);

  assert_eq!(
    matching_rules(&element, &stylesheet),
    vec![(specificity_1, &rule_1), (specificity_2, &rule_2)]
  );
}
