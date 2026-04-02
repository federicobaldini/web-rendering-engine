use std::collections::HashSet;

use crate::css;
use crate::dom;

// A single CSS rule and the specificity of its most specific matching selector
pub(super) type MatchedRule<'a> = (css::Specificity, &'a css::Rule);

pub(super) fn matches_simple_selector(element: &dom::ElementData, selector: &css::SimpleSelector) -> bool {
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
pub(super) fn match_rule<'a>(
  element: &dom::ElementData,
  rule: &'a css::Rule,
) -> Option<MatchedRule<'a>> {
  // Find the first (highest-specificity) matching selector
  rule
    .selectors()
    .iter()
    .find(|selector: &&css::Selector| matches(element, *selector))
    .map(|selector: &css::Selector| (selector.specificity(), rule))
}

// Find all CSS rules that match the given element
pub(super) fn matching_rules<'a>(
  element: &dom::ElementData,
  stylesheet: &'a css::Stylesheet,
) -> Vec<MatchedRule<'a>> {
  stylesheet
    .rules()
    .iter()
    .filter_map(|rule: &css::Rule| match_rule(element, rule))
    .collect()
}

#[cfg(test)]
mod tests;
