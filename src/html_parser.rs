/**
 * Features to add:
 * - Modify the parser to take a subset of HTML as input and produces a tree of DOM nodes;
 * - Manage HTML comments `<!-- Hi! -->`;
 * - Create an invalid HTML file that causes the parser fail. Modify the parser to recover
 *   from the error and produce a DOM tree for the test file;
 */
use crate::dom;
use crate::hashmap;
use std::str::CharIndices;

pub struct Parser {
  position: usize,
  input: String,
}

impl Parser {
  pub fn new(position: usize, input: String) -> Parser {
    Parser { position, input }
  }

  // Read the current character without consuming it
  fn next_char(&self) -> char {
    self.input[self.position..].chars().next().unwrap()
  }

  // Do the next characters start with the given string?
  fn starts_with(&self, s: &str) -> bool {
    self.input[self.position..].starts_with(s)
  }

  // Return true if all input is consumed
  fn eof(&self) -> bool {
    self.position >= self.input.len()
  }

  // Return the current character, and advance self.position to the next character
  fn consume_char(&mut self) -> char {
    let mut iter: CharIndices = self.input[self.position..].char_indices();
    let (_, current_char): (_, char) = iter.next().unwrap();
    let (next_position, _): (usize, _) = iter.next().unwrap_or((1, ' '));
    self.position += next_position;
    return current_char;
  }

  // Consume characters until `test` returns false
  fn consume_while<F>(&mut self, test: F) -> String
  where
    F: Fn(char) -> bool,
  {
    let mut result: String = String::new();
    while !self.eof() && test(self.next_char()) {
      result.push(self.consume_char());
    }
    return result;
  }

  // Consume and discard zero or more whitespace characters
  fn consume_whitespace(&mut self) {
    self.consume_while(char::is_whitespace);
  }

  // Parse a tag or attribute name
  fn parse_tag_name(&mut self) -> String {
    self.consume_while(|c: char| match c {
      'a'..='z' | 'A'..='Z' | '0'..='9' => true,
      _ => false,
    })
  }

  // Parse a text node
  fn parse_text(&mut self) -> dom::Node {
    dom::Node::text(self.consume_while(|c: char| c != '<'))
  }

  // Parse a single element, including its open tag, contents, and closing tag
  fn parse_element(&mut self) -> dom::Node {
    // Opening tag
    assert!(self.consume_char() == '<');
    let tag_name: String = self.parse_tag_name();
    let attributes: dom::AttributeMap = self.parse_attributes();
    assert!(self.consume_char() == '>');

    // Contents
    let children: Vec<dom::Node> = self.parse_nodes();

    // Closing tag
    assert!(self.consume_char() == '<');
    assert!(self.consume_char() == '/');
    assert!(self.parse_tag_name() == tag_name);
    assert!(self.consume_char() == '>');

    return dom::Node::element(tag_name, attributes, children);
  }

  // Parse a single node
  fn parse_node(&mut self) -> dom::Node {
    match self.next_char() {
      '<' => self.parse_element(),
      _ => self.parse_text(),
    }
  }

  // Parse a single name="value" pair
  fn parse_attribute(&mut self) -> (String, String) {
    let name: String = self.parse_tag_name();
    assert!(self.consume_char() == '=');
    let value: String = self.parse_attribute_value();
    return (name, value);
  }

  // Parse a quoted value
  fn parse_attribute_value(&mut self) -> String {
    let open_quote: char = self.consume_char();
    assert!(open_quote == '"' || open_quote == '\'');
    let value: String = self.consume_while(|c: char| c != open_quote);
    assert!(self.consume_char() == open_quote);
    return value;
  }

  // Parse a list of name="value" pairs, separated by whitespace
  fn parse_attributes(&mut self) -> dom::AttributeMap {
    let mut attributes: dom::AttributeMap = hashmap![];
    loop {
      self.consume_whitespace();
      if self.next_char() == '>' {
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
      self.consume_whitespace();
      if self.eof() || self.starts_with("</") {
        break;
      }
      nodes.push(self.parse_node());
    }
    return nodes;
  }

  // Parse an HTML document and return the root element
  pub fn parse(source: String) -> dom::Node {
    let mut nodes: Vec<dom::Node> = Parser::new(0, source).parse_nodes();

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

  // Test the method next_char() of the Parser struct implementation
  #[test]
  fn test_next_char() {
    let mut parser: Parser = Parser::new(0, "<p>Hello World!</p>".to_string());

    // Test character in position 0
    assert_eq!(parser.next_char(), '<');
    parser.position = 1;
    // Test character in position 1
    assert_eq!(parser.next_char(), 'p');
    parser.position = 18;
    // Test character in position 20
    assert_eq!(parser.next_char(), '>');
  }

  // Test the method starts_with() of the Parser struct implementation
  #[test]
  fn test_starts_with() {
    let parser: Parser = Parser::new(0, "<p>Hello World!</p>".to_string());

    // Test that input starts (or not) with specific string from position 0
    assert!(parser.starts_with("<"));
    assert!(parser.starts_with("<p>"));
    assert!(!parser.starts_with("Hello"));
    assert!(!parser.starts_with("!"));

    let parser: Parser = Parser {
      position: 3,
      input: "<p>Hello World!</p>".to_string(),
    };
    // Test that input starts (or not) with specific string from position 4
    assert!(!parser.starts_with("<"));
    assert!(!parser.starts_with("<p>"));
    assert!(parser.starts_with("H"));
    assert!(parser.starts_with("Hello"));
    assert!(!parser.starts_with("!"));
  }

  // Test the method eof() of the Parser struct implementation
  #[test]
  fn test_eof() {
    let parser: Parser = Parser::new(0, "<p>Hello World!</p>".to_string());

    // Test end of file for position 0
    assert_eq!(parser.eof(), false);

    let parser: Parser = Parser {
      position: 5,
      input: "<p>Hello World!</p>".to_string(),
    };

    // Test end of file for position 5
    assert_eq!(parser.eof(), false);

    let parser: Parser = Parser {
      position: 19,
      input: "<p>Hello World!</p>".to_string(),
    };

    // Test end of file for position 21
    assert_eq!(parser.eof(), true);
  }

  // Test the method consume_char() of the Parser struct implementation
  #[test]
  fn test_consume_char() {
    let mut parser: Parser = Parser::new(0, "<p>Hello World!</p>".to_string());

    // Test consuming character in position 0
    assert_eq!(parser.consume_char(), '<');
    assert_eq!(parser.position, 1);

    let mut parser: Parser = Parser {
      position: 3,
      input: "<p>Hello World!</p>".to_string(),
    };

    // Test consuming character in position 4
    assert_eq!(parser.consume_char(), 'H');
    assert_eq!(parser.position, 4);

    let mut parser: Parser = Parser {
      position: 8,
      input: "<p>Hello World!</p>".to_string(),
    };

    // Test consuming character in position 9
    assert_eq!(parser.consume_char(), ' ');
    assert_eq!(parser.position, 9);
  }

  // Test the method consume_while() of the Parser struct implementation
  #[test]
  fn test_consume_while() {
    let mut parser: Parser = Parser::new(3, "<p>Hello World!</p>".to_string());

    // Test consuming while character is a letter
    assert_eq!(parser.consume_while(|c| c.is_alphabetic()), "Hello");
    assert_eq!(parser.position, 8);

    // Test consuming while character is a whitespace
    assert_eq!(parser.consume_while(|c| c.is_whitespace()), " ");
    assert_eq!(parser.position, 9);

    // Test consuming while character is a digit
    assert_eq!(parser.consume_while(|c| c.is_digit(10)), "");
    assert_eq!(parser.position, 9);
  }

  // Test the method parse_tag_name() of the Parser struct implementation
  #[test]
  fn test_parse_tag_name() {
    let mut parser: Parser = Parser::new(1, "<p>Hello World!</p>".to_string());

    // Test consuming the tag name with class
    assert_eq!(parser.parse_tag_name(), "p");
    assert_eq!(parser.position, 2);

    let mut parser: Parser = Parser {
      position: 1,
      input: "<div>".to_string(),
    };

    // Test consuming the tag name
    assert_eq!(parser.parse_tag_name(), "div");
    assert_eq!(parser.position, 4);
  }

  // Test the method parse_text() of the Parser struct implementation
  #[test]
  fn test_parse_text() {
    let mut parser: Parser = Parser::new(3, "<p>Hello World!</p>".to_string());
    let node: dom::Node = dom::Node::text("Hello World!".to_string());

    // Test consuming the tag text
    assert_eq!(parser.parse_text(), node);
    assert_eq!(parser.position, 15);
  }

  // Test the method parse_element() of the Parser struct implementation
  #[test]
  fn test_parse_element() {
    let mut parser: Parser = Parser::new(0, "<p class='paragraph'>Hello World!</p>".to_string());
    let tag_name: String = String::from("p");
    let attributes: dom::AttributeMap =
      hashmap![String::from("class") => String::from("paragraph")];
    let children: Vec<dom::Node> = vec![dom::Node::text("Hello World!".to_string())];
    let node: dom::Node =
      dom::Node::element(tag_name.clone(), attributes.clone(), children.clone());

    // Test consuming the tag name, the tag class and the tag text
    assert_eq!(parser.parse_element(), node);
    assert_eq!(parser.position, 37);
  }

  // Test the method parse_node() of the Parser struct implementation
  #[test]
  fn test_parse_node() {
    let mut parser: Parser = Parser::new(0, "Hello World!".to_string());
    let node: dom::Node = dom::Node::text("Hello World!".to_string());

    // Test consuming a node with only the tag text
    assert_eq!(parser.parse_node(), node);
    assert_eq!(parser.position, 12);

    let mut parser: Parser = Parser::new(0, "<p class='paragraph'>Hello World!</p>".to_string());
    let tag_name: String = String::from("p");
    let attributes: dom::AttributeMap =
      hashmap![String::from("class") => String::from("paragraph")];
    let children: Vec<dom::Node> = vec![dom::Node::text("Hello World!".to_string())];
    let node: dom::Node =
      dom::Node::element(tag_name.clone(), attributes.clone(), children.clone());

    // Test consuming a node with tag name, the tag class and the tag text
    assert_eq!(parser.parse_node(), node);
    assert_eq!(parser.position, 37);
  }

  // Test the method parse_attribute() of the Parser struct implementation
  #[test]
  fn test_parse_attribute() {
    let mut parser: Parser = Parser::new(3, "<p class='paragraph'>Hello World!</p>".to_string());

    // Test consuming a node with the attribute `class='paragraph'`
    assert_eq!(
      parser.parse_attribute(),
      ("class".to_string(), "paragraph".to_string())
    );
    assert_eq!(parser.position, 20);
  }

  // Test the method parse_attribute_value() of the Parser struct implementation
  #[test]
  fn test_parse_attribute_value() {
    let mut parser: Parser = Parser::new(9, "<p class='paragraph'>Hello World!</p>".to_string());

    // Test consuming a node with the attribute value `paragraph`
    assert_eq!(parser.parse_attribute_value(), "paragraph".to_string());
    assert_eq!(parser.position, 20);
  }

  // Test the method parse_attributes() of the Parser struct implementation
  #[test]
  fn test_parse_attributes() {
    let mut parser: Parser = Parser::new(
      3,
      "<p class='paragraph' style='color:red;'>Hello World!</p>".to_string(),
    );
    let attributes: dom::AttributeMap = hashmap![String::from("class") => String::from("paragraph"), String::from("style") => String::from("color:red;")];

    // Test consuming a node with the attributes `class='paragraph'` and `style='color:red;'`
    assert_eq!(parser.parse_attributes(), attributes);
    assert_eq!(parser.position, 39);
  }

  // Test the method parse_nodes() of the Parser struct implementation
  #[test]
  fn test_parse_nodes() {
    let mut parser: Parser = Parser::new(
      0,
      "<div class='container-1'></div><div class='container-2'><p class='paragraph'>Hello World!</p></div>".to_string(),
    );
    // Node 1: <div class='container-1'>
    let tag_name: String = String::from("div");
    let attributes: dom::AttributeMap =
      hashmap![String::from("class") => String::from("container-1")];
    let children: Vec<dom::Node> = vec![];
    let node_1: dom::Node =
      dom::Node::element(tag_name.clone(), attributes.clone(), children.clone());
    // Node 3: <p class='paragraph'>
    let tag_name: String = String::from("p");
    let attributes: dom::AttributeMap =
      hashmap![String::from("class") => String::from("paragraph")];
    let children: Vec<dom::Node> = vec![dom::Node::text("Hello World!".to_string())];
    let node_3: dom::Node =
      dom::Node::element(tag_name.clone(), attributes.clone(), children.clone());
    // Node 2: <div class='container-2'>
    let tag_name: String = String::from("div");
    let attributes: dom::AttributeMap =
      hashmap![String::from("class") => String::from("container-2")];
    let children: Vec<dom::Node> = vec![node_3];
    let node_2: dom::Node =
      dom::Node::element(tag_name.clone(), attributes.clone(), children.clone());

    // Test consuming nested and sibling nodes: node_1, node_2 -> node_3
    assert_eq!(parser.parse_nodes(), vec![node_1, node_2]);
    assert_eq!(parser.position, 99);
  }

  // Test the method parse() of the Parser struct implementation
  #[test]
  fn test_parse() {
    // Node 2: <div class='container-1'>
    let tag_name: String = String::from("div");
    let attributes: dom::AttributeMap =
      hashmap![String::from("class") => String::from("container-1")];
    let children: Vec<dom::Node> = vec![];
    let node_2: dom::Node =
      dom::Node::element(tag_name.clone(), attributes.clone(), children.clone());
    // Node 2: <p class='paragraph'>
    let tag_name: String = String::from("p");
    let attributes: dom::AttributeMap =
      hashmap![String::from("class") => String::from("paragraph")];
    let children: Vec<dom::Node> = vec![dom::Node::text("Hello World!".to_string())];
    let node_4: dom::Node =
      dom::Node::element(tag_name.clone(), attributes.clone(), children.clone());
    // Node 3: <div class='container-2'>
    let tag_name: String = String::from("div");
    let attributes: dom::AttributeMap =
      hashmap![String::from("class") => String::from("container-2")];
    let children: Vec<dom::Node> = vec![node_4];
    let node_3: dom::Node =
      dom::Node::element(tag_name.clone(), attributes.clone(), children.clone());
    // Node 1: <html>
    let tag_name: String = String::from("html");
    let attributes: dom::AttributeMap = hashmap![];
    let children: Vec<dom::Node> = vec![node_2.clone(), node_3.clone()];
    let node_1: dom::Node =
      dom::Node::element(tag_name.clone(), attributes.clone(), children.clone());

    // Test parsing nodes without a root element
    assert_eq!(Parser::parse("<div class='container-1'></div><div class='container-2'><p class='paragraph'>Hello World!</p></div>".to_string()), node_1);
    // Test parsing nodes with a root element
    assert_eq!(
      Parser::parse(
        "<div class='container-2'><p class='paragraph'>Hello World!</p></div>".to_string()
      ),
      node_3
    );
  }
}
