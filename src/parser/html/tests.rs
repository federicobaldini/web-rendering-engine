use crate::dom;
use crate::hashmap;
use super::*;

// Test the method parse_tag_name of the HTMLParser struct implementation
#[test]
fn test_parse_tag_name() {
  let mut html_parser: HTMLParser = HTMLParser::new(1, "<p>Hello World!</p>".to_string());

  // Assert that the parse_tag_name method correctly parses the tag name "p"
  assert_eq!(html_parser.parse_tag_name(), "p");
}

// Test the method parse_comment_content of the HTMLParser struct implementation
#[test]
fn test_parse_comment_content() {
  let mut html_parser: HTMLParser = HTMLParser::new(1, " Hello World! -->".to_string());

  // Assert that the parse_comment_content method correctly parses the tag name "p"
  assert_eq!(html_parser.parse_comment_content(), "Hello World! ");
}

// Test the method parse_text of the HTMLParser struct implementation
#[test]
fn test_parse_text() {
  let mut html_parser: HTMLParser = HTMLParser::new(3, "<p>Hello World!</p>".to_string());
  let node: dom::Node = dom::Node::text("Hello World!".to_string());

  // Assert that the parse_tag_name method correctly parses the text "Hello World!" inside the tag "p"
  assert_eq!(html_parser.parse_text(), node);
}

// Test the method parse_element of the HTMLParser struct implementation
#[test]
fn test_parse_element() {
  let mut html_parser: HTMLParser =
    HTMLParser::new(0, "<p class='paragraph'\n>Hello World!</p\n>".to_string());
  let tag_name_1: String = String::from("p");
  let attributes_1: dom::AttributeMap =
    hashmap![String::from("class") => String::from("paragraph")];
  let children_1: Vec<dom::Node> = vec![dom::Node::text("Hello World!".to_string())];
  let node_1: dom::Node = dom::Node::element(tag_name_1, attributes_1, children_1);

  // Assert that the parse_element method correctly parses the element "<p class='paragraph'\n>Hello World!</p\n>"
  assert_eq!(html_parser.parse_element().unwrap(), node_1);

  html_parser = HTMLParser::new(0, "<div class='container' />".to_string());
  let tag_name_2: String = String::from("div");
  let attributes_2: dom::AttributeMap =
    hashmap![String::from("class") => String::from("container")];
  let node_2: dom::Node = dom::Node::element(tag_name_2, attributes_2, vec![]);

  // Assert that the parse_element method correctly parses the element "<div class='container' />"
  assert_eq!(html_parser.parse_element().unwrap(), node_2);

  html_parser = HTMLParser::new(0, "<link href='https://www.test.com' >".to_string());
  let tag_name_3: String = String::from("link");
  let attributes_3: dom::AttributeMap =
    hashmap![String::from("href") => String::from("https://www.test.com")];
  let node_3: dom::Node = dom::Node::element(tag_name_3, attributes_3, vec![]);

  // Assert that the parse_element method correctly parses the element "<link href='https://www.test.com' >"
  assert_eq!(html_parser.parse_element().unwrap(), node_3);
}

// Test the method parse_comment of the HTMLParser struct implementation
#[test]
fn test_parse_comment() {
  let mut html_parser: HTMLParser = HTMLParser::new(0, "<!-- Hello World! -->".to_string());
  let node: dom::Node = dom::Node::comment(" Hello World! ".to_string());

  // Assert that the parse_comment method correctly parses the element "<!-- Hello World! -->"
  assert_eq!(html_parser.parse_comment().unwrap(), node);
}

// Test the method parse_node of the HTMLParser struct implementation
#[test]
fn test_parse_node() {
  let mut html_parser: HTMLParser = HTMLParser::new(0, "Hello World!".to_string());
  let node: dom::Node = dom::Node::text("Hello World!".to_string());

  // Assert that the parse_node method correctly parses the text "Hello World!"
  assert_eq!(html_parser.parse_node().unwrap(), node);

  let mut html_parser: HTMLParser =
    HTMLParser::new(0, "<p class='paragraph'>Hello World!</p>".to_string());
  let tag_name: String = String::from("p");
  let attributes: dom::AttributeMap =
    hashmap![String::from("class") => String::from("paragraph")];
  let children: Vec<dom::Node> = vec![dom::Node::text("Hello World!".to_string())];
  let node: dom::Node = dom::Node::element(tag_name, attributes, children);

  // Assert that the parse_element method correctly parses the element "<p class='paragraph'>Hello World!</p>"
  assert_eq!(html_parser.parse_node().unwrap(), node);
}

// Test the method parse_attribute of the HTMLParser struct implementation
#[test]
fn test_parse_attribute() {
  let mut html_parser: HTMLParser =
    HTMLParser::new(3, "<p class='paragraph'>Hello World!</p>".to_string());

  // Assert that the parse_attribute method correctly parses the attribute "class='paragraph'"
  assert_eq!(
    html_parser.parse_attribute().unwrap(),
    ("class".to_string(), "paragraph".to_string())
  );
}

// Test the method parse_attribute_value of the HTMLParser struct implementation
#[test]
fn test_parse_attribute_value() {
  let mut html_parser: HTMLParser =
    HTMLParser::new(9, "<p class='paragraph'>Hello World!</p>".to_string());

  // Assert that the parse_attribute_value method correctly parses the attribute value "paragraph"
  assert_eq!(
    html_parser.parse_attribute_value().unwrap(),
    "paragraph".to_string()
  );
}

// Test the method parse_attributes of the HTMLParser struct implementation
#[test]
fn test_parse_attributes() {
  let mut html_parser: HTMLParser = HTMLParser::new(
    3,
    "<p class='paragraph' style='color:red;'>Hello World!</p>".to_string(),
  );
  let attributes: dom::AttributeMap = hashmap![String::from("class") => String::from("paragraph"), String::from("style") => String::from("color:red;")];

  // Assert that the parse_attributes method correctly parses the attributes "class='paragraph' style='color:red;'"
  assert_eq!(html_parser.parse_attributes().unwrap(), attributes);
}

// Test the method parse_nodes of the HTMLParser struct implementation
#[test]
fn test_parse_nodes() {
  let mut html_parser: HTMLParser = HTMLParser::new(
    0,
    "<div class='container-1'></div><div class='container-2'><p class='paragraph'>Hello World!</p></div>".to_string(),
  );
  // Node 1: <div class='container-1'>
  let tag_name_1: String = String::from("div");
  let attributes_1: dom::AttributeMap =
    hashmap![String::from("class") => String::from("container-1")];
  let children_1: Vec<dom::Node> = vec![];
  let node_1: dom::Node = dom::Node::element(tag_name_1, attributes_1, children_1);
  // Node 3: <p class='paragraph'>
  let tag_name_3: String = String::from("p");
  let attributes_3: dom::AttributeMap =
    hashmap![String::from("class") => String::from("paragraph")];
  let children_3: Vec<dom::Node> = vec![dom::Node::text("Hello World!".to_string())];
  let node_3: dom::Node = dom::Node::element(tag_name_3, attributes_3, children_3);
  // Node 2: <div class='container-2'>
  let tag_name_2: String = String::from("div");
  let attributes_2: dom::AttributeMap =
    hashmap![String::from("class") => String::from("container-2")];
  let children_2: Vec<dom::Node> = vec![node_3];
  let node_2: dom::Node = dom::Node::element(tag_name_2, attributes_2, children_2);

  // Assert that the parse_nodes method correctly parses the nested and sibling nodes: node_1, node_2.node_3
  assert_eq!(html_parser.parse_nodes().unwrap(), vec![node_1, node_2]);
}

// Test recovery: boolean attribute without '=value'
#[test]
fn test_recovery_boolean_attribute() {
  let mut html_parser: HTMLParser = HTMLParser::new(0, "<div disabled>text</div>".to_string());
  let attributes: dom::AttributeMap = hashmap![String::from("disabled") => String::from("")];
  let children: Vec<dom::Node> = vec![dom::Node::text("text".to_string())];
  let expected: dom::Node = dom::Node::element("div".to_string(), attributes, children);

  // Assert that a boolean attribute (no '=value') is recovered as an empty string value
  assert_eq!(html_parser.parse_element().unwrap(), expected);
}

// Test recovery: attribute value without quotes
#[test]
fn test_recovery_unquoted_attribute() {
  let mut html_parser: HTMLParser =
    HTMLParser::new(0, "<div class=myclass>text</div>".to_string());
  let attributes: dom::AttributeMap =
    hashmap![String::from("class") => String::from("myclass")];
  let children: Vec<dom::Node> = vec![dom::Node::text("text".to_string())];
  let expected: dom::Node = dom::Node::element("div".to_string(), attributes, children);

  // Assert that an unquoted attribute value is recovered by reading until whitespace or '>'
  assert_eq!(html_parser.parse_element().unwrap(), expected);
}

// Test recovery: element not closed before EOF
#[test]
fn test_recovery_unclosed_element() {
  let mut html_parser: HTMLParser =
    HTMLParser::new(0, "<div>unclosed text".to_string());
  let children: Vec<dom::Node> = vec![dom::Node::text("unclosed text".to_string())];
  let expected: dom::Node = dom::Node::element("div".to_string(), hashmap![], children);

  // Assert that an element without a closing tag is auto-closed at EOF
  assert_eq!(html_parser.parse_element().unwrap(), expected);
}

// Test recovery: closing tag that doesn't match the opening tag
#[test]
fn test_recovery_mismatched_closing_tag() {
  let mut html_parser: HTMLParser =
    HTMLParser::new(0, "<div>text</p>".to_string());
  let children: Vec<dom::Node> = vec![dom::Node::text("text".to_string())];
  let expected: dom::Node = dom::Node::element("div".to_string(), hashmap![], children);

  // Assert that a mismatched closing tag is skipped and the element is closed anyway
  assert_eq!(html_parser.parse_element().unwrap(), expected);
}

// Test the function parse of the HTMLParser struct implementation
#[test]
fn test_parse() {
  // Node 2: <div class='container-1'>
  let tag_name_2: String = String::from("div");
  let attributes_2: dom::AttributeMap =
    hashmap![String::from("class") => String::from("container-1")];
  let children_2: Vec<dom::Node> = vec![];
  let node_2: dom::Node = dom::Node::element(tag_name_2, attributes_2, children_2);
  // Node 4: <p class='paragraph'>
  let tag_name_4: String = String::from("p");
  let attributes_4: dom::AttributeMap =
    hashmap![String::from("class") => String::from("paragraph")];
  let children_4: Vec<dom::Node> = vec![dom::Node::text("Hello World!".to_string())];
  let node_4: dom::Node = dom::Node::element(tag_name_4, attributes_4, children_4);
  // Node 3: <div class='container-2'>
  let tag_name_3: String = String::from("div");
  let attributes_3: dom::AttributeMap =
    hashmap![String::from("class") => String::from("container-2")];
  let children_3: Vec<dom::Node> = vec![node_4];
  let node_3: dom::Node = dom::Node::element(tag_name_3, attributes_3, children_3);
  // Node 1: <html>
  let tag_name_1: String = String::from("html");
  let attributes_1: dom::AttributeMap = hashmap![];
  let children_1: Vec<dom::Node> = vec![node_2, node_3.clone()];
  let node_1: dom::Node = dom::Node::element(tag_name_1, attributes_1, children_1);

  // Assert that the parse function correctly parses the nodes without a root element and then add the "html" tag as root element, returning it
  assert_eq!(HTMLParser::parse("<div class='container-1'></div><div class='container-2'><p class='paragraph'>Hello World!</p></div>".to_string()).unwrap(), node_1);
  // Assert that the parse function correctly parses the nodes with a root element and then returning it
  assert_eq!(
    HTMLParser::parse(
      "<div class='container-2'><p class='paragraph'>Hello World!</p></div>".to_string()
    )
    .unwrap(),
    node_3
  );
}
