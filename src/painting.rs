use ab_glyph::{Font, PxScale, ScaleFont};

use crate::css;
use crate::dom;
use crate::layout;

type DisplayList = Vec<DisplayCommand>;

#[derive(Debug)]
enum DisplayCommand {
  SolidColor(css::Color, layout::Rectangle),
  // color, content bounds, text string, font size in px
  DrawText(css::Color, layout::Rectangle, String, f32),
}

impl PartialEq for DisplayCommand {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (DisplayCommand::SolidColor(a, b), DisplayCommand::SolidColor(c, d)) => a == c && b == d,
      (DisplayCommand::DrawText(a, b, c, d), DisplayCommand::DrawText(e, f, g, h)) => {
        a == e && b == f && c == g && d == h
      }
      _ => false,
    }
  }
}

// Return the specified color for CSS property "name", or None if no color was specified.
fn get_color(layout_box: &layout::LayoutBox, name: &str) -> Option<css::Color> {
  match layout_box.box_type() {
    layout::BoxType::BlockNode(style) | layout::BoxType::InlineNode(style) => {
      match style.value(name) {
        Some(css::Value::ColorValue(color)) => Some(color),
        _ => None,
      }
    }
    layout::BoxType::AnonymousBlock => None,
  }
}

fn render_background(list: &mut DisplayList, layout_box: &layout::LayoutBox) {
  get_color(layout_box, "background").map(|color: css::Color| {
    list.push(DisplayCommand::SolidColor(
      color,
      layout_box.dimensions().border_box(),
    ))
  });
}

fn render_borders(list: &mut DisplayList, layout_box: &layout::LayoutBox) {
  let color: css::Color = match get_color(layout_box, "border-color") {
    Some(color) => color,
    _ => return, // fail out if no border-color is specified
  };

  let dimensions: &layout::Dimensions = layout_box.dimensions();
  let border_box: layout::Rectangle = dimensions.border_box();

  // Top border
  list.push(DisplayCommand::SolidColor(
    color,
    layout::Rectangle::new(
      border_box.x(),
      border_box.y(),
      border_box.width(),
      dimensions.border().top(),
    ),
  ));

  // Right border
  list.push(DisplayCommand::SolidColor(
    color,
    layout::Rectangle::new(
      border_box.x() + border_box.width() - dimensions.border().right(),
      border_box.y(),
      dimensions.border().right(),
      border_box.height(),
    ),
  ));

  // Bottom border
  list.push(DisplayCommand::SolidColor(
    color,
    layout::Rectangle::new(
      border_box.x(),
      border_box.y() + border_box.height() - dimensions.border().bottom(),
      border_box.width(),
      dimensions.border().bottom(),
    ),
  ));

  // Left border
  list.push(DisplayCommand::SolidColor(
    color,
    layout::Rectangle::new(
      border_box.x(),
      border_box.y(),
      dimensions.border().left(),
      border_box.height(),
    ),
  ));
}

// If this layout box is an inline node wrapping a DOM text node, add a DrawText command.
fn render_text(list: &mut DisplayList, layout_box: &layout::LayoutBox) {
  if let layout::BoxType::InlineNode(style) = layout_box.box_type() {
    if let dom::NodeType::Text(text) = style.node().node_type() {
      // Skip whitespace-only text nodes produced by HTML indentation
      if text.trim().is_empty() {
        return;
      }
      // Text nodes have no specified values (inheritance is not yet implemented),
      // so default to black text at 16px.
      let color: css::Color = css::Color::new(0, 0, 0, 255);
      let font_size: f32 = style
        .value("font-size")
        .map(|v: css::Value| v.to_px())
        .unwrap_or(16.0);
      list.push(DisplayCommand::DrawText(
        color,
        *layout_box.dimensions().content(),
        text.clone(),
        font_size,
      ));
    }
  }
}

fn render_layout_box(list: &mut DisplayList, layout_box: &layout::LayoutBox) {
  render_background(list, layout_box);
  render_borders(list, layout_box);
  render_text(list, layout_box);

  for child in layout_box.children() {
    render_layout_box(list, child);
  }
}

fn build_display_list(layout_root: &layout::LayoutBox) -> DisplayList {
  let mut list: Vec<DisplayCommand> = Vec::new();
  render_layout_box(&mut list, layout_root);
  return list;
}

trait Clamp {
  fn clamp(self, lower: Self, upper: Self) -> Self;
}

impl Clamp for f32 {
  fn clamp(self, lower: f32, upper: f32) -> f32 {
    self.max(lower).min(upper)
  }
}

pub struct Canvas {
  pixels: Vec<css::Color>,
  width: usize,
  height: usize,
  // Optional font used for text rendering; None if the font file could not be loaded.
  font: Option<ab_glyph::FontVec>,
}

impl Canvas {
  // Create a blank canvas
  fn new(width: usize, height: usize) -> Canvas {
    let white = css::Color::new(255, 255, 255, 255);
    return Canvas {
      pixels: vec![white; width * height],
      width,
      height,
      font: None,
    };
  }

  pub fn pixels(&self) -> &Vec<css::Color> {
    &self.pixels
  }

  pub fn width(&self) -> usize {
    self.width
  }

  pub fn height(&self) -> usize {
    self.height
  }

  fn paint_item(&mut self, item: &DisplayCommand) {
    match item {
      DisplayCommand::SolidColor(color, rectangle) => {
        // Clip the rectangle to the canvas boundaries.
        let x0: usize = rectangle.x().clamp(0.0, self.width as f32) as usize;
        let y0: usize = rectangle.y().clamp(0.0, self.height as f32) as usize;
        let x1: usize = (rectangle.x() + rectangle.width()).clamp(0.0, self.width as f32) as usize;
        let y1: usize =
          (rectangle.y() + rectangle.height()).clamp(0.0, self.height as f32) as usize;

        for y in y0..y1 {
          for x in x0..x1 {
            // TODO: alpha compositing with existing pixel
            self.pixels[x + y * self.width] = *color;
          }
        }
      }
      DisplayCommand::DrawText(color, rect, text, font_size) => {
        if let Some(ref font) = self.font {
          let scale: PxScale = PxScale::from(*font_size);
          let scaled_font = font.as_scaled(scale);
          // The caret y is the baseline: top of content box plus the font ascent.
          let ascent: f32 = scaled_font.ascent();
          let mut caret_x: f32 = rect.x();
          let caret_y: f32 = rect.y() + ascent;
          let mut prev_glyph_id: Option<ab_glyph::GlyphId> = None;

          for ch in text.chars() {
            if ch.is_control() {
              continue;
            }
            let glyph_id: ab_glyph::GlyphId = scaled_font.glyph_id(ch);
            // Apply kerning between the previous and current glyph.
            if let Some(prev_id) = prev_glyph_id {
              caret_x += scaled_font.kern(prev_id, glyph_id);
            }
            let glyph: ab_glyph::Glyph = glyph_id.with_scale_and_position(
              scale,
              ab_glyph::point(caret_x, caret_y),
            );
            caret_x += scaled_font.h_advance(glyph_id);
            prev_glyph_id = Some(glyph_id);

            if let Some(outlined) = font.outline_glyph(glyph) {
              let px_bounds: ab_glyph::Rect = outlined.px_bounds();
              // px_bounds.min is the top-left corner in canvas pixel coordinates.
              let box_x: i32 = px_bounds.min.x as i32;
              let box_y: i32 = px_bounds.min.y as i32;
              outlined.draw(|px, py, coverage| {
                // px, py are pixel offsets within the glyph bitmap.
                let canvas_x: i32 = box_x + px as i32;
                let canvas_y: i32 = box_y + py as i32;
                if canvas_x >= 0
                  && canvas_y >= 0
                  && (canvas_x as usize) < self.width
                  && (canvas_y as usize) < self.height
                {
                  let idx: usize = canvas_x as usize + canvas_y as usize * self.width;
                  // Alpha-blend the glyph color over the existing background pixel.
                  let existing: css::Color = self.pixels[idx];
                  let inv: f32 = 1.0 - coverage;
                  let r: u8 = (color.red() as f32 * coverage + existing.red() as f32 * inv) as u8;
                  let g: u8 =
                    (color.green() as f32 * coverage + existing.green() as f32 * inv) as u8;
                  let b: u8 =
                    (color.blue() as f32 * coverage + existing.blue() as f32 * inv) as u8;
                  self.pixels[idx] = css::Color::new(r, g, b, 255);
                }
              });
            }
          }
        }
      }
    }
  }

  // Paint a tree of LayoutBoxes to an array of pixels.
  pub fn paint(layout_root: &layout::LayoutBox, bounds: layout::Rectangle) -> Canvas {
    let display_list: Vec<DisplayCommand> = build_display_list(layout_root);
    let mut canvas: Canvas = Canvas::new(bounds.width() as usize, bounds.height() as usize);
    // Try to load a system font for text rendering. If the file is missing, text is silently skipped.
    let font_path: &str = "/System/Library/Fonts/Supplemental/Arial.ttf";
    canvas.font = std::fs::read(font_path)
      .ok()
      .and_then(|bytes: Vec<u8>| ab_glyph::FontVec::try_from_vec(bytes).ok());
    for item in display_list {
      canvas.paint_item(&item);
    }
    canvas
  }
}

#[cfg(test)]
mod tests {
  use crate::dom;
  use crate::hashmap;
  use crate::layout;
  use crate::painting::*;
  use crate::style;

  // Test the function get_color
  #[test]
  fn test_get_color() {
    // Node: <div class='container-1'>
    let tag_name: String = String::from("div");
    let attributes: dom::AttributeMap =
      hashmap![String::from("class") => String::from("container-1")];
    let node: dom::Node = dom::Node::element(tag_name, attributes, vec![]);
    // Selector
    let simple_selector: css::SimpleSelector =
      css::SimpleSelector::new(None, None, vec!["container-1".to_string()]);
    let selector: css::Selector = css::Selector::Simple(simple_selector);
    // Declaration
    let background_unit: css::Value = css::Value::ColorValue(css::Color::new(255, 0, 0, 255));
    let background_declaration: css::Declaration =
      css::Declaration::new("background".to_string(), background_unit);
    // Rule
    let rule: css::Rule = css::Rule::new(vec![selector], vec![background_declaration]);
    // Stylesheet
    let stylesheet: css::Stylesheet = css::Stylesheet::new(vec![rule.clone()]);
    // Value
    let mut values: style::PropertyMap = hashmap![];

    match node.node_type() {
      dom::NodeType::Element(element) => {
        values = style::specified_values(&element, &stylesheet);
      }
      _ => {}
    }
    // StyleNode
    let style_node: style::StyledNode = style::StyledNode::new(&node, values, vec![]);
    // LayoutBox
    let layout_box: layout::LayoutBox =
      layout::LayoutBox::new(layout::BoxType::BlockNode(&style_node));

    assert_eq!(
      get_color(&layout_box, "background"),
      Some(css::Color::new(255, 0, 0, 255))
    );
  }

  // Test the function render_background
  #[test]
  fn test_render_background() {
    // Node: <div class='container-1'>
    let tag_name: String = String::from("div");
    let attributes: dom::AttributeMap =
      hashmap![String::from("class") => String::from("container-1")];
    let node: dom::Node = dom::Node::element(tag_name, attributes, vec![]);
    // Selector
    let simple_selector: css::SimpleSelector =
      css::SimpleSelector::new(None, None, vec!["container-1".to_string()]);
    let selector: css::Selector = css::Selector::Simple(simple_selector);
    // Declaration
    let background_unit: css::Value = css::Value::ColorValue(css::Color::new(255, 0, 0, 255));
    let background_declaration: css::Declaration =
      css::Declaration::new("background".to_string(), background_unit);
    // Rule
    let rule: css::Rule = css::Rule::new(vec![selector], vec![background_declaration]);
    // Stylesheet
    let stylesheet: css::Stylesheet = css::Stylesheet::new(vec![rule.clone()]);
    // Value
    let mut values: style::PropertyMap = hashmap![];

    match node.node_type() {
      dom::NodeType::Element(element) => {
        values = style::specified_values(&element, &stylesheet);
      }
      _ => {}
    }
    // StyleNode
    let style_node: style::StyledNode = style::StyledNode::new(&node, values, vec![]);
    // LayoutBox
    let layout_box: layout::LayoutBox =
      layout::LayoutBox::new(layout::BoxType::BlockNode(&style_node));
    // Display list
    let mut display_list: DisplayList = vec![];

    // Assert that the render_background function correctly add the layout_box background and border box to the display_list
    render_background(&mut display_list, &layout_box);

    // Assert that the first element of the display list is a SolidColor DisplayCommand with the right color and border box
    assert_eq!(
      display_list[0],
      DisplayCommand::SolidColor(
        css::Color::new(255, 0, 0, 255),
        layout_box.dimensions().border_box()
      )
    );
  }

  // Test the function render_borders
  #[test]
  fn test_render_borders() {
    // Node: <div class='container-1'>
    let tag_name: String = String::from("div");
    let attributes: dom::AttributeMap =
      hashmap![String::from("class") => String::from("container-1")];
    let node: dom::Node = dom::Node::element(tag_name, attributes, vec![]);
    // Selector
    let simple_selector: css::SimpleSelector =
      css::SimpleSelector::new(None, None, vec!["container-1".to_string()]);
    let selector: css::Selector = css::Selector::Simple(simple_selector);
    // Declarations
    let border_color_unit: css::Value = css::Value::ColorValue(css::Color::new(255, 0, 0, 255));
    let border_color_declaration: css::Declaration =
      css::Declaration::new("border-color".to_string(), border_color_unit);

    let border_top_width_unit: css::Value = css::Value::Length(1.0, css::Unit::Px);
    let border_top_width_declaration: css::Declaration =
      css::Declaration::new("border-top-width".to_string(), border_top_width_unit);

    let border_right_width_unit: css::Value = css::Value::Length(1.0, css::Unit::Px);
    let border_right_width_declaration: css::Declaration =
      css::Declaration::new("border-right-width".to_string(), border_right_width_unit);

    let border_bottom_width_unit: css::Value = css::Value::Length(1.0, css::Unit::Px);
    let border_bottom_width_declaration: css::Declaration =
      css::Declaration::new("border-bottom-width".to_string(), border_bottom_width_unit);

    let border_left_width_unit: css::Value = css::Value::Length(1.0, css::Unit::Px);
    let border_left_width_declaration: css::Declaration =
      css::Declaration::new("border-left-width".to_string(), border_left_width_unit);
    // Rule
    let rule: css::Rule = css::Rule::new(
      vec![selector],
      vec![
        border_color_declaration,
        border_top_width_declaration,
        border_right_width_declaration,
        border_bottom_width_declaration,
        border_left_width_declaration,
      ],
    );
    // Stylesheet
    let stylesheet: css::Stylesheet = css::Stylesheet::new(vec![rule.clone()]);
    // Value
    let mut values: style::PropertyMap = hashmap![];

    match node.node_type() {
      dom::NodeType::Element(element) => {
        values = style::specified_values(&element, &stylesheet);
      }
      _ => {}
    }
    // StyleNode
    let style_node: style::StyledNode = style::StyledNode::new(&node, values, vec![]);
    // LayoutBox
    let layout_box: layout::LayoutBox =
      layout::LayoutBox::new(layout::BoxType::BlockNode(&style_node));
    // Display list
    let mut display_list: DisplayList = vec![];

    // Assert that the render_borders function correctly add the layout_box borders
    render_borders(&mut display_list, &layout_box);

    // Assert that the first element of the display list is a SolidColor DisplayCommand with the right color and top border data
    assert_eq!(
      display_list[0],
      DisplayCommand::SolidColor(
        css::Color::new(255, 0, 0, 255),
        layout::Rectangle::new(
          layout_box.dimensions().border_box().x(),
          layout_box.dimensions().border_box().y(),
          layout_box.dimensions().border_box().width(),
          layout_box.dimensions().border().top()
        )
      )
    );
    // Assert that the first element of the display list is a SolidColor DisplayCommand with the right color and right border data
    assert_eq!(
      display_list[1],
      DisplayCommand::SolidColor(
        css::Color::new(255, 0, 0, 255),
        layout::Rectangle::new(
          layout_box.dimensions().border_box().x()
            + layout_box.dimensions().border_box().width()
            - layout_box.dimensions().border().right(),
          layout_box.dimensions().border_box().y(),
          layout_box.dimensions().border().right(),
          layout_box.dimensions().border_box().height()
        )
      )
    );
    // Assert that the first element of the display list is a SolidColor DisplayCommand with the right color and bottom border data
    assert_eq!(
      display_list[2],
      DisplayCommand::SolidColor(
        css::Color::new(255, 0, 0, 255),
        layout::Rectangle::new(
          layout_box.dimensions().border_box().x(),
          layout_box.dimensions().border_box().y() + layout_box.dimensions().border_box().height()
            - layout_box.dimensions().border().bottom(),
          layout_box.dimensions().border_box().width(),
          layout_box.dimensions().border().bottom()
        )
      )
    );
    // Assert that the first element of the display list is a SolidColor DisplayCommand with the right color and left border data
    assert_eq!(
      display_list[3],
      DisplayCommand::SolidColor(
        css::Color::new(255, 0, 0, 255),
        layout::Rectangle::new(
          layout_box.dimensions().border_box().x(),
          layout_box.dimensions().border_box().y(),
          layout_box.dimensions().border().left(),
          layout_box.dimensions().border_box().height()
        )
      )
    );
  }

  // Test that render_text adds a DrawText command for an inline text node.
  #[test]
  fn test_render_text() {
    let text_node: dom::Node = dom::Node::text("Hello".to_string());
    let style_node: style::StyledNode = style::StyledNode::new(&text_node, hashmap![], vec![]);
    let layout_box: layout::LayoutBox =
      layout::LayoutBox::new(layout::BoxType::InlineNode(&style_node));
    let mut display_list: DisplayList = vec![];

    render_text(&mut display_list, &layout_box);

    assert_eq!(display_list.len(), 1);
    assert_eq!(
      display_list[0],
      DisplayCommand::DrawText(
        css::Color::new(0, 0, 0, 255),
        *layout_box.dimensions().content(),
        "Hello".to_string(),
        16.0,
      )
    );
  }

  // Test that render_text skips whitespace-only text nodes.
  #[test]
  fn test_render_text_skips_whitespace() {
    let text_node: dom::Node = dom::Node::text("   \n  ".to_string());
    let style_node: style::StyledNode = style::StyledNode::new(&text_node, hashmap![], vec![]);
    let layout_box: layout::LayoutBox =
      layout::LayoutBox::new(layout::BoxType::InlineNode(&style_node));
    let mut display_list: DisplayList = vec![];

    render_text(&mut display_list, &layout_box);

    assert_eq!(display_list.len(), 0);
  }

  // Test the method paint_item of the Canvas struct implementation
  #[test]
  fn test_paint_item() {
    // Node: <div class='container-1'>
    let tag_name: String = String::from("div");
    let attributes: dom::AttributeMap =
      hashmap![String::from("class") => String::from("container-1")];
    let node: dom::Node = dom::Node::element(tag_name, attributes, vec![]);
    // Selector
    let simple_selector: css::SimpleSelector =
      css::SimpleSelector::new(None, None, vec!["container-1".to_string()]);
    let selector: css::Selector = css::Selector::Simple(simple_selector);
    // Declarations
    let display_declaraion: css::Declaration = css::Declaration::new(
      "display".to_string(),
      css::Value::Keyword("block".to_string()),
    );
    let background_unit: css::Value = css::Value::ColorValue(css::Color::new(255, 0, 0, 255));
    let background_declaration: css::Declaration =
      css::Declaration::new("background".to_string(), background_unit);
    let width_unit: css::Value = css::Value::Length(50.0, css::Unit::Px);
    let width_declaration: css::Declaration =
      css::Declaration::new("width".to_string(), width_unit);
    let height_unit: css::Value = css::Value::Length(50.0, css::Unit::Px);
    let height_declaration: css::Declaration =
      css::Declaration::new("height".to_string(), height_unit);
    // Rule
    let rule: css::Rule = css::Rule::new(
      vec![selector],
      vec![
        display_declaraion,
        width_declaration,
        height_declaration,
        background_declaration,
      ],
    );
    // Stylesheet
    let stylesheet: css::Stylesheet = css::Stylesheet::new(vec![rule.clone()]);
    // Value
    let mut values: style::PropertyMap = hashmap![];

    match node.node_type() {
      dom::NodeType::Element(element) => {
        values = style::specified_values(&element, &stylesheet);
      }
      _ => {}
    }
    // StyleNode
    let style_node: style::StyledNode = style::StyledNode::new(&node, values, vec![]);
    // Containing block
    let content: layout::Rectangle = layout::Rectangle::new(0.0, 0.0, 100.0, 100.0);
    let padding: layout::EdgeSizes = layout::EdgeSizes::new(0.0, 0.0, 0.0, 0.0);
    let border: layout::EdgeSizes = layout::EdgeSizes::new(0.0, 0.0, 0.0, 0.0);
    let margin: layout::EdgeSizes = layout::EdgeSizes::new(0.0, 0.0, 0.0, 0.0);
    let containing_block: layout::Dimensions =
      layout::Dimensions::new(content, padding, border, margin);
    // LayoutBox
    let mut layout_box: layout::LayoutBox =
      layout::LayoutBox::new(layout::BoxType::BlockNode(&style_node));
    layout_box.layout_block(containing_block);
    // Display list
    let mut display_list: DisplayList = vec![];
    render_background(&mut display_list, &layout_box);
    // Canvas
    let mut canvas: Canvas = Canvas::new(200, 200);

    canvas.paint_item(&display_list[0]);

    let mut layout_box_pixels_count: i32 = 0;
    let mut layout_box_pixels_right_position: bool = true;

    for pixel in canvas.pixels() {
      if *pixel == css::Color::new(255, 0, 0, 255) {
        layout_box_pixels_count += 1;
      }
    }

    for row in (20000..30000).step_by(200) {
      for column in 0..49 {
        if canvas.pixels()[row + column] != css::Color::new(255, 0, 0, 255) {
          layout_box_pixels_right_position = false;
        }
      }
    }

    assert_eq!(layout_box_pixels_count, 2500);
    assert_eq!(layout_box_pixels_right_position, true);
  }
}
