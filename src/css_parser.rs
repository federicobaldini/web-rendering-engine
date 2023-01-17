/**
 * Features to add:
 * - Extend CSS parser to support more values, or one or more selector combinators;
 * - Extend the CSS parser to discard any declaration that contains a parse error, and follow
 *   the error handling rules to resume parsing after the end of the declaration;
 * - Make the HTML parser pass the contents of any <style> nodes to the CSS parser, and
 *   return a Document object that includes a list of Stylesheets in addition to the DOM tree;
 */
use std::str::CharIndices;

use crate::css;

fn valid_identifier_char(c: char) -> bool {
  match c {
    'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => true, // TODO: Include U+00A0 and higher
    _ => false,
  }
}

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

  // Parse a property name or keyword
  fn parse_identifier(&mut self) -> String {
    self.consume_while(valid_identifier_char)
  }

  fn parse_unit(&mut self) -> css::Unit {
    match &*self.parse_identifier().to_ascii_lowercase() {
      "px" => css::Unit::Px,
      _ => panic!("unrecognized unit"),
    }
  }

  fn parse_float(&mut self) -> f32 {
    let s: String = self.consume_while(|c: char| match c {
      '0'..='9' | '.' => true,
      _ => false,
    });
    s.parse().unwrap()
  }

  fn parse_length(&mut self) -> css::Value {
    css::Value::Length(self.parse_float(), self.parse_unit())
  }

  // Parse two hexadecimal digits
  fn parse_hex_pair(&mut self) -> u8 {
    let s: &str = &self.input[self.position..self.position + 2];
    self.position += 2;
    u8::from_str_radix(s, 16).unwrap()
  }

  fn parse_color(&mut self) -> css::Value {
    assert_eq!(self.consume_char(), '#');
    css::Value::ColorValue(css::Color::new(
      self.parse_hex_pair(),
      self.parse_hex_pair(),
      self.parse_hex_pair(),
      255,
    ))
  }

  fn parse_value(&mut self) -> css::Value {
    match self.next_char() {
      '0'..='9' => self.parse_length(),
      '#' => self.parse_color(),
      _ => css::Value::Keyword(self.parse_identifier()),
    }
  }

  // Parse one `<property>: <value>;` declaration
  fn parse_declaration(&mut self) -> css::Declaration {
    let property_name: String = self.parse_identifier();
    self.consume_whitespace();
    assert_eq!(self.consume_char(), ':');
    self.consume_whitespace();
    let value: css::Value = self.parse_value();
    self.consume_whitespace();
    assert_eq!(self.consume_char(), ';');

    css::Declaration::new(property_name, value)
  }

  // Parse a list of declarations enclosed in `{ ... }`
  fn parse_declarations(&mut self) -> Vec<css::Declaration> {
    assert_eq!(self.consume_char(), '{');
    let mut declarations: Vec<css::Declaration> = Vec::new();
    loop {
      self.consume_whitespace();
      if self.next_char() == '}' {
        self.consume_char();
        break;
      }
      declarations.push(self.parse_declaration());
    }
    declarations
  }

  // Parse one simple selector, e.g.: `type#id.class1.class2.class3`
  fn parse_simple_selector(&mut self) -> css::SimpleSelector {
    let mut selector: css::SimpleSelector = css::SimpleSelector::new(None, None, vec![]);
    while !self.eof() {
      match self.next_char() {
        '#' => {
          self.consume_char();
          selector.set_id(Some(self.parse_identifier()));
        }
        '.' => {
          self.consume_char();
          selector.add_class(self.parse_identifier());
        }
        '*' => {
          // universal selector
          self.consume_char();
        }
        c if valid_identifier_char(c) => {
          selector.set_tag_name(Some(self.parse_identifier()));
        }
        _ => break,
      }
    }
    return selector;
  }

  // Parse a comma-separated list of selectors
  fn parse_selectors(&mut self) -> Vec<css::Selector> {
    let mut selectors: Vec<css::Selector> = Vec::new();
    loop {
      selectors.push(css::Selector::Simple(self.parse_simple_selector()));
      self.consume_whitespace();
      match self.next_char() {
        ',' => {
          self.consume_char();
          self.consume_whitespace();
        }
        '{' => break, // start of declarations
        c => panic!("Unexpected character {} in selector list", c),
      }
    }
    // Return selectors with highest specificity first, for use in matching
    selectors.sort_by(|a: &css::Selector, b: &css::Selector| b.specificity().cmp(&a.specificity()));
    return selectors;
  }

  // Parse a rule set: `<selectors> { <declarations> }`
  fn parse_rule(&mut self) -> css::Rule {
    css::Rule::new(self.parse_selectors(), self.parse_declarations())
  }

  // Parse a list of rule sets, separated by optional whitespace
  fn parse_rules(&mut self) -> Vec<css::Rule> {
    let mut rules = Vec::new();
    loop {
      self.consume_whitespace();
      if self.eof() {
        break;
      }
      rules.push(self.parse_rule());
    }
    rules
  }

  // Parse a whole CSS stylesheet
  pub fn parse(source: String) -> css::Stylesheet {
    let mut parser: Parser = Parser::new(0, source);
    css::Stylesheet::new(parser.parse_rules())
  }
}

#[cfg(test)]
mod tests {
  use crate::css_parser::*;

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

  // Test the method parse_unit() of the Parser struct implementation
  #[test]
  #[should_panic]
  fn test_parse_unit() {
    let mut parser: Parser = Parser::new(19, ".container{width:200px}".to_string());

    // Test unit in position 19
    assert_eq!(parser.parse_unit(), css::Unit::Px);

    let mut parser = Parser::new(19, ".container{width:200em}".to_string());

    // Test unit in position 19, it has to panic because "em" unit is not supported
    parser.parse_unit();
  }

  // Test the method parse_float() of the Parser struct implementation
  #[test]
  #[should_panic]
  fn test_parse_float() {
    let mut parser: Parser = Parser::new(17, ".container{width:100.5px}".to_string());

    // Test float number in position 17
    assert_eq!(parser.parse_float(), 100.5);

    let mut parser: Parser = Parser::new(17, ".container{width:100px}".to_string());

    // Test integer number in position 17
    assert_eq!(parser.parse_float(), 100.0);

    let mut parser: Parser = Parser::new(17, ".container{width:ABCpx}".to_string());

    // Test number in position 17, it has to panic because "ABC" is not a number
    parser.parse_float();
  }
}
