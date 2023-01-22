pub type Specificity = (usize, usize, usize);

pub struct SimpleSelector {
  tag_name: Option<String>,
  id: Option<String>,
  classes: Vec<String>,
}

impl SimpleSelector {
  pub fn new(tag_name: Option<String>, id: Option<String>, classes: Vec<String>) -> SimpleSelector {
    SimpleSelector {
      tag_name,
      id,
      classes,
    }
  }
  pub fn set_tag_name(&mut self, tag_name: Option<String>) {
    self.tag_name = tag_name;
  }
  pub fn set_id(&mut self, id: Option<String>) {
    self.id = id;
  }
  pub fn add_class(&mut self, class: String) {
    self.classes.push(class);
  }
}

pub enum Selector {
  Simple(SimpleSelector),
}

impl Selector {
  pub fn specificity(&self) -> Specificity {
    // http://www.w3.org/TR/selectors/#specificity
    let Selector::Simple(ref simple) = *self;
    let a = simple.id.iter().count();
    let b = simple.classes.len();
    let c = simple.tag_name.iter().count();
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

#[derive(Debug)]
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

impl Color {
  pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Color {
    Color {
      red,
      green,
      blue,
      alpha,
    }
  }
}

#[derive(Debug)]
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

pub struct Declaration {
  name: String,
  value: Value,
}

impl Declaration {
  pub fn new(name: String, value: Value) -> Declaration {
    Declaration { name, value }
  }
}

pub struct Rule {
  selectors: Vec<Selector>,
  declarations: Vec<Declaration>,
}

impl Rule {
  pub fn new(selectors: Vec<Selector>, declarations: Vec<Declaration>) -> Rule {
    Rule {
      selectors,
      declarations,
    }
  }
}

pub struct Stylesheet {
  rules: Vec<Rule>,
}

impl Stylesheet {
  pub fn new(rules: Vec<Rule>) -> Stylesheet {
    Stylesheet { rules }
  }
}
