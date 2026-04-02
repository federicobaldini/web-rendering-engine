/**
 * Features to add:
 * - Create an invalid HTML file that causes the parser fail. Modify the parser to recover
 *   from the error and produce a DOM tree for the test file;
 */
use crate::dom;
use crate::hashmap;
use crate::parser::text::TextParser;

#[cfg(test)]
mod tests;

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

  // Parse a comment
  fn parse_comment_content(&mut self) -> String {
    self.text_parser.consume_until_match("-->")
  }

  // Parse a text node
  fn parse_text(&mut self) -> dom::Node {
    dom::Node::text(self.text_parser.consume_while(|c: char| c != '<'))
  }

  // Parse a single element, including its open tag, contents, and closing tag
  fn parse_element(&mut self) -> Result<dom::Node, String> {
    let self_closing_tags_by_default: Vec<&str> = vec!["br", "img", "input", "meta", "link", "hr"];
    let mut self_closing_tag_flag: bool = false;
    let mut children: Vec<dom::Node> = vec![];

    // Opening tag
    self.text_parser.expect_char('<')?;
    let tag_name: String = self.parse_tag_name();
    let attributes: dom::AttributeMap = self.parse_attributes()?;

    if self.text_parser.next_char() == '/' {
      self_closing_tag_flag = true;
      self.text_parser.expect_char('/')?;
    }
    if self.text_parser.next_char() == '>' {
      // Closing character '>' could not be inline
      self.text_parser.consume_whitespace();
      self.text_parser.expect_char('>')?;
    }

    if !self_closing_tag_flag
      && self_closing_tags_by_default
        .iter()
        .all(|self_closing_tag| !tag_name.contains(self_closing_tag))
    {
      // Contents
      children = self.parse_nodes()?;

      // Recovery: if we reached EOF, the element was never closed — return it as-is
      if self.text_parser.eof() {
        return Ok(dom::Node::element(tag_name, attributes, children));
      }

      // Closing tag
      self.text_parser.expect_char('<')?;
      self.text_parser.expect_char('/')?;
      let closing_tag: String = self.parse_tag_name();
      if closing_tag != tag_name {
        // Recovery: mismatched closing tag — close the current element anyway and skip to '>'
        self.text_parser.consume_while(|c: char| c != '>');
        if !self.text_parser.eof() {
          self.text_parser.consume_char(); // consume '>'
        }
        return Ok(dom::Node::element(tag_name, attributes, children));
      }
      // Closing character '>' could not be inline
      self.text_parser.consume_whitespace();
      self.text_parser.expect_char('>')?;
    }
    Ok(dom::Node::element(tag_name, attributes, children))
  }

  // Parse a single comment, including its open tag, contents, and closing tag
  fn parse_comment(&mut self) -> Result<dom::Node, String> {
    // Opening tag
    self.text_parser.expect_char('<')?;
    self.text_parser.expect_char('!')?;
    self.text_parser.expect_char('-')?;
    self.text_parser.expect_char('-')?;

    // Comment
    let comment: String = self.parse_comment_content();

    // Closing characters '-->' could not be inline
    self.text_parser.consume_whitespace();
    // Closing tag
    self.text_parser.expect_char('-')?;
    self.text_parser.expect_char('-')?;
    self.text_parser.expect_char('>')?;

    Ok(dom::Node::comment(comment))
  }

  // Parse a single node
  fn parse_node(&mut self) -> Result<dom::Node, String> {
    match self.text_parser.next_char() {
      '<' => {
        if self.text_parser.next_offset_char(1) == '!' {
          return self.parse_comment();
        }
        self.parse_element()
      }
      _ => Ok(self.parse_text()),
    }
  }

  // Parse a single name="value" pair
  fn parse_attribute(&mut self) -> Result<(String, String), String> {
    let name: String = self.parse_tag_name();
    // Consume optional whitespace between the name and '=' (e.g. class = "foo")
    self.text_parser.consume_whitespace();
    // Recovery: an attribute without '=' is treated as a boolean attribute with an empty value
    if self.text_parser.eof() || self.text_parser.next_char() != '=' {
      return Ok((name, String::new()));
    }
    self.text_parser.expect_char('=')?;
    // Consume optional whitespace between '=' and the value (e.g. class= "foo")
    self.text_parser.consume_whitespace();
    let value: String = self.parse_attribute_value()?;
    Ok((name, value))
  }

  // Parse a quoted value
  fn parse_attribute_value(&mut self) -> Result<String, String> {
    let open_quote: char = self.text_parser.consume_char();
    if open_quote != '"' && open_quote != '\'' {
      // Recovery: unquoted value — read until whitespace, '/', or '>'
      let rest: String = self.text_parser.consume_while(|c: char| {
        c != ' ' && c != '\t' && c != '\n' && c != '/' && c != '>'
      });
      return Ok(format!("{}{}", open_quote, rest));
    }
    let value: String = self.text_parser.consume_while(|c: char| c != open_quote);
    self.text_parser.expect_char(open_quote)?;
    Ok(value)
  }

  // Parse a list of name="value" pairs, separated by whitespace
  fn parse_attributes(&mut self) -> Result<dom::AttributeMap, String> {
    let mut attributes: dom::AttributeMap = hashmap![];
    loop {
      self.text_parser.consume_whitespace();
      if self.text_parser.next_char() == '>' || self.text_parser.next_char() == '/' {
        break;
      }
      let (name, value): (String, String) = self.parse_attribute()?;
      attributes.insert(name, value);
    }
    Ok(attributes)
  }

  // Parse a sequence of sibling nodes
  fn parse_nodes(&mut self) -> Result<Vec<dom::Node>, String> {
    let mut nodes: Vec<dom::Node> = Vec::new();
    loop {
      self.text_parser.consume_whitespace();
      if self.text_parser.eof() || self.text_parser.starts_with("</") {
        break;
      }
      nodes.push(self.parse_node()?);
    }
    Ok(nodes)
  }

  // Parse an HTML document and return the root element
  pub fn parse(source: String) -> Result<dom::Node, String> {
    let mut nodes: Vec<dom::Node> = HTMLParser::new(0, source).parse_nodes()?;

    // If the document contains a root element, just return it. Otherwise, create one
    Ok(if nodes.len() == 1 {
      nodes.swap_remove(0)
    } else {
      dom::Node::element("html".to_string(), hashmap![], nodes)
    })
  }
}
