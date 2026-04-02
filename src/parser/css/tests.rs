use crate::css;
use crate::css::Stylesheet;
use super::*;

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
  assert_eq!(css_parser.parse_unit().unwrap(), css::Unit::Px);
}

// Test the method parse_float of the CSSParser struct implementation
#[test]
fn test_parse_float() {
  let mut css_parser: CSSParser = CSSParser::new(17, ".container{width:100.5px;}".to_string());

  // Assert that the parse_float method correctly parses the float value "100.5"
  assert_eq!(css_parser.parse_float().unwrap(), 100.5);

  let mut css_parser: CSSParser = CSSParser::new(17, ".container{width:100px;}".to_string());

  // Assert that the parse_float method correctly parses the float value "100"
  assert_eq!(css_parser.parse_float().unwrap(), 100.0);
}

// Test the method parse_length of the CSSParser struct implementation
#[test]
fn test_parse_length() {
  let mut css_parser: CSSParser = CSSParser::new(17, ".container{width:100.5px;}".to_string());
  let unit: css::Value = css::Value::Length(100.5, css::Unit::Px);

  // Assert that the parse_value method correctly parses the value "100.5" with unit "px"
  assert_eq!(css_parser.parse_length().unwrap(), unit);
}

// Test the method parse_hex_pair of the CSSParser struct implementation
#[test]
fn test_parse_hex_pair() {
  let mut css_parser: CSSParser =
    CSSParser::new(23, ".container{background:#A3E4D7;}".to_string());

  // Assert that the parse_hex_pair method correctly parses the first hex pair "A3" as 163
  assert_eq!(css_parser.parse_hex_pair().unwrap(), 163);
  // Assert that the parse_hex_pair method correctly parses the second hex pair "E4" as 228
  assert_eq!(css_parser.parse_hex_pair().unwrap(), 228);
  // Assert that the parse_hex_pair method correctly parses the third hex pair "D7" as 215
  assert_eq!(css_parser.parse_hex_pair().unwrap(), 215);
}

// Test the method parse_color of the CSSParser struct implementation
#[test]
fn test_parse_color() {
  let mut css_parser: CSSParser =
    CSSParser::new(22, ".container{background:#A3E4D7;}".to_string());
  let color: css::Value = css::Value::ColorValue(css::Color::new(163, 228, 215, 255));

  // Assert that the parse_color method correctly parses the color "A3E4D7"
  assert_eq!(css_parser.parse_color().unwrap(), color);
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
  let color: css::Value = css::Value::ColorValue(css::Color::new(163, 228, 215, 255));

  // Assert that the parse_value method correctly parses the keyword "width"
  assert_eq!(css_parser.parse_value().unwrap(), keyword);

  css_parser.text_parser.increment_position(1);
  // Assert that the parse_value method correctly parses the value "100" with unit "px"
  assert_eq!(css_parser.parse_value().unwrap(), unit);

  css_parser.text_parser.increment_position(12);
  // Assert that the parse_color method correctly parses the color "A3E4D7"
  assert_eq!(css_parser.parse_value().unwrap(), color);
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
  assert_eq!(css_parser.parse_declaration().unwrap(), declaration);
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
  let color: css::Value = css::Value::ColorValue(css::Color::new(163, 228, 215, 255));
  let declaration_2: css::Declaration = css::Declaration::new("background".to_string(), color);

  // Assert that the parse_declarations method correctly parses the declarations "{width: 100px;background:#A3E4D7;}"
  assert_eq!(
    css_parser.parse_declarations().unwrap(),
    vec![declaration_1, declaration_2]
  );
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
  assert_eq!(
    css_parser.parse_selectors().unwrap(),
    vec![selector_1, selector_2]
  );
}

// Test the method parse_rule of the CSSParser struct implementation
#[test]
fn test_parse_rule() {
  let mut css_parser: CSSParser = CSSParser::new(0, ".class1{width:100px;}".to_string());
  // Selector
  let simple_selector: css::SimpleSelector =
    css::SimpleSelector::new(None, None, vec!["class1".to_string()]);
  let selector: css::Selector = css::Selector::Simple(simple_selector);
  // Declaration
  let unit: css::Value = css::Value::Length(100.0, css::Unit::Px);
  let declaration: css::Declaration = css::Declaration::new("width".to_string(), unit);
  // Rule
  let rule: css::Rule = css::Rule::new(vec![selector], vec![declaration]);

  // Assert that the parse_rule method correctly parses the selector and its declaration ".class1{width:100px;}"
  assert_eq!(css_parser.parse_rule().unwrap(), rule);
}

// Test the method parse_rules of the CSSParser struct implementation
#[test]
fn test_parse_rules() {
  let mut css_parser: CSSParser = CSSParser::new(
    0,
    ".class1{width:100px;}.class2{background:#A3E4D7;}".to_string(),
  );
  // Selectors
  let simple_selector_1: css::SimpleSelector =
    css::SimpleSelector::new(None, None, vec!["class1".to_string()]);
  let simple_selector_2: css::SimpleSelector =
    css::SimpleSelector::new(None, None, vec!["class2".to_string()]);
  let selector_1: css::Selector = css::Selector::Simple(simple_selector_1);
  let selector_2: css::Selector = css::Selector::Simple(simple_selector_2);
  // Declarations
  let unit: css::Value = css::Value::Length(100.0, css::Unit::Px);
  let color: css::Value = css::Value::ColorValue(css::Color::new(163, 228, 215, 255));
  let declaration_1: css::Declaration = css::Declaration::new("width".to_string(), unit);
  let declaration_2: css::Declaration = css::Declaration::new("background".to_string(), color);
  // Rules
  let rule_1: css::Rule = css::Rule::new(vec![selector_1], vec![declaration_1]);
  let rule_2: css::Rule = css::Rule::new(vec![selector_2], vec![declaration_2]);

  // Assert that the parse_rules method correctly parses the selectors and their declaration ".class1{width:100px;}.class2{background:#A3E4D7;}"
  assert_eq!(css_parser.parse_rules().unwrap(), vec![rule_1, rule_2]);
}

// Test the method parse of the CSSParser struct implementation
#[test]
fn test_parse() {
  // Selectors
  let simple_selector_1: css::SimpleSelector = css::SimpleSelector::new(
    Some("div".to_string()),
    Some("main-container".to_string()),
    vec!["class1".to_string()],
  );
  let simple_selector_2: css::SimpleSelector =
    css::SimpleSelector::new(None, None, vec!["class2".to_string()]);
  let selector_1: css::Selector = css::Selector::Simple(simple_selector_1);
  let selector_2: css::Selector = css::Selector::Simple(simple_selector_2);
  // Declarations
  let unit_1: css::Value = css::Value::Length(100.0, css::Unit::Px);
  let unit_2: css::Value = css::Value::Length(200.0, css::Unit::Px);
  let color: css::Value = css::Value::ColorValue(css::Color::new(163, 228, 215, 255));
  let declaration_1: css::Declaration = css::Declaration::new("width".to_string(), unit_1);
  let declaration_2: css::Declaration = css::Declaration::new("height".to_string(), unit_2);
  let declaration_3: css::Declaration = css::Declaration::new("background".to_string(), color);
  // Rules
  let rule_1: css::Rule = css::Rule::new(vec![selector_1], vec![declaration_1]);
  let rule_2: css::Rule = css::Rule::new(vec![selector_2], vec![declaration_2, declaration_3]);

  // Assert that the parse method correctly parses ".class1{width:100px;}.class2{background:#A3E4D7;}"
  assert_eq!(
    CSSParser::parse(
      "div#main-container.class1{width:100px;}.class2{height:200px;background:#A3E4D7;}"
        .to_string()
    )
    .unwrap(),
    Stylesheet::new(vec![rule_1, rule_2])
  );
}

// Test recovery: a declaration with an unknown unit is discarded; valid ones are kept
#[test]
fn test_recovery_invalid_declaration() {
  let stylesheet: Stylesheet =
    CSSParser::parse(".foo{width:100px;height:50unknownunit;background:#ff0000;}".to_string())
      .unwrap();

  // Assert that only 1 rule was parsed
  assert_eq!(stylesheet.rules().len(), 1);

  let declarations: &Vec<css::Declaration> = stylesheet.rules()[0].declarations();

  // Assert that the invalid declaration was discarded and the two valid ones are kept
  assert_eq!(declarations.len(), 2);
  assert_eq!(declarations[0].name(), "width");
  assert_eq!(declarations[0].value(), &css::Value::Length(100.0, css::Unit::Px));
  assert_eq!(declarations[1].name(), "background");
  assert_eq!(
    declarations[1].value(),
    &css::Value::ColorValue(css::Color::new(255, 0, 0, 255))
  );
}

// Test recovery: a rule with an invalid selector is discarded; valid rules are kept
#[test]
fn test_recovery_invalid_rule() {
  let stylesheet: Stylesheet = CSSParser::parse(
    ".valid1{width:100px;}!!!invalid!!!{color:red;}.valid2{height:50px;}".to_string(),
  )
  .unwrap();

  // Assert that only the 2 valid rules were kept; the invalid one was discarded
  assert_eq!(stylesheet.rules().len(), 2);

  let declarations_1: &Vec<css::Declaration> = stylesheet.rules()[0].declarations();
  assert_eq!(declarations_1.len(), 1);
  assert_eq!(declarations_1[0].name(), "width");
  assert_eq!(declarations_1[0].value(), &css::Value::Length(100.0, css::Unit::Px));

  let declarations_2: &Vec<css::Declaration> = stylesheet.rules()[1].declarations();
  assert_eq!(declarations_2.len(), 1);
  assert_eq!(declarations_2[0].name(), "height");
  assert_eq!(declarations_2[0].value(), &css::Value::Length(50.0, css::Unit::Px));
}
