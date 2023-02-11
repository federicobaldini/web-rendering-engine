use std::collections::HashMap;

use crate::css;
use crate::dom;

// Map from CSS property names to values.
type PropertyMap = HashMap<String, css::Value>;
// A single CSS rule and the specificity of its most specific matching selector.
type MatchedRule<'a> = (css::Specificity, &'a css::Rule);

// A node with associated style data.
pub struct StyledNode<'a> {
  node: &'a dom::Node, // pointer to a DOM node
  specified_values: PropertyMap,
  children: Vec<StyledNode<'a>>,
}
