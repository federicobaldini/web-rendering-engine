use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

use crate::css;
use crate::dom;
use crate::hashmap;

// Map from CSS property names to values
type PropertyMap = HashMap<String, css::Value>;
// A single CSS rule and the specificity of its most specific matching selector
type MatchedRule<'a> = (css::Specificity, &'a css::Rule);

// A node with associated style data.
#[derive(Clone, Debug)]
pub struct StyledNode<'a> {
  node: &'a dom::Node, // pointer to a DOM node
  specified_values: PropertyMap,
  children: Vec<StyledNode<'a>>,
}

impl<'a> PartialEq for StyledNode<'a> {
  fn eq(&self, other: &Self) -> bool {
    self.node == other.node
      && self.specified_values == other.specified_values
      && self.children == other.children
  }
}

impl<'a> StyledNode<'a> {
  pub fn new(
    node: &'a dom::Node,
    specified_values: PropertyMap,
    children: Vec<StyledNode<'a>>,
  ) -> Self {
    Self {
      node,
      specified_values,
      children,
    }
  }

  pub fn node(&self) -> &'a dom::Node {
    self.node
  }

  pub fn specified_values(&self) -> &PropertyMap {
    &self.specified_values
  }

  pub fn children(&self) -> &Vec<StyledNode<'a>> {
    &self.children
  }

  fn specified_values_to_string(&self) -> String {
    let mut result = String::new();
    for (key, value) in self.specified_values.iter() {
      result.push_str(&format!("{}:{};", key, value));
    }
    result
  }

  pub fn print_style_node_tree(style_node: &'a StyledNode, indent: usize) {
    match style_node.node().node_type() {
      dom::NodeType::Text(ref text) => {
        println!("{:spaces$}{}", "", text, spaces = indent);
      }
      dom::NodeType::Element(ref element) => {
        if *style_node.specified_values() != hashmap![] {
          println!(
            "{:spaces$}<{} style={:?}>",
            "",
            element.tag_name(),
            style_node.specified_values_to_string(),
            spaces = indent
          );
        } else {
          println!("{:spaces$}<{}>", "", element.tag_name(), spaces = indent);
        }
        for child in style_node.children() {
          StyledNode::print_style_node_tree(child, indent + 2);
        }
        println!("{:spaces$}</{}>", "", element.tag_name(), spaces = indent);
      }
    }
  }
}

fn matches_simple_selector(element: &dom::ElementData, selector: &css::SimpleSelector) -> bool {
  // Check type selector
  if selector
    .tag_name()
    .iter()
    .any(|name: &String| *element.tag_name() != *name)
  {
    return false;
  }

  // Check ID selector
  if selector
    .id()
    .iter()
    .any(|id: &String| element.id() != Some(id))
  {
    return false;
  }

  // Check class selectors
  let element_classes: HashSet<&str> = element.classes();
  if selector
    .classes()
    .iter()
    .any(|class: &String| !element_classes.contains(&**class))
  {
    return false;
  }

  // We didn't find any non-matching selector components
  return true;
}

// Selector matching:
fn matches(element: &dom::ElementData, selector: &css::Selector) -> bool {
  match *selector {
    css::Selector::Simple(ref simple_selector) => matches_simple_selector(element, simple_selector),
  }
}

// If 'rule' matches 'element', return a 'MatchedRule'. Otherwise return 'None'
fn match_rule<'a>(element: &dom::ElementData, rule: &'a css::Rule) -> Option<MatchedRule<'a>> {
  // Find the first (highest-specificity) matching selector
  rule
    .selectors()
    .iter()
    .find(|selector: &&Selector| matches(element, *selector))
    .map(|selector: &Selector| (selector.specificity(), rule))
}

// Find all CSS rules that match the given element
fn matching_rules<'a>(
  element: &dom::ElementData,
  stylesheet: &'a css::Stylesheet,
) -> Vec<MatchedRule<'a>> {
  // For now, we just do a linear scan of all the rules.  For large
  // documents, it would be more efficient to store the rules in hash tables
  // based on tag name, id, class, etc.
  stylesheet
    .rules()
    .iter()
    .filter_map(|rule: &css::Rule| match_rule(element, rule))
    .collect()
}

// Apply styles to a single element, returning the specified values
fn specified_values(element: &dom::ElementData, stylesheet: &css::Stylesheet) -> PropertyMap {
  let mut values: HashMap<String, css::Value> = hashmap![];
  let mut rules: Vec<((usize, usize, usize), &css::Rule)> = matching_rules(element, stylesheet);

  // Go through the rules from lowest to highest specificity
  rules.sort_by(|&(a, _), &(b, _)| a.cmp(&b));
  for (_, rule) in rules {
    for declaration in rule.declarations() {
      values.insert(declaration.name().to_string(), declaration.value().clone());
    }
  }
  return values;
}

// Apply a stylesheet to an entire DOM tree, returning a StyledNode tree
pub fn style_tree<'a>(root: &'a dom::Node, stylesheet: &'a css::Stylesheet) -> StyledNode<'a> {
  StyledNode::new(
    root,
    match root.node_type() {
      dom::NodeType::Element(ref elem) => specified_values(elem, stylesheet),
      dom::NodeType::Text(_) => hashmap![],
    },
    root
      .children()
      .iter()
      .map(|child: &dom::Node| style_tree(child, stylesheet))
      .collect(),
  )
}

#[cfg(test)]
mod tests {
  use crate::hashmap;
  use crate::style::*;

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
}
