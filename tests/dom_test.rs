#[path = "../src/dom.rs"]
mod dom;
use dom::*;

// test the associated function text() of the Node struct implementation
#[test]
fn test_node_text() {
  let node: Node = Node::text("Hello World!".to_string());

  assert_eq!(node.children(), Vec::new());
  match node.node_type() {
    NodeType::Text(data) => assert_eq!(data, "Hello World!".to_string()),
    _ => panic!("Unexpected node type"),
  }
}

// test the associated function element() of the Node struct implementation
#[test]
fn test_node_element_() {
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
