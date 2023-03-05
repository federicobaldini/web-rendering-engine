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

#[derive(Copy, Clone, Default, Debug)]
pub struct EdgeSizes {
  top: f32,
  right: f32,
  bottom: f32,
  left: f32,
}

impl PartialEq for EdgeSizes {
  fn eq(&self, other: &Self) -> bool {
    self.left == other.left
      && self.right == other.right
      && self.top == other.top
      && self.bottom == other.bottom
  }
}

impl EdgeSizes {
  pub fn new(top: f32, right: f32, bottom: f32, left: f32) -> Self {
    Self {
      top,
      right,
      bottom,
      left,
    }
  }

  pub fn top(&self) -> f32 {
    self.top
  }

  pub fn right(&self) -> f32 {
    self.right
  }

  pub fn bottom(&self) -> f32 {
    self.bottom
  }

  pub fn left(&self) -> f32 {
    self.left
  }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct Dimensions {
  // Position of the content area relative to the document origin:
  content: Rectangle,
  // Surrounding edges:
  padding: EdgeSizes,
  border: EdgeSizes,
  margin: EdgeSizes,
}

impl PartialEq for Dimensions {
  fn eq(&self, other: &Self) -> bool {
    self.content == other.content
      && self.padding == other.padding
      && self.border == other.border
      && self.margin == other.margin
  }
}

impl Dimensions {
  pub fn content(&self) -> &Rectangle {
    &self.content
  }

  pub fn set_content(&mut self) -> &mut Rectangle {
    &mut self.content
  }

  pub fn border(&self) -> &EdgeSizes {
    &self.border
  }

  // The area covered by the content area plus its padding
  pub fn padding_box(self) -> Rectangle {
    self.content.expanded_by(self.padding)
  }

  // The area covered by the content area plus padding and borders
  pub fn border_box(self) -> Rectangle {
    self.padding_box().expanded_by(self.border)
  }

  // The area covered by the content area plus padding, borders, and margin
  pub fn margin_box(self) -> Rectangle {
    self.border_box().expanded_by(self.margin)
  }
}

#[derive(Clone, Debug)]
pub enum BoxType<'a> {
  BlockNode(&'a style::StyledNode<'a>),
  InlineNode(&'a style::StyledNode<'a>),
  AnonymousBlock,
}

impl<'a> PartialEq for BoxType<'a> {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (BoxType::BlockNode(a), BoxType::BlockNode(b)) => a == b,
      (BoxType::InlineNode(a), BoxType::InlineNode(b)) => a == b,
      (BoxType::AnonymousBlock, BoxType::AnonymousBlock) => true,
      _ => false,
    }
  }
}
