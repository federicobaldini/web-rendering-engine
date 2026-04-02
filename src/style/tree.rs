use std::collections::HashMap;

use crate::css;
use crate::dom;
use crate::hashmap;
use super::cascade::specified_values;

// Map from CSS property names to values
pub type PropertyMap = HashMap<String, css::Value>;

pub enum Display {
  Inline,
  Block,
  InlineBlock,
  None,
}

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

  // Return the specified value of a property if it exists, otherwise "None"
  pub fn value(&self, name: &str) -> Option<css::Value> {
    self
      .specified_values
      .get(name)
      .map(|v: &css::Value| v.clone())
  }

  // Return the specified value of property "name", or property "fallback_name" if that doesn't
  // exist, or value "default" if neither does.
  pub fn lookup(&self, name: &str, fallback_name: &str, default: &css::Value) -> css::Value {
    self
      .value(name)
      .unwrap_or_else(|| self.value(fallback_name).unwrap_or_else(|| default.clone()))
  }

  // The value of the "display" property (defaults to inline).
  pub fn display(&self) -> Display {
    match self.value("display") {
      Some(css::Value::Keyword(s)) => match &*s {
        "block" => Display::Block,
        "inline-block" => Display::InlineBlock,
        "none" => Display::None,
        _ => Display::Inline,
      },
      _ => Display::Inline,
    }
  }

  pub fn specified_values_to_string(&self) -> String {
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
      dom::NodeType::Comment(ref comment) => {
        println!("{:spaces$}<!--{}-->", "", comment, spaces = indent);
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

// Apply a stylesheet to an entire DOM tree, returning a StyledNode tree
pub fn style_tree<'a>(root: &'a dom::Node, stylesheet: &'a css::Stylesheet) -> StyledNode<'a> {
  StyledNode::new(
    root,
    match root.node_type() {
      dom::NodeType::Element(ref elem) => specified_values(elem, stylesheet),
      dom::NodeType::Text(_) => hashmap![],
      dom::NodeType::Comment(_) => hashmap![],
    },
    root
      .children()
      .iter()
      .map(|child: &dom::Node| style_tree(child, stylesheet))
      .collect(),
  )
}
