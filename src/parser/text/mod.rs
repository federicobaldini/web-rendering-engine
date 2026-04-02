use std::str::CharIndices;

#[cfg(test)]
mod tests;

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

  // Consume the next character and return it, or an error if it doesn't match expected
  pub fn expect_char(&mut self, expected: char) -> Result<char, String> {
    let c: char = self.consume_char();
    if c == expected {
      Ok(c)
    } else {
      Err(format!("Expected '{}', found '{}'", expected, c))
    }
  }
}
