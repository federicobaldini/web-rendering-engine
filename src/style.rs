use std::collections::HashMap;
use std::collections::HashSet;

use crate::css;
use crate::dom;

// Map from CSS property names to values
type PropertyMap = HashMap<String, css::Value>;
// A single CSS rule and the specificity of its most specific matching selector
type MatchedRule<'a> = (css::Specificity, &'a css::Rule);

// A node with associated style data.
pub struct StyledNode<'a> {
  node: &'a dom::Node, // pointer to a DOM node
  specified_values: PropertyMap,
  children: Vec<StyledNode<'a>>,
}

fn matches_simple_selector(element: &dom::ElementData, selector: &css::SimpleSelector) -> bool {
  // Check type selector
  if selector
    .tag_name()
    .iter()
    .any(|name| *element.tag_name() != *name)
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
    .find(|selector| matches(element, *selector))
    .map(|selector| (selector.specificity(), rule))
}
