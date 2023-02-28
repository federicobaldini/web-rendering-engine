mod css;
mod css_parser;
mod dom;
mod html_parser;
mod layout;
mod style;
mod text_parser;

use style::StyledNode;

use crate::css_parser::CSSParser;
use crate::dom::Node;
use crate::html_parser::HTMLParser;

fn main() {
  let _html_text_dom: Node = Node::text("Hello World!".to_string());
  let root_dom_node: Node = HTMLParser::parse("<div class='container-1'><h1>My Minimal Browser</h1></div><div class='container-2'><p class='paragraph'>It works!</p></div>".to_string());
  // Node::print_node_tree(&root_dom_node, 0);
  let _css_stylesheet: css::Stylesheet = CSSParser::parse(
    ".container-1{width:200px;background:#FFFFFF;}.container-2{background:#A3E4D7;}".to_string(),
  );
  let root_style_node: StyledNode = style::style_tree(&root_dom_node, &_css_stylesheet);
  StyledNode::print_style_node_tree(&root_style_node, 0)
}
