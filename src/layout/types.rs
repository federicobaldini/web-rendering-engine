use crate::style;
use std::default::Default;

#[derive(Copy, Clone, Default, Debug)]
pub struct Rectangle {
  pub(super) x: f32,
  pub(super) y: f32,
  pub(super) width: f32,
  pub(super) height: f32,
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
  pub(super) top: f32,
  pub(super) right: f32,
  pub(super) bottom: f32,
  pub(super) left: f32,
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
  pub(super) content: Rectangle,
  // Surrounding edges:
  pub(super) padding: EdgeSizes,
  pub(super) border: EdgeSizes,
  pub(super) margin: EdgeSizes,
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
  pub fn new(content: Rectangle, padding: EdgeSizes, border: EdgeSizes, margin: EdgeSizes) -> Self {
    Self {
      content,
      padding,
      border,
      margin,
    }
  }

  pub fn content(&self) -> &Rectangle {
    &self.content
  }

  pub fn set_content(&mut self) -> &mut Rectangle {
    &mut self.content
  }

  pub fn padding(&self) -> &EdgeSizes {
    &self.padding
  }

  pub fn border(&self) -> &EdgeSizes {
    &self.border
  }

  pub fn margin(&self) -> &EdgeSizes {
    &self.margin
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
  InlineBlockNode(&'a style::StyledNode<'a>),
  AnonymousBlock,
}

impl<'a> PartialEq for BoxType<'a> {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (BoxType::BlockNode(a), BoxType::BlockNode(b)) => a == b,
      (BoxType::InlineNode(a), BoxType::InlineNode(b)) => a == b,
      (BoxType::InlineBlockNode(a), BoxType::InlineBlockNode(b)) => a == b,
      (BoxType::AnonymousBlock, BoxType::AnonymousBlock) => true,
      _ => false,
    }
  }
}

// A node in the layout tree
#[derive(Clone, Debug)]
pub struct LayoutBox<'a> {
  pub(super) dimensions: Dimensions,
  pub(super) box_type: BoxType<'a>,
  pub(super) children: Vec<LayoutBox<'a>>,
}

impl<'a> PartialEq for LayoutBox<'a> {
  fn eq(&self, other: &Self) -> bool {
    self.dimensions == other.dimensions
      && self.box_type == other.box_type
      && self.children == other.children
  }
}

impl<'a> LayoutBox<'a> {
  pub fn new(box_type: BoxType<'a>) -> Self {
    Self {
      box_type,
      dimensions: Default::default(),
      children: Vec::new(),
    }
  }

  pub fn dimensions(&self) -> &Dimensions {
    &self.dimensions
  }

  pub fn box_type(&self) -> &BoxType<'a> {
    &self.box_type
  }

  pub fn children(&self) -> &Vec<LayoutBox<'a>> {
    &self.children
  }

  pub fn add_child(&mut self, new_layout_box: LayoutBox<'a>) {
    self.children.push(new_layout_box);
  }

  pub(super) fn get_style_node(&self) -> &'a style::StyledNode<'a> {
    match &self.box_type {
      BoxType::BlockNode(node) | BoxType::InlineNode(node) | BoxType::InlineBlockNode(node) => node,
      BoxType::AnonymousBlock => panic!("Anonymous block box has no style node"),
    }
  }

  // Where a new inline child should go
  pub fn get_inline_container(&mut self) -> &mut LayoutBox<'a> {
    match self.box_type {
      BoxType::InlineNode(_) | BoxType::AnonymousBlock => self,
      BoxType::BlockNode(_) | BoxType::InlineBlockNode(_) => {
        // If we've just generated an anonymous block box, keep using it
        // Otherwise, create a new one
        match self.children.last() {
          Some(&LayoutBox {
            box_type: BoxType::AnonymousBlock,
            ..
          }) => {}
          _ => self.children.push(LayoutBox::new(BoxType::AnonymousBlock)),
        }
        self.children.last_mut().unwrap()
      }
    }
  }
}
