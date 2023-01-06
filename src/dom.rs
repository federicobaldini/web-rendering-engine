use std::collections::HashMap;

type AttributesMap = HashMap<String, String>;

struct ElementData {
  tag_name: String,
  attributes: AttributesMap,
}

enum NodeType {
  Text(String),
  Element(ElementData),
}

struct Node {
  // data common to all nodes:
  children: Vec<Node>,
  // data specific to each node type:
  node_type: NodeType,
}

impl Node {
  pub fn text(data: String) -> Node {
    Node {
      children: Vec::new(),
      node_type: NodeType::Text(data),
    }
  }

  pub fn element(name: String, attributes: AttributesMap, children: Vec<Node>) -> Node {
    Node {
      children,
      node_type: NodeType::Element(ElementData {
        tag_name: name,
        attributes,
      }),
    }
  }
}
