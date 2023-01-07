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

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[cfg(test)]
mod tests {
  use crate::dom::*;

  // Test the associated function text() of the Node struct implementation.
  #[test]
  fn test_node_text() {
    let node: Node = Node::text("Hello World!".to_string());

    assert_eq!(node.children(), Vec::new());
    match node.node_type() {
      NodeType::Text(data) => assert_eq!(data, "Hello World!".to_string()),
      _ => panic!("Unexpected node type"),
    }
  }

  // Test the associated function element() of the Node struct implementation.
  #[test]
  fn test_node_element() {
    let tag_name: String = String::from("p");
    let attributes: AttributesMap = hashmap![String::from("class") => String::from("paragraph")];
    let children: Vec<Node> = vec![];
    let node: Node = Node::element(tag_name.clone(), attributes.clone(), children.clone());

    assert_eq!(
      node.node_type(),
      NodeType::Element(ElementData::new(tag_name, attributes))
    );
    assert_eq!(node.children(), children);
  }
}
