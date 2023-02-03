/**
 * Features to add:
 * - Extend CSS parser to support more values, or one or more selector combinators;
 * - Extend the CSS parser to discard any declaration that contains a parse error, and follow
 *   the error handling rules to resume parsing after the end of the declaration;
 * - Make the HTML parser pass the contents of any <style> nodes to the CSS parser, and
 *   return a Document object that includes a list of Stylesheets in addition to the DOM tree;
 */
use crate::css;
use crate::text_parser::TextParser;

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

  fn parse_unit(&mut self) -> css::Unit {
    match &*self.parse_identifier().to_ascii_lowercase() {
      "px" => css::Unit::Px,
      _ => panic!("unrecognized unit"),
    }
  }

  fn parse_float(&mut self) -> f32 {
    let s: String = self.text_parser.consume_while(|c: char| match c {
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
    let s: &str =
      &self.text_parser.input()[self.text_parser.position()..self.text_parser.position() + 2];
    self.text_parser.increment_position(2);
    u8::from_str_radix(s, 16).unwrap()
  }

  fn parse_color(&mut self) -> css::Value {
    assert_eq!(self.text_parser.consume_char(), '#');
    css::Value::ColorValue(css::Color::new(
      self.parse_hex_pair(),
      self.parse_hex_pair(),
      self.parse_hex_pair(),
      1,
    ))
  }

  fn parse_value(&mut self) -> css::Value {
    match self.text_parser.next_char() {
      '0'..='9' => self.parse_length(),
      '#' => self.parse_color(),
      _ => css::Value::Keyword(self.parse_identifier()),
    }
  }

  // Parse one `<property>: <value>;` declaration
  fn parse_declaration(&mut self) -> css::Declaration {
    let property_name: String = self.parse_identifier();
    self.text_parser.consume_whitespace();
    assert_eq!(self.text_parser.consume_char(), ':');
    self.text_parser.consume_whitespace();
    let value: css::Value = self.parse_value();
    self.text_parser.consume_whitespace();
    assert_eq!(self.text_parser.consume_char(), ';');

    css::Declaration::new(property_name, value)
  }

  // Parse a list of declarations enclosed in `{ ... }`
  fn parse_declarations(&mut self) -> Vec<css::Declaration> {
    assert_eq!(self.text_parser.consume_char(), '{');
    let mut declarations: Vec<css::Declaration> = Vec::new();
    loop {
      self.text_parser.consume_whitespace();
      if self.text_parser.next_char() == '}' {
        self.text_parser.consume_char();
        break;
      }
      declarations.push(self.parse_declaration());
    }
    declarations
  }

  // Parse one simple selector, e.g.: `type#id.class1.class2.class3`
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
  fn parse_selectors(&mut self) -> Vec<css::Selector> {
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
      self.text_parser.consume_whitespace();
      if self.text_parser.eof() {
        break;
      }
      rules.push(self.parse_rule());
    }
    rules
  }

  // Parse a whole CSS stylesheet
  pub fn parse(source: String) -> css::Stylesheet {
    let mut css_parser: CSSParser = CSSParser::new(0, source);
    css::Stylesheet::new(css_parser.parse_rules())
  }
}

#[cfg(test)]
mod tests {
  use crate::css_parser::*;

  // Test the method parse_identifier of the CSSParser struct implementation
  #[test]
  fn test_parse_identifier() {
    let mut css_parser: CSSParser = CSSParser::new(11, ".container{width:100px;}".to_string());

    // Assert that the parse_value method correctly parses the string "width"
    assert_eq!(css_parser.parse_identifier(), "width");
  }

  // Test the method parse_unit of the CSSParser struct implementation
  #[test]
  fn test_parse_unit() {
    let mut css_parser: CSSParser = CSSParser::new(20, ".container{width:200px;}".to_string());

    // Assert that the parse_float method correctly parses the unit "px"
    assert_eq!(css_parser.parse_unit(), css::Unit::Px);
  }

  // Test the method parse_float of the CSSParser struct implementation
  #[test]
  fn test_parse_float() {
    let mut css_parser: CSSParser = CSSParser::new(17, ".container{width:100.5px;}".to_string());

    // Assert that the parse_float method correctly parses the float value "100.5"
    assert_eq!(css_parser.parse_float(), 100.5);

    let mut css_parser: CSSParser = CSSParser::new(17, ".container{width:100px;}".to_string());

    // Assert that the parse_float method correctly parses the float value "100"
    assert_eq!(css_parser.parse_float(), 100.0);
  }

  // Test the method parse_length of the CSSParser struct implementation
  #[test]
  fn test_parse_length() {
    let mut css_parser: CSSParser = CSSParser::new(17, ".container{width:100.5px;}".to_string());
    let unit: css::Value = css::Value::Length(100.5, css::Unit::Px);

    // Assert that the parse_value method correctly parses the value "100.5" with unit "px"
    assert_eq!(css_parser.parse_length(), unit);
  }

  // Test the method parse_hex_pair of the CSSParser struct implementation
  #[test]
  fn test_parse_hex_pair() {
    let mut css_parser: CSSParser =
      CSSParser::new(23, ".container{background:#A3E4D7;}".to_string());

    // Assert that the parse_hex_pair method correctly parses the first hex pair "A3" as 163
    assert_eq!(css_parser.parse_hex_pair(), 163);
    // Assert that the parse_hex_pair method correctly parses the second hex pair "E4" as 228
    assert_eq!(css_parser.parse_hex_pair(), 228);
    // Assert that the parse_hex_pair method correctly parses the third hex pair "D7" as 215
    assert_eq!(css_parser.parse_hex_pair(), 215);
  }

  // Test the method parse_color of the CSSParser struct implementation
  #[test]
  fn test_parse_color() {
    let mut css_parser: CSSParser =
      CSSParser::new(22, ".container{background:#A3E4D7;}".to_string());
    let color: css::Value = css::Value::ColorValue(css::Color::new(163, 228, 215, 1));

    // Assert that the parse_color method correctly parses the color "A3E4D7"
    assert_eq!(css_parser.parse_color(), color);
  }

  // Test the method parse_value of the CSSParser struct implementation
  #[test]
  fn test_parse_value() {
    let mut css_parser: CSSParser = CSSParser::new(
      11,
      ".container{width:100px;background:#A3E4D7;}".to_string(),
    );
    let keyword: css::Value = css::Value::Keyword(String::from("width"));
    let unit: css::Value = css::Value::Length(100.0, css::Unit::Px);
    let color: css::Value = css::Value::ColorValue(css::Color::new(163, 228, 215, 1));

    // Assert that the parse_value method correctly parses the keyword "width"
    assert_eq!(css_parser.parse_value(), keyword);

    css_parser.text_parser.increment_position(1);
    // Assert that the parse_value method correctly parses the value "100" with unit "px"
    assert_eq!(css_parser.parse_value(), unit);

    css_parser.text_parser.increment_position(12);
    // Assert that the parse_color method correctly parses the color "A3E4D7"
    assert_eq!(css_parser.parse_value(), color);
  }

  // Test the method parse_declaration of the CSSParser struct implementation
  #[test]
  fn test_parse_declaration() {
    let mut css_parser: CSSParser = CSSParser::new(
      11,
      ".container{width:100px;background:#A3E4D7;}".to_string(),
    );
    let unit: css::Value = css::Value::Length(100.0, css::Unit::Px);
    let declaration: css::Declaration = css::Declaration::new("width".to_string(), unit);

    // Assert that the parse_declaration method correctly parses the declaration "width: 100px;"
    assert_eq!(css_parser.parse_declaration(), declaration);
  }

  // Test the method parse_declarations of the CSSParser struct implementation
  #[test]
  fn test_parse_declarations() {
    let mut css_parser: CSSParser = CSSParser::new(
      10,
      ".container{width:100px;background:#A3E4D7;}".to_string(),
    );
    let unit: css::Value = css::Value::Length(100.0, css::Unit::Px);
    let declaration_1: css::Declaration = css::Declaration::new("width".to_string(), unit);
    let color: css::Value = css::Value::ColorValue(css::Color::new(163, 228, 215, 1));
    let declaration_2: css::Declaration = css::Declaration::new("background".to_string(), color);
    let declarations: Vec<css::Declaration> = vec![declaration_1, declaration_2];

    // Assert that the parse_declarations method correctly parses the declarations "{width: 100px;background:#A3E4D7;}"
    assert_eq!(css_parser.parse_declarations(), declarations);
  }

  // Test the method parse_simple_selector of the CSSParser struct implementation
  #[test]
  fn test_parse_simple_selector() {
    let mut css_parser: CSSParser =
      CSSParser::new(0, "div#main-container.class1.class2{}".to_string());
    let simple_selector: css::SimpleSelector = css::SimpleSelector::new(
      Some("div".to_string()),
      Some("main-container".to_string()),
      vec!["class1".to_string(), "class2".to_string()],
    );

    // Assert that the parse_declarations method correctly parses the simple selector "div#main-container.class1.class2"
    assert_eq!(css_parser.parse_simple_selector(), simple_selector);
  }

  // Test the method parse_selectors of the CSSParser struct implementation
  #[test]
  fn test_parse_selectors() {
    let mut css_parser: CSSParser = CSSParser::new(
      0,
      "div#main-container.class1.class2,h1#main-title.class3.class4{}".to_string(),
    );
    let simple_selector_1: css::SimpleSelector = css::SimpleSelector::new(
      Some("div".to_string()),
      Some("main-container".to_string()),
      vec!["class1".to_string(), "class2".to_string()],
    );
    let simple_selector_2: css::SimpleSelector = css::SimpleSelector::new(
      Some("h1".to_string()),
      Some("main-title".to_string()),
      vec!["class3".to_string(), "class4".to_string()],
    );
    let selector_1: css::Selector = css::Selector::Simple(simple_selector_1);
    let selector_2: css::Selector = css::Selector::Simple(simple_selector_2);

    // Assert that the parse_selectors method correctly parses the selectors "div#main-container.class1.class2" and "h1#main-title.class3.class4"
    assert_eq!(css_parser.parse_selectors(), vec![selector_1, selector_2]);
  }
}
