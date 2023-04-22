mod css;
mod css_parser;
mod dom;
mod html_parser;
mod layout;
mod painting;
mod style;
mod text_parser;

use std::fs::File;
use std::io::Read;

use crate::css_parser::CSSParser;
use crate::html_parser::HTMLParser;

fn read_source(filename: String) -> String {
  let mut str = String::new();
  File::open(filename)
    .unwrap()
    .read_to_string(&mut str)
    .unwrap();
  str
}

fn main() {
  // Parse command-line options:
  let mut options: getopts::Options = getopts::Options::new();
  options.optopt("h", "html", "HTML document", "FILENAME");
  options.optopt("c", "css", "CSS stylesheet", "FILENAME");
  options.optopt("o", "output", "Output file", "FILENAME");
  options.optopt("f", "format", "Output file format", "png");

  let matches = options.parse(std::env::args().skip(1)).unwrap();
  let str_arg =
    |flag: &str, default: &str| -> String { matches.opt_str(flag).unwrap_or(default.to_string()) };

  // Choose a format:
  let png: bool = match &str_arg("f", "png")[..] {
    "png" => true,
    x => panic!("Unknown output format: {}", x),
  };

  // Read input files:
  let html: String = read_source(str_arg("h", "examples/test.html"));
  let css: String = read_source(str_arg("c", "examples/test.css"));

  // Since we don't have an actual window, hard-code the "viewport" size.
  let mut viewport: layout::Dimensions = Default::default();
  viewport.set_content().set_width(800.0);
  viewport.set_content().set_height(600.0);

  // Parsing and rendering:
  let root_node: dom::Node = HTMLParser::parse(html);
  let stylesheet: css::Stylesheet = CSSParser::parse(css);
  let style_root: style::StyledNode = style::style_tree(&root_node, &stylesheet);
  let layout_root: layout::LayoutBox = layout::layout_tree(&style_root, viewport);

  dom::Node::print_node_tree(&root_node, 0);
  // style::StyledNode::print_style_node_tree(&style_root, 0);

  // Create the output file:
  let filename: String = str_arg("o", if png { "output.png" } else { "output.txt" });

  // Write to the file:
  let ok: bool = if png {
    let canvas: painting::Canvas = painting::Canvas::paint(&layout_root, *viewport.content());
    let (w, h): (u32, u32) = (canvas.width() as u32, canvas.height() as u32);
    let img: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> =
      image::ImageBuffer::from_fn(w, h, move |x: u32, y: u32| {
        let color: css::Color = canvas.pixels()[(y * w + x) as usize];
        image::Rgba([color.red(), color.green(), color.blue(), color.alpha()])
      });
    image::DynamicImage::ImageRgba8(img)
      .save_with_format(filename.clone(), image::ImageFormat::Png)
      .is_ok()
  } else {
    false
  };
  if ok {
    println!("Saved output as {}", filename)
  } else {
    println!("Error saving output as {}", filename)
  }
}
