// CSS box model. All sizes are in px
use crate::css;
use crate::style;
use crate::style::StyledNode;
use std::default::Default;

fn sum<I>(iter: I) -> f32
where
  I: Iterator<Item = f32>,
{
  iter.fold(0., |a: f32, b: f32| a + b)
}

#[derive(Copy, Clone, Default, Debug)]
pub struct Rectangle {
  x: f32,
  y: f32,
  width: f32,
  height: f32,
}

impl PartialEq for Rectangle {
  fn eq(&self, other: &Self) -> bool {
    self.x == other.x
      && self.y == other.y
      && self.width == other.width
      && self.height == other.height
  }
}

impl Rectangle {
  pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
    Self {
      x,
      y,
      width,
      height,
    }
  }

  pub fn expanded_by(self, edge: EdgeSizes) -> Self {
    Self {
      x: self.x - edge.left,
      y: self.y - edge.top,
      width: self.width + edge.left + edge.right,
      height: self.height + edge.top + edge.bottom,
    }
  }

  pub fn x(&self) -> f32 {
    self.x
  }

  pub fn y(&self) -> f32 {
    self.y
  }

  pub fn width(&self) -> f32 {
    self.width
  }

  pub fn set_width(&mut self, width: f32) {
    self.width = width;
  }

  pub fn height(&self) -> f32 {
    self.height
  }

  pub fn set_height(&mut self, height: f32) {
    self.height = height;
  }
}
