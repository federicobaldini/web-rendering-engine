/**
 * Features to add:
 * - Extend NodeType to include additional types like comment nodes;
 */
use std::{
  collections::{HashMap, HashSet},
  fmt::{self, Formatter, Result},
};

pub type AttributeMap = HashMap<String, String>;

#[derive(Clone, Debug)]
pub struct ElementData {
  tag_name: String,
  attributes: AttributeMap,
}

impl PartialEq for ElementData {
  fn eq(&self, other: &Self) -> bool {
    *self.tag_name == *other.tag_name && self.attributes == other.attributes
  }
}

impl fmt::Display for ElementData {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "<{}", self.tag_name)?;
    for (key, value) in &self.attributes {
      write!(f, " {}='{}'", key, value)?;
    }
    write!(f, ">")
  }
}

impl ElementData {
  pub fn new(tag_name: String, attributes: AttributeMap) -> Self {
    Self {
      tag_name,
      attributes,
    }
  }

  pub fn tag_name(&self) -> &String {
    &self.tag_name
  }

  pub fn attributes(&self) -> &AttributeMap {
    &self.attributes
  }
  }
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
  pub fn children(&self) -> &Vec<Node> {
    &self.children
  }

  pub fn node_type(&self) -> &NodeType {
    &self.node_type
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

  pub fn element(tag_name: String, attributes: AttributeMap, children: Vec<Node>) -> Node {
    Node {
      children,
      node_type: NodeType::Element(ElementData {
        tag_name,
        attributes,
      }),
    }
  }

  pub fn print_node_tree(node: Node, indent: usize) {
    match node.node_type {
      NodeType::Text(text) => {
        // Indentation of `indent` spaces before the next argument
        println!("{:spaces$}{}", "", text, spaces = indent);
      }
      NodeType::Element(element) => {
        if node.children.len() > 0 {
          println!("{:spaces$}{}", "", element, spaces = indent);
          for child in node.children {
            Node::print_node_tree(child, indent + 2);
          }
          println!("{:spaces$}</{}>", "", element.tag_name(), spaces = indent);
        } else {
          print!("{:spaces$}{}", "", element, spaces = indent);
          println!("</{}>", element.tag_name());
        }
      }
    }
  }
}

#[macro_export]
macro_rules! hashmap {
  ($( $key: expr => $val: expr ),*) => {{
       let mut _map = ::std::collections::HashMap::new();
       $( _map.insert($key, $val); )*
       _map
  }}
}

#[cfg(test)]
mod tests {
  use crate::dom::*;

  // Test the function text of the Node struct implementation
  #[test]
  fn test_text() {
    let node: Node = Node::text("Hello World!".to_string());

    // Assert that the node_type method correctly returns the text "Hello World!"
    assert_eq!(
      *node.node_type(),
      NodeType::Text("Hello World!".to_string())
    );
    // Assert that the children method correctly returns no elements
    assert_eq!(*node.children(), Vec::new());
  }

  // Test the function element of the Node struct implementation
  #[test]
  fn test_element() {
    // Node 2: <span class='text'>
    let tag_name_2: String = String::from("span");
    let attributes_2: AttributeMap = hashmap![String::from("class") => String::from("text")];
    let children_2: Vec<Node> = vec![];
    let node_2: Node = Node::element(tag_name_2, attributes_2, children_2);
    // Node 1: <p class='paragraph'>
    let tag_name_1: String = String::from("p");
    let attributes_1: AttributeMap = hashmap![String::from("class") => String::from("paragraph")];
    let children_1: Vec<Node> = vec![node_2];
    let node_1: Node = Node::element(tag_name_1.clone(), attributes_1.clone(), children_1.clone());

    // Assert that the node_type method correctly returns the NodeType (Element) with the correct ElementData (tag name and attributes).
    assert_eq!(
      *node_1.node_type(),
      NodeType::Element(ElementData::new(tag_name_1, attributes_1))
    );
    // Assert that the children method correctly returns the node_1 children
    assert_eq!(*node_1.children(), children_1);
  }
}
