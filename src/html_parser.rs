/**
 * Features to add:
 * - Manage HTML comments '<!-- Hi! -->';
 * - Create an invalid HTML file that causes the parser fail. Modify the parser to recover
 *   from the error and produce a DOM tree for the test file;
 */
use crate::dom;
use crate::hashmap;
use crate::text_parser::TextParser;

pub struct HTMLParser {
  text_parser: TextParser,
}

impl HTMLParser {
  pub fn new(position: usize, input: String) -> Self {
    Self {
      text_parser: TextParser::new(position, input),
    }
  }

  // Parse a tag or attribute name
  fn parse_tag_name(&mut self) -> String {
    self.text_parser.consume_while(|c: char| match c {
      'a'..='z' | 'A'..='Z' | '0'..='9' => true,
      _ => false,
    })
  }

  // Parse a text node
  fn parse_text(&mut self) -> dom::Node {
    dom::Node::text(self.text_parser.consume_while(|c: char| c != '<'))
  }

  // Parse a single element, including its open tag, contents, and closing tag
  fn parse_element(&mut self) -> dom::Node {
    // Opening tag
    assert!(self.text_parser.consume_char() == '<');
    let tag_name: String = self.parse_tag_name();
    let attributes: dom::AttributeMap = self.parse_attributes();
    assert!(self.text_parser.consume_char() == '>');

    // Contents
    let children: Vec<dom::Node> = self.parse_nodes();

    // Closing tag
    assert!(self.text_parser.consume_char() == '<');
    assert!(self.text_parser.consume_char() == '/');
    assert!(self.parse_tag_name() == tag_name);
    assert!(self.text_parser.consume_char() == '>');

    return dom::Node::element(tag_name, attributes, children);
  }

  // Parse a single node
  fn parse_node(&mut self) -> dom::Node {
    match self.text_parser.next_char() {
      '<' => self.parse_element(),
      _ => self.parse_text(),
    }
  }

  // Parse a single name="value" pair
  fn parse_attribute(&mut self) -> (String, String) {
    let name: String = self.parse_tag_name();
    assert!(self.text_parser.consume_char() == '=');
    let value: String = self.parse_attribute_value();
    return (name, value);
  }

  // Parse a quoted value
  fn parse_attribute_value(&mut self) -> String {
    let open_quote: char = self.text_parser.consume_char();
    assert!(open_quote == '"' || open_quote == '\'');
    let value: String = self.text_parser.consume_while(|c: char| c != open_quote);
    assert!(self.text_parser.consume_char() == open_quote);
    return value;
  }

  // Parse a list of name="value" pairs, separated by whitespace
  fn parse_attributes(&mut self) -> dom::AttributeMap {
    let mut attributes: dom::AttributeMap = hashmap![];
    loop {
      self.text_parser.consume_whitespace();
      if self.text_parser.next_char() == '>' {
        break;
      }
      let (name, value): (String, String) = self.parse_attribute();
      attributes.insert(name, value);
    }
    return attributes;
  }

  // Parse a sequence of sibling nodes
  fn parse_nodes(&mut self) -> Vec<dom::Node> {
    let mut nodes: Vec<dom::Node> = Vec::new();
    loop {
      self.text_parser.consume_whitespace();
      if self.text_parser.eof() || self.text_parser.starts_with("</") {
        break;
      }
      nodes.push(self.parse_node());
    }
    return nodes;
  }

  // Parse an HTML document and return the root element
  pub fn parse(source: String) -> dom::Node {
    let mut nodes: Vec<dom::Node> = HTMLParser::new(0, source).parse_nodes();

    // If the document contains a root element, just return it. Otherwise, create one
    if nodes.len() == 1 {
      nodes.swap_remove(0)
    } else {
      dom::Node::element("html".to_string(), hashmap![], nodes)
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::hashmap;
  use crate::html_parser::*;

  // Test the method parse_tag_name of the HTMLParser struct implementation
  #[test]
  fn test_parse_tag_name() {
    let mut html_parser: HTMLParser = HTMLParser::new(1, "<p>Hello World!</p>".to_string());

    // Assert that the parse_tag_name method correctly parses the tag name "p"
    assert_eq!(html_parser.parse_tag_name(), "p");
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
      HTMLParser::new(0, "<p class='paragraph'>Hello World!</p>".to_string());
    let tag_name: String = String::from("p");
    let attributes: dom::AttributeMap =
      hashmap![String::from("class") => String::from("paragraph")];
    let children: Vec<dom::Node> = vec![dom::Node::text("Hello World!".to_string())];
    let node: dom::Node = dom::Node::element(tag_name, attributes, children);

    // Assert that the parse_element method correctly parses the element "<p class='paragraph'>Hello World!</p>"
    assert_eq!(html_parser.parse_element(), node);
  }

  // Test the method parse_node of the HTMLParser struct implementation
  #[test]
  fn test_parse_node() {
    let mut html_parser: HTMLParser = HTMLParser::new(0, "Hello World!".to_string());
    let node: dom::Node = dom::Node::text("Hello World!".to_string());

    // Assert that the parse_node method correctly parses the text "Hello World!"
    assert_eq!(html_parser.parse_node(), node);

    let mut html_parser: HTMLParser =
      HTMLParser::new(0, "<p class='paragraph'>Hello World!</p>".to_string());
    let tag_name: String = String::from("p");
    let attributes: dom::AttributeMap =
      hashmap![String::from("class") => String::from("paragraph")];
    let children: Vec<dom::Node> = vec![dom::Node::text("Hello World!".to_string())];
    let node: dom::Node = dom::Node::element(tag_name, attributes, children);

    // Assert that the parse_element method correctly parses the element "<p class='paragraph'>Hello World!</p>"
    assert_eq!(html_parser.parse_node(), node);
  }

  // Test the method parse_attribute of the HTMLParser struct implementation
  #[test]
  fn test_parse_attribute() {
    let mut html_parser: HTMLParser =
      HTMLParser::new(3, "<p class='paragraph'>Hello World!</p>".to_string());

    // Assert that the parse_attribute method correctly parses the attribute "class='paragraph'"
    assert_eq!(
      html_parser.parse_attribute(),
      ("class".to_string(), "paragraph".to_string())
    );
  }

  // Test the method parse_attribute_value of the HTMLParser struct implementation
  #[test]
  fn test_parse_attribute_value() {
    let mut html_parser: HTMLParser =
      HTMLParser::new(9, "<p class='paragraph'>Hello World!</p>".to_string());

    // Assert that the parse_attribute_value method correctly parses the attribute value "paragraph"
    assert_eq!(html_parser.parse_attribute_value(), "paragraph".to_string());
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
    assert_eq!(html_parser.parse_attributes(), attributes);
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
    assert_eq!(html_parser.parse_nodes(), vec![node_1, node_2]);
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
    assert_eq!(HTMLParser::parse("<div class='container-1'></div><div class='container-2'><p class='paragraph'>Hello World!</p></div>".to_string()), node_1);
    // Assert that the parse function correctly parses the nodes with a root element and then returning it
    assert_eq!(
      HTMLParser::parse(
        "<div class='container-2'><p class='paragraph'>Hello World!</p></div>".to_string()
      ),
      node_3
    );
  }
}
