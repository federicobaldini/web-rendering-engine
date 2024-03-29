use std::fmt;

pub type Specificity = (usize, usize, usize);

#[derive(Clone, Debug)]
pub struct SimpleSelector {
  tag_name: Option<String>,
  id: Option<String>,
  classes: Vec<String>,
}

impl PartialEq for SimpleSelector {
  fn eq(&self, other: &Self) -> bool {
    self.tag_name == other.tag_name && self.id == other.id && self.classes == other.classes
  }
}

impl SimpleSelector {
  pub fn new(tag_name: Option<String>, id: Option<String>, classes: Vec<String>) -> Self {
    Self {
      tag_name,
      id,
      classes,
    }
  }

  pub fn tag_name(&self) -> &Option<String> {
    &self.tag_name
  }

  pub fn set_tag_name(&mut self, tag_name: Option<String>) {
    self.tag_name = tag_name;
  }

  pub fn id(&self) -> &Option<String> {
    &self.id
  }

  pub fn set_id(&mut self, id: Option<String>) {
    self.id = id;
  }

  pub fn classes(&self) -> &Vec<String> {
    &self.classes
  }

  pub fn add_class(&mut self, class: String) {
    self.classes.push(class);
  }
}

#[derive(Clone, Debug)]
pub enum Selector {
  Simple(SimpleSelector),
}

impl PartialEq for Selector {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Selector::Simple(a), Selector::Simple(b)) => a == b,
    }
  }
}

impl Selector {
  // Specificity is one of the ways a rendering engine decides which style overrides the other in a conflict
  pub fn specificity(&self) -> Specificity {
    // http://www.w3.org/TR/selectors/#specificity
    let Selector::Simple(ref simple) = *self;
    let a: usize = simple.id.iter().count();
    let b: usize = simple.classes.len();
    let c: usize = simple.tag_name.iter().count();
    (a, b, c)
  }
}

#[derive(Copy, Clone, Debug)]
pub enum Unit {
  Px,
  // insert more units here
}

impl PartialEq for Unit {
  fn eq(&self, other: &Self) -> bool {
    *self as u8 == *other as u8
  }
}

impl fmt::Display for Unit {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Unit::Px => write!(f, "px"),
      // handle more variants here
    }
  }
}

#[derive(Copy, Clone, Debug)]
pub struct Color {
  red: u8,
  green: u8,
  blue: u8,
  alpha: u8,
}

impl PartialEq for Color {
  fn eq(&self, other: &Self) -> bool {
    self.red == other.red
      && self.green == other.green
      && self.blue == other.blue
      && self.alpha == other.alpha
  }
}

impl fmt::Display for Color {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "rgba({},{},{},{})",
      self.red, self.green, self.blue, self.alpha
    )
  }
}

impl Color {
  pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
    Self {
      red,
      green,
      blue,
      alpha,
    }
  }

  pub fn red(&self) -> u8 {
    self.red
  }

  pub fn green(&self) -> u8 {
    self.green
  }

  pub fn blue(&self) -> u8 {
    self.blue
  }

  pub fn alpha(&self) -> u8 {
    self.alpha
  }
}

#[derive(Clone, Debug)]
pub enum Value {
  Keyword(String),
  Length(f32, Unit),
  ColorValue(Color),
  // insert more values here
}

impl PartialEq for Value {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Value::Keyword(a), Value::Keyword(b)) => a == b,
      (Value::Length(a, b), Value::Length(c, d)) => a == c && b == d,
      (Value::ColorValue(a), Value::ColorValue(b)) => a == b,
      _ => false,
    }
  }
}

impl fmt::Display for Value {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Value::Keyword(s) => write!(f, "{}", s),
      Value::Length(value, unit) => write!(f, "{}{}", value, unit),
      Value::ColorValue(color) => write!(f, "{}", color),
      // handle more variants here
    }
  }
}

impl Value {
  // Return the size of a length in px, or zero for non-lengths.
  pub fn to_px(&self) -> f32 {
    match *self {
      Value::Length(f, Unit::Px) => f,
      _ => 0.0,
    }
  }
}

#[derive(Clone, Debug)]
pub struct Declaration {
  name: String,
  value: Value,
}

impl PartialEq for Declaration {
  fn eq(&self, other: &Self) -> bool {
    self.name == other.name && self.value == other.value
  }
}

impl Declaration {
  pub fn new(name: String, value: Value) -> Self {
    Self { name, value }
  }

  pub fn name(&self) -> &str {
    &&self.name
  }

  pub fn value(&self) -> &Value {
    &self.value
  }
}

#[derive(Clone, Debug)]
pub struct Rule {
  selectors: Vec<Selector>,
  declarations: Vec<Declaration>,
}

impl PartialEq for Rule {
  fn eq(&self, other: &Self) -> bool {
    self.selectors == other.selectors && self.declarations == other.declarations
  }
}

impl Rule {
  pub fn new(selectors: Vec<Selector>, declarations: Vec<Declaration>) -> Self {
    Self {
      selectors,
      declarations,
    }
  }

  pub fn selectors(&self) -> &Vec<Selector> {
    &self.selectors
  }

  pub fn declarations(&self) -> &Vec<Declaration> {
    &self.declarations
  }
}

#[derive(Debug)]
pub struct Stylesheet {
  rules: Vec<Rule>,
}

impl PartialEq for Stylesheet {
  fn eq(&self, other: &Self) -> bool {
    self.rules == other.rules
  }
}

impl Stylesheet {
  pub fn new(rules: Vec<Rule>) -> Self {
    Self { rules }
  }

  pub fn rules(&self) -> &Vec<Rule> {
    &self.rules
  }
}
