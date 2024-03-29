use crate::css;
use crate::layout;

type DisplayList = Vec<DisplayCommand>;

#[derive(Debug)]
enum DisplayCommand {
  SolidColor(css::Color, layout::Rectangle),
  // insert more commands here
}

impl PartialEq for DisplayCommand {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (DisplayCommand::SolidColor(a, b), DisplayCommand::SolidColor(c, d)) => a == c && b == d,
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

fn render_layout_box(list: &mut DisplayList, layout_box: &layout::LayoutBox) {
  render_background(list, layout_box);
  render_borders(list, layout_box);
  // TODO: render text

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
}

impl Canvas {
  // Create a blank canvas
  fn new(width: usize, height: usize) -> Canvas {
    let white = css::Color::new(255, 255, 255, 255);
    return Canvas {
      pixels: vec![white; width * height],
      width,
      height,
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
    }
  }

  // Paint a tree of LayoutBoxes to an array of pixels.
  pub fn paint(layout_root: &layout::LayoutBox, bounds: layout::Rectangle) -> Canvas {
    let display_list: Vec<DisplayCommand> = build_display_list(layout_root);
    let mut canvas: Canvas = Canvas::new(bounds.width() as usize, bounds.height() as usize);
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
          layout_box.dimensions().border().left(),
          layout_box.dimensions().border_box().height()
        )
      )
    );
    // Assert that the first element of the display list is a SolidColor DisplayCommand with the right color and right border data
    assert_eq!(
      display_list[1],
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
