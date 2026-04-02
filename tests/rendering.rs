use web_rendering_engine::css;
use web_rendering_engine::parser::css::CSSParser;
use web_rendering_engine::dom;
use web_rendering_engine::parser::html::HTMLParser;
use web_rendering_engine::layout;
use web_rendering_engine::painting;
use web_rendering_engine::style;

// Test the full rendering pipeline using the example files:
// examples/test.html and examples/test.css
//
// Expected layout (viewport 800x600):
//
//  container-1 (.container-1)
//    padding: 32px → content at x=32, y=32, width=736
//    height = sum of children margin boxes = 136 + 156 = 292
//
//    container-2 (.container-2)
//      width: 120px, height: 80px, padding: 16px, margin: 8px, border-width: 4px
//      content at x=60, y=60
//      margin_box height = 80 + 32(padding) + 8(border) + 16(margin) = 136
//
//    container-3 (.container-3)
//      width: 60px, height: 100px, padding: 16px, margin: 8px, border-width: 4px
//      content at x=60, y=196  (136 + 32 + 8 + 4 + 16)
//      margin_box height = 100 + 32 + 8 + 16 = 156
#[test]
fn test_full_rendering_pipeline() {
  // Read example files from disk
  let html: String = std::fs::read_to_string("examples/test.html")
    .expect("Failed to read examples/test.html");
  let css: String = std::fs::read_to_string("examples/test.css")
    .expect("Failed to read examples/test.css");

  // Parsing
  let root_node: dom::Node = HTMLParser::parse(html).expect("HTML parse error");
  let stylesheet: css::Stylesheet = CSSParser::parse(css).expect("CSS parse error");

  // Style tree
  let style_root: style::StyledNode = style::style_tree(&root_node, &stylesheet);

  // Layout
  let mut viewport: layout::Dimensions = Default::default();
  viewport.set_content().set_width(800.0);
  viewport.set_content().set_height(600.0);
  let layout_root: layout::LayoutBox = layout::layout_tree(&style_root, viewport);

  // --- container-1 (root) ---
  // No border or margin; padding 32px all sides; width auto → fills 800px viewport
  assert_eq!(layout_root.dimensions().content().x(), 32.0);
  assert_eq!(layout_root.dimensions().content().y(), 32.0);
  assert_eq!(layout_root.dimensions().content().width(), 736.0);
  assert_eq!(layout_root.dimensions().content().height(), 292.0);
  assert_eq!(layout_root.dimensions().padding().top(), 32.0);
  assert_eq!(layout_root.dimensions().padding().left(), 32.0);
  assert_eq!(layout_root.dimensions().border().top(), 0.0);
  assert_eq!(layout_root.dimensions().margin().top(), 0.0);
  assert_eq!(layout_root.children().len(), 2);

  // --- container-2 (first child) ---
  let container_2: &layout::LayoutBox = &layout_root.children()[0];
  assert_eq!(container_2.dimensions().content().x(), 60.0);
  assert_eq!(container_2.dimensions().content().y(), 60.0);
  assert_eq!(container_2.dimensions().content().width(), 120.0);
  assert_eq!(container_2.dimensions().content().height(), 80.0);
  assert_eq!(container_2.dimensions().padding().top(), 16.0);
  assert_eq!(container_2.dimensions().border().top(), 4.0);
  assert_eq!(container_2.dimensions().margin().top(), 8.0);
  assert_eq!(container_2.dimensions().margin().left(), 8.0);

  // --- container-3 (second child) ---
  let container_3: &layout::LayoutBox = &layout_root.children()[1];
  assert_eq!(container_3.dimensions().content().x(), 60.0);
  assert_eq!(container_3.dimensions().content().y(), 196.0);
  assert_eq!(container_3.dimensions().content().width(), 60.0);
  assert_eq!(container_3.dimensions().content().height(), 100.0);
  assert_eq!(container_3.dimensions().padding().top(), 16.0);
  assert_eq!(container_3.dimensions().border().top(), 4.0);
  assert_eq!(container_3.dimensions().margin().top(), 8.0);

  // --- Canvas ---
  // Paint to an 800x600 canvas and verify pixel colors at key positions.
  // Pixels are stored as pixels[x + y * width].
  let canvas: painting::Canvas = painting::Canvas::paint(&layout_root, *viewport.content());

  // (0, 0): container-1 background #ec7063 = (236, 112, 99)
  // Its border_box covers the full {x:0, y:0, w:800, h:356} area.
  let pixel_c1: css::Color = canvas.pixels()[0 + 0 * 800];
  assert_eq!(pixel_c1.red(), 236);
  assert_eq!(pixel_c1.green(), 112);
  assert_eq!(pixel_c1.blue(), 99);

  // (50, 50): inside container-2 background #58d68d = (88, 214, 141)
  // Its border_box is {x:40, y:40, w:160, h:120}; (50,50) is past the 4px borders.
  let pixel_c2: css::Color = canvas.pixels()[50 + 50 * 800];
  assert_eq!(pixel_c2.red(), 88);
  assert_eq!(pixel_c2.green(), 214);
  assert_eq!(pixel_c2.blue(), 141);

  // (50, 190): inside container-3 background #5dade2 = (93, 173, 226)
  // Its border_box is {x:40, y:176, w:100, h:140}; (50,190) is past the 4px borders.
  let pixel_c3: css::Color = canvas.pixels()[50 + 190 * 800];
  assert_eq!(pixel_c3.red(), 93);
  assert_eq!(pixel_c3.green(), 173);
  assert_eq!(pixel_c3.blue(), 226);

  // (400, 400): below container-1 border_box (which ends at y=356) → white canvas
  let pixel_white: css::Color = canvas.pixels()[400 + 400 * 800];
  assert_eq!(pixel_white.red(), 255);
  assert_eq!(pixel_white.green(), 255);
  assert_eq!(pixel_white.blue(), 255);
}
