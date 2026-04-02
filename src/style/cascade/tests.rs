use crate::css;
use crate::dom;
use crate::hashmap;
use crate::style::PropertyMap;
use super::specified_values;

// Test the function specified_values
#[test]
fn test_specified_values() {
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
  let stylesheet: css::Stylesheet = css::Stylesheet::new(vec![rule.clone()]);
  let values: PropertyMap = specified_values(&element, &stylesheet);

  assert_eq!(
    values.get("width"),
    Some(&css::Value::Length(100.0, css::Unit::Px))
  );
}
