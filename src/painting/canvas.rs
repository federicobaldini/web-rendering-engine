use ab_glyph::{Font, PxScale, ScaleFont};

use crate::css;
use crate::layout;
use super::display_list::{build_display_list, DisplayCommand};

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
  pub(super) font: Option<ab_glyph::FontVec>,
}

impl Canvas {
  // Create a blank canvas
  pub(super) fn new(width: usize, height: usize) -> Canvas {
    let white: css::Color = css::Color::new(255, 255, 255, 255);
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

  pub(super) fn paint_item(&mut self, item: &DisplayCommand) {
    match item {
      DisplayCommand::SolidColor(color, rectangle) => {
        // Clip the rectangle to the canvas boundaries.
        let x0: usize = rectangle.x().clamp(0.0, self.width as f32) as usize;
        let y0: usize = rectangle.y().clamp(0.0, self.height as f32) as usize;
        let x1: usize = (rectangle.x() + rectangle.width()).clamp(0.0, self.width as f32) as usize;
        let y1: usize =
          (rectangle.y() + rectangle.height()).clamp(0.0, self.height as f32) as usize;

        // Normalize source alpha to [0.0, 1.0].
        let src_a: f32 = color.alpha() as f32 / 255.0;
        let inv_a: f32 = 1.0 - src_a;

        for y in y0..y1 {
          for x in x0..x1 {
            // Porter-Duff "over": blend src color over the existing destination pixel.
            let dst: css::Color = self.pixels[x + y * self.width];
            let dst_a: f32 = dst.alpha() as f32 / 255.0;
            let r: u8 = (color.red() as f32 * src_a + dst.red() as f32 * inv_a) as u8;
            let g: u8 = (color.green() as f32 * src_a + dst.green() as f32 * inv_a) as u8;
            let b: u8 = (color.blue() as f32 * src_a + dst.blue() as f32 * inv_a) as u8;
            let a: u8 = ((src_a + dst_a * inv_a) * 255.0) as u8;
            self.pixels[x + y * self.width] = css::Color::new(r, g, b, a);
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
