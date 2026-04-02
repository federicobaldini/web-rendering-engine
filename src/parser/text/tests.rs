use super::*;

// Test the method next_char of the TextParser struct implementation
#[test]
fn test_next_char() {
  let mut text_parser: TextParser = TextParser::new(0, "<p>Hello World!</p>".to_string());

  // Assert that the next_char method correctly returns the character '<'
  assert_eq!(text_parser.next_char(), '<');

  text_parser.position = 1;
  // Assert that the next_char method correctly returns the character 'p'
  assert_eq!(text_parser.next_char(), 'p');

  text_parser.position = 18;
  // Assert that the next_char method correctly returns the character '>'
  assert_eq!(text_parser.next_char(), '>');
}

// Test the method next_offset_char of the TextParser struct implementation
#[test]
fn test_next_offset_char() {
  let text_parser: TextParser = TextParser::new(0, "<p>Hello World!</p>".to_string());

  // Assert that the next_offset_char method correctly returns the character '<'
  assert_eq!(text_parser.next_offset_char(1), 'p');

  // Assert that the next_offset_char method correctly returns the character 'p'
  assert_eq!(text_parser.next_offset_char(9), 'W');

  // Assert that the next_offset_char method correctly returns the character '>'
  assert_eq!(text_parser.next_offset_char(18), '>');
}

// Test the method starts_with of the TextParser struct implementation
#[test]
fn test_starts_with() {
  let mut text_parser: TextParser = TextParser::new(0, "<p>Hello World!</p>".to_string());

  // Assert that the starts_with method correctly returns true because the input starts with the string "<"
  assert!(text_parser.starts_with("<"));
  // Assert that the starts_with method correctly returns true because the input starts with the string "<p>"
  assert!(text_parser.starts_with("<p>"));
  // Assert that the starts_with method correctly returns false because the input doesn't start with the string "Hello"
  assert!(!text_parser.starts_with("Hello"));
  // Assert that the starts_with method correctly returns false because the input doesn't start with the string "!"
  assert!(!text_parser.starts_with("!"));

  text_parser.increment_position(3);
  // Assert that the starts_with method correctly returns false because the input doesn't start with the string "<"
  assert!(!text_parser.starts_with("<"));
  // Assert that the starts_with method correctly returns false because the input doesn't start with the string "<p>"
  assert!(!text_parser.starts_with("<p>"));
  // Assert that the starts_with method correctly returns true because the input starts with the string "H"
  assert!(text_parser.starts_with("H"));
  // Assert that the starts_with method correctly returns true because the input starts with the string "Hello"
  assert!(text_parser.starts_with("Hello"));
}

// Test the method eof of the TextParser struct implementation
#[test]
fn test_eof() {
  let mut text_parser: TextParser = TextParser::new(0, "<p>Hello World!</p>".to_string());

  // Assert that the eof method correctly returns false because the current position is not at the end of the input string
  assert_eq!(text_parser.eof(), false);

  text_parser.increment_position(5);
  // Assert that the eof method correctly returns false because the current position is not at the end of the input string
  assert_eq!(text_parser.eof(), false);

  text_parser.increment_position(19);
  // Assert that the eof method correctly returns true because the current position is at the end of the input string
  assert_eq!(text_parser.eof(), true);
}

// Test the method consume_char of the TextParser struct implementation
#[test]
fn test_consume_char() {
  let mut text_parser: TextParser = TextParser::new(0, "<p>Hello World!</p>".to_string());

  // Assert that the consume_char method correctly consumes only the character '<'
  // and correctly returns the consumed characters as a character
  assert_eq!(text_parser.consume_char(), '<');
  // Assert that the position is correctly updated to 1 after consuming the character
  assert_eq!(text_parser.position, 1);

  text_parser.position = 3;
  // Assert that the consume_char method correctly consumes only the character 'H'
  // and correctly returns the consumed characters as a character
  assert_eq!(text_parser.consume_char(), 'H');
  // Assert that the position is correctly updated to 4 after consuming the character
  assert_eq!(text_parser.position, 4);

  text_parser.position = 8;
  // Assert that the consume_char method correctly consumes only the character ' '
  // and correctly returns the consumed characters as a character
  assert_eq!(text_parser.consume_char(), ' ');
  // Assert that the position is correctly updated to 9 after consuming the character
  assert_eq!(text_parser.position, 9);
}

// Test the method consume_while of the TextParser struct implementation
#[test]
fn test_consume_while() {
  let mut text_parser: TextParser = TextParser::new(3, "<p>Hello World!</p>".to_string());

  // Assert that the consume_while method correctly consumes only the alphabetic characters "Hello"
  // and correctly returns the consumed characters as a string
  assert_eq!(text_parser.consume_while(|c| c.is_alphabetic()), "Hello");
  // Assert that the position is correctly updated to 8 after consuming the characters
  assert_eq!(text_parser.position, 8);

  // Assert that the consume_while method correctly consumes only the whitespace character ' '
  // and correctly returns the consumed character as a string
  assert_eq!(text_parser.consume_while(|c| c.is_whitespace()), " ");
  // Assert that the position is correctly updated to 9 after consuming the characters
  assert_eq!(text_parser.position, 9);

  // Assert that the consume_while method correctly returns an empty string when no digits are found
  // and correctly returns an empty character as a string
  assert_eq!(text_parser.consume_while(|c| c.is_digit(10)), "");
  // Assert that the position is correctly updated to 9 after consuming the characters
  assert_eq!(text_parser.position, 9);
}

// Test the method consume_until_match of the TextParser struct implementation
#[test]
fn test_consume_until_match() {
  let mut text_parser: TextParser = TextParser::new(4, "<!-- Hello World! -->".to_string());

  // Assert that the consume_until_match method correctly consumes everything less the keyword
  // and correctly returns the consumed characters as a string
  assert_eq!(text_parser.consume_until_match("-->"), " Hello World! ");
  // Assert that the position is correctly updated to 8 after consuming the characters
  assert_eq!(text_parser.position, 18);
}
