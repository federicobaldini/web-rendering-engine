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
    _ => return, // bail out if no border-color is specified
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
}
