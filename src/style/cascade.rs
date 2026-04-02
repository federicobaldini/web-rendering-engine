use std::collections::HashMap;

use crate::css;
use crate::dom;
use crate::hashmap;
use super::matching::matching_rules;
use super::tree::PropertyMap;

// Apply styles to a single element, returning the specified values
pub fn specified_values(element: &dom::ElementData, stylesheet: &css::Stylesheet) -> PropertyMap {
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
