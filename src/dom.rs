#![allow(dead_code)]

use std::collections::HashMap;

pub type AttributesMap = HashMap<String, String>;

#[derive(Clone, Debug)]
pub struct ElementData {
  tag_name: String,
  attributes: AttributesMap,
}

impl PartialEq for ElementData {
  fn eq(&self, other: &Self) -> bool {
    *self.tag_name == *other.tag_name && self.attributes == other.attributes
  }
}

impl ElementData {
  pub fn new(tag_name: String, attributes: AttributesMap) -> ElementData {
    ElementData {
      tag_name,
      attributes,
    }
  }
  pub fn tag_name(&self) -> String {
    self.tag_name.clone()
  }
  pub fn attributes(&self) -> AttributesMap {
    self.attributes.clone()
  }
}

#[derive(Clone, PartialEq, Debug)]
pub enum NodeType {
  Text(String),
  Element(ElementData),
}

#[derive(Clone, Debug)]
pub struct Node {
  // data common to all nodes:
  children: Vec<Node>,
  // data specific to each node type:
  node_type: NodeType,
}

impl Node {
  pub fn children(&self) -> Vec<Node> {
    self.children.clone()
  }
  pub fn node_type(&self) -> NodeType {
    self.node_type.clone()
  }
}

impl PartialEq for Node {
  fn eq(&self, other: &Self) -> bool {
    *self.children == *other.children && self.node_type == other.node_type
  }
}

impl Node {
  pub fn text(data: String) -> Node {
    Node {
      children: Vec::new(),
      node_type: NodeType::Text(data),
    }
  }

  pub fn element(tag_name: String, attributes: AttributesMap, children: Vec<Node>) -> Node {
    Node {
      children,
      node_type: NodeType::Element(ElementData {
        tag_name,
        attributes,
      }),
    }
  }
}

#[macro_export]
macro_rules! hashmap {
  ($( $key: expr => $val: expr ),*) => {{
       let mut map = ::std::collections::HashMap::new();
       $( map.insert($key, $val); )*
       map
  }}
}
