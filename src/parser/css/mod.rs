/**
 * Features to add:
 * - Extend CSS parser to support more values, or one or more selector combinators;
 * - Extend the CSS parser to discard any declaration that contains a parse error, and follow
 *   the error handling rules to resume parsing after the end of the declaration;
 * - Make the HTML parser pass the contents of any <style> nodes to the CSS parser, and
 *   return a Document object that includes a list of Stylesheets in addition to the DOM tree;
 */
use crate::css;
use crate::parser::text::TextParser;

#[cfg(test)]
mod tests;

fn valid_identifier_char(c: char) -> bool {
  match c {
    'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => true, // TODO: Include U+00A0 and higher
    _ => false,
  }
}

pub struct CSSParser {
  text_parser: TextParser,
}

impl CSSParser {
  pub fn new(position: usize, input: String) -> Self {
    Self {
      text_parser: TextParser::new(position, input),
    }
  }

  // Parse a property name or keyword
  fn parse_identifier(&mut self) -> String {
    self.text_parser.consume_while(valid_identifier_char)
  }

  fn parse_unit(&mut self) -> Result<css::Unit, String> {
    match &*self.parse_identifier().to_ascii_lowercase() {
      "px" => Ok(css::Unit::Px),
      unit => Err(format!("Unrecognized unit: '{}'", unit)),
    }
  }

  fn parse_float(&mut self) -> Result<f32, String> {
    let s: String = self.text_parser.consume_while(|c: char| match c {
      '0'..='9' | '.' => true,
      _ => false,
    });
    s.parse::<f32>().map_err(|_| format!("Invalid float value: '{}'", s))
  }

  fn parse_length(&mut self) -> Result<css::Value, String> {
    Ok(css::Value::Length(self.parse_float()?, self.parse_unit()?))
  }

  // Parse two hexadecimal digits
  fn parse_hex_pair(&mut self) -> Result<u8, String> {
    let pos: usize = self.text_parser.position();
    if pos + 2 > self.text_parser.input().len() {
      return Err("Unexpected end of input while parsing hex value".to_string());
    }
    let s: String = self.text_parser.input()[pos..pos + 2].to_string();
    self.text_parser.increment_position(2);
    u8::from_str_radix(&s, 16).map_err(|_| format!("Invalid hex value: '{}'", s))
  }

  fn parse_color(&mut self) -> Result<css::Value, String> {
    self.text_parser.expect_char('#')?;
    Ok(css::Value::ColorValue(css::Color::new(
      self.parse_hex_pair()?,
      self.parse_hex_pair()?,
      self.parse_hex_pair()?,
      255,
    )))
  }

  fn parse_value(&mut self) -> Result<css::Value, String> {
    match self.text_parser.next_char() {
      '0'..='9' => self.parse_length(),
      '#' => self.parse_color(),
      _ => Ok(css::Value::Keyword(self.parse_identifier())),
    }
  }

  // Parse one '<property>: <value>;' declaration
  fn parse_declaration(&mut self) -> Result<css::Declaration, String> {
    let property_name: String = self.parse_identifier();
    self.text_parser.consume_whitespace();
    self.text_parser.expect_char(':')?;
    self.text_parser.consume_whitespace();
    let value: css::Value = self.parse_value()?;
    self.text_parser.consume_whitespace();
    self.text_parser.expect_char(';')?;

    Ok(css::Declaration::new(property_name, value))
  }

  // Parse a list of declarations enclosed in '{ ... }'
  fn parse_declarations(&mut self) -> Result<Vec<css::Declaration>, String> {
    self.text_parser.expect_char('{')?;
    let mut declarations: Vec<css::Declaration> = Vec::new();
    loop {
      self.text_parser.consume_whitespace();
      if self.text_parser.next_char() == '}' {
        self.text_parser.consume_char();
        break;
      }
      match self.parse_declaration() {
        Ok(declaration) => declarations.push(declaration),
        Err(_) => {
          // Recovery: discard this declaration, skip to the next ';' or '}'
          self.text_parser.consume_while(|c: char| c != ';' && c != '}');
          if !self.text_parser.eof() && self.text_parser.next_char() == ';' {
            self.text_parser.consume_char(); // consume ';'
          }
        }
      }
    }
    Ok(declarations)
  }

  // Parse one simple selector, e.g.: 'type#id.class1.class2.class3'
  fn parse_simple_selector(&mut self) -> css::SimpleSelector {
    let mut selector: css::SimpleSelector = css::SimpleSelector::new(None, None, vec![]);
    while !self.text_parser.eof() {
      match self.text_parser.next_char() {
        '#' => {
          self.text_parser.consume_char();
          selector.set_id(Some(self.parse_identifier()));
        }
        '.' => {
          self.text_parser.consume_char();
          selector.add_class(self.parse_identifier());
        }
        '*' => {
          // universal selector
          self.text_parser.consume_char();
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
  fn parse_selectors(&mut self) -> Result<Vec<css::Selector>, String> {
    let mut selectors: Vec<css::Selector> = Vec::new();
    loop {
      selectors.push(css::Selector::Simple(self.parse_simple_selector()));
      self.text_parser.consume_whitespace();
      match self.text_parser.next_char() {
        ',' => {
          self.text_parser.consume_char();
          self.text_parser.consume_whitespace();
        }
        '{' => break, // start of declarations
        c => return Err(format!("Unexpected character '{}' in selector list", c)),
      }
    }
    // Return selectors with highest specificity first, for use in matching
    selectors.sort_by(|a: &css::Selector, b: &css::Selector| b.specificity().cmp(&a.specificity()));
    Ok(selectors)
  }

  // Parse a rule set: '<selectors> { <declarations> }'
  fn parse_rule(&mut self) -> Result<css::Rule, String> {
    Ok(css::Rule::new(self.parse_selectors()?, self.parse_declarations()?))
  }

  // Parse a list of rule sets, separated by optional whitespace
  fn parse_rules(&mut self) -> Result<Vec<css::Rule>, String> {
    let mut rules: Vec<css::Rule> = Vec::new();
    loop {
      self.text_parser.consume_whitespace();
      if self.text_parser.eof() {
        break;
      }
      match self.parse_rule() {
        Ok(rule) => rules.push(rule),
        Err(_) => {
          // Recovery: discard this rule, skip to its closing '}'
          self.text_parser.consume_while(|c: char| c != '}');
          if !self.text_parser.eof() {
            self.text_parser.consume_char(); // consume '}'
          }
        }
      }
    }
    Ok(rules)
  }

  // Parse a whole CSS stylesheet
  pub fn parse(source: String) -> Result<css::Stylesheet, String> {
    let mut css_parser: CSSParser = CSSParser::new(0, source);
    Ok(css::Stylesheet::new(css_parser.parse_rules()?))
  }
}
