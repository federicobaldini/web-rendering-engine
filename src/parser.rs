use crate::dom;
use crate::hashmap;
use std::str::CharIndices;

pub struct Parser {
  position: usize,
  input: String,
}

impl Parser {
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
    let mut nodes: Vec<dom::Node> = Parser {
      position: 0,
      input: source,
    }
    .parse_nodes();

    // If the document contains a root element, just return it. Otherwise, create one
    if nodes.len() == 1 {
      nodes.swap_remove(0)
    } else {
      dom::Node::element("html".to_string(), hashmap![], nodes)
    }
  }
}
