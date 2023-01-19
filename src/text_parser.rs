use std::str::CharIndices;

pub struct TextParser {
  position: usize,
  input: String,
}

impl TextParser {
  pub fn new(position: usize, input: String) -> TextParser {
    TextParser { position, input }
  }

  pub fn position(&self) -> usize {
    self.position
  }
  pub fn input(&self) -> String {
    self.input.clone()
  }

  // Read the current character without consuming it
  pub fn next_char(&self) -> char {
    self.input[self.position..].chars().next().unwrap()
  }

  // Do the next characters start with the given string?
  pub fn starts_with(&self, s: &str) -> bool {
    self.input[self.position..].starts_with(s)
  }

  // Return true if all input is consumed
  pub fn eof(&self) -> bool {
    self.position >= self.input.len()
  }

  // Return the current character, and advance self.position to the next character
  pub fn consume_char(&mut self) -> char {
    let mut iter: CharIndices = self.input[self.position..].char_indices();
    let (_, current_char): (_, char) = iter.next().unwrap();
    let (next_position, _): (usize, _) = iter.next().unwrap_or((1, ' '));
    self.position += next_position;
    return current_char;
  }

  // Consume characters until `test` returns false
  pub fn consume_while<F>(&mut self, test: F) -> String
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
  pub fn consume_whitespace(&mut self) {
    self.consume_while(char::is_whitespace);
  }
}

#[cfg(test)]
mod tests {
  use crate::text_parser::*;

  // Test the method next_char of the TextParser struct implementation
  #[test]
  fn test_next_char() {
    let mut parser: TextParser = TextParser::new(0, "<p>Hello World!</p>".to_string());

    // Assert that the next_char method return the character '<'
    assert_eq!(parser.next_char(), '<');

    parser.position = 1;
    // Assert that the next_char method return the character 'p'
    assert_eq!(parser.next_char(), 'p');

    parser.position = 18;
    // Assert that the next_char method return the character '>'
    assert_eq!(parser.next_char(), '>');
  }

  // Test the method starts_with of the TextParser struct implementation
  #[test]
  fn test_starts_with() {
    let mut text_parser: TextParser = TextParser::new(0, "<p>Hello World!</p>".to_string());

    // Assert that the starts_with method returns true because the input starts with the string "<"
    assert!(text_parser.starts_with("<"));
    // Assert that the starts_with method returns true because the input starts with the string "<p>"
    assert!(text_parser.starts_with("<p>"));
    // Assert that the starts_with method returns false because the input doesn't start with the string "Hello"
    assert!(!text_parser.starts_with("Hello"));
    // Assert that the starts_with method returns false because the input doesn't start with the string "!"
    assert!(!text_parser.starts_with("!"));

    text_parser.increment_position(3);
    // Assert that the starts_with method returns false because the input doesn't start with the string "<"
    assert!(!text_parser.starts_with("<"));
    // Assert that the starts_with method returns false because the input doesn't start with the string "<p>"
    assert!(!text_parser.starts_with("<p>"));
    // Assert that the starts_with method returns true because the input starts with the string "H"
    assert!(text_parser.starts_with("H"));
    // Assert that the starts_with method returns true because the input starts with the string "Hello"
    assert!(text_parser.starts_with("Hello"));
  }

  // Test the method eof of the TextParser struct implementation
  #[test]
  fn test_eof() {
    let parser: TextParser = TextParser::new(0, "<p>Hello World!</p>".to_string());

    // Assert that the eof method returns false because the current position is not at the end of the input string
    assert_eq!(text_parser.eof(), false);

    text_parser.increment_position(5);
    // Assert that the eof method returns false because the current position is not at the end of the input string
    assert_eq!(text_parser.eof(), false);

    text_parser.increment_position(19);
    // Assert that the eof method returns true because the current position is at the end of the input string
    assert_eq!(text_parser.eof(), true);
  }

  // Test the method consume_char of the TextParser struct implementation
  #[test]
  fn test_consume_char() {
    let mut parser: TextParser = TextParser::new(0, "<p>Hello World!</p>".to_string());

    // Assert that the consume_char method correctly consumes only the character '<'
    // and returns the consumed characters as a string
    assert_eq!(parser.consume_char(), '<');
    // Assert that the position is correctly updated to 1 after consuming the character
    assert_eq!(parser.position, 1);

    parser.position = 3;
    // Assert that the consume_char method correctly consumes only the character 'H'
    // and returns the consumed characters as a string
    assert_eq!(parser.consume_char(), 'H');
    // Assert that the position is correctly updated to 4 after consuming the character
    assert_eq!(parser.position, 4);

    parser.position = 8;
    // Assert that the consume_char method correctly consumes only the character ' '
    // and returns the consumed characters as a string
    assert_eq!(parser.consume_char(), ' ');
    // Assert that the position is correctly updated to 9 after consuming the character
    assert_eq!(parser.position, 9);
  }

  // Test the method consume_while of the TextParser struct implementation
  #[test]
  fn test_consume_while() {
    let mut parser: TextParser = TextParser::new(3, "<p>Hello World!</p>".to_string());

    // Assert that the consume_while method correctly consumes only the alphabetic characters "Hello"
    // and returns the consumed characters as a string
    assert_eq!(text_parser.consume_while(|c| c.is_alphabetic()), "Hello");
    // Assert that the position is correctly updated to 8 after consuming the characters
    assert_eq!(text_parser.position, 8);

    // Assert that the consume_while method correctly consumes only the whitespace character ' '
    // and returns the consumed character as a string
    assert_eq!(text_parser.consume_while(|c| c.is_whitespace()), " ");
    // Assert that the position is correctly updated to 9 after consuming the characters
    assert_eq!(text_parser.position, 9);

    // Assert that the consume_while method correctly returns an empty string when no digits are found
    // and returns an empty character as a string
    assert_eq!(text_parser.consume_while(|c| c.is_digit(10)), "");
    // Assert that the position is correctly updated to 9 after consuming the characters
    assert_eq!(text_parser.position, 9);
  }
}
