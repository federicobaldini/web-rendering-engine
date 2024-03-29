use std::str::CharIndices;

pub struct TextParser {
  position: usize,
  input: String,
}

impl TextParser {
  pub fn new(position: usize, input: String) -> Self {
    Self { position, input }
  }

  pub fn position(&self) -> usize {
    self.position
  }

  pub fn increment_position(&mut self, value: usize) {
    self.position += value;
  }

  pub fn input(&self) -> &str {
    &self.input
  }

  // Read the current character without consuming it
  pub fn next_char(&self) -> char {
    self.input[self.position..].chars().next().unwrap()
  }

  // Read the character from the current position to a given offset without consuming it
  pub fn next_offset_char(&self, offset: usize) -> char {
    self.input[self.position..].chars().nth(offset).unwrap()
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

  // Consume characters until 'test' returns false
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

  pub fn consume_until_match(&mut self, target: &str) -> String {
    let mut result: String = String::new();
    let mut target_found: bool = false;
    let mut potential_match: &str = "";
    while !self.eof() && !target_found {
      potential_match = &self.input[self.position..self.position + target.len()];
      if potential_match.starts_with(target) {
        target_found = true;
      } else {
        result.push(self.consume_char());
      }
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
      let mut text_parser: TextParser = TextParser::new(0, "<p>Hello World!</p>".to_string());
  
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
}
