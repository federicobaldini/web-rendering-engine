use crate::style;
use super::{BoxType, Dimensions, LayoutBox};

impl<'a> LayoutBox<'a> {
  // Lay out a box and its descendants
  pub(super) fn layout(&mut self, containing_block: Dimensions) {
    match &self.box_type {
      BoxType::BlockNode(_) => self.layout_block(containing_block),
      BoxType::InlineNode(_) => self.layout_inline(containing_block),
      BoxType::AnonymousBlock => self.layout_anonymous_block(containing_block),
    }
  }
}

// Build the tree of LayoutBoxes, but don't perform any layout calculations yet
pub(super) fn build_layout_tree<'a>(style_node: &'a style::StyledNode<'a>) -> LayoutBox<'a> {
  // Create the root box
  let mut root: LayoutBox = LayoutBox::new(match style_node.display() {
    style::Display::Block => BoxType::BlockNode(style_node),
    style::Display::Inline => BoxType::InlineNode(style_node),
    style::Display::None => panic!("Root node has display: none."),
  });

  // Create the descendant boxes
  for child in style_node.children() {
    match child.display() {
      style::Display::Block => root.children.push(build_layout_tree(child)),
      style::Display::Inline => root
        .get_inline_container()
        .children
        .push(build_layout_tree(child)),
      style::Display::None => {} // Skip nodes with "display: none;"
    }
  }
  return root;
}

// Transform a style tree into a layout tree
pub fn layout_tree<'a>(
  node: &'a style::StyledNode<'a>,
  mut containing_block: Dimensions,
) -> LayoutBox<'a> {
  // The layout algorithm expects the container height to start at 0
  // TODO: Save the initial containing block height, for calculating percent heights
  containing_block.content.height = 0.0;

  let mut root_box: LayoutBox = build_layout_tree(node);
  root_box.layout(containing_block);
  root_box
}
