// CSS box model. All sizes are in px
/**
 * Features to add:
 * - Collapsing vertical margins;
 * - Relative positioning; (https://www.w3.org/TR/CSS2/visuren.html#relative-positioning)
 * - Parallelize the layout process, and measure the effect on performance;
 */
pub mod types;
pub mod block;
pub mod inline;
pub mod tree;
#[cfg(test)]
mod tests;

pub use types::{BoxType, Dimensions, EdgeSizes, LayoutBox, Rectangle};
pub use tree::layout_tree;
