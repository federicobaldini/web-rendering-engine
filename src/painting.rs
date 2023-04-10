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
}
