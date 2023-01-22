mod css;
mod css_parser;
mod dom;
mod html_parser;
mod text_parser;

use crate::css_parser::CSSParser;
use crate::dom::Node;
use crate::html_parser::HTMLParser;

fn main() {
  let _html_text_dom: Node = Node::text("Hello World!".to_string());
  let root_dom_node: Node = HTMLParser::parse("<div class='container-1'></div><div class='container-2'><p class='paragraph'>Hello World!</p></div>".to_string());
  Node::print_node_tree(root_dom_node, 0);
  let _css_stylesheet: css::Stylesheet = CSSParser::parse(".container{width:200px;}".to_string());
}
