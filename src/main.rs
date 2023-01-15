mod css_parser;
mod dom;
mod html_parser;
mod css;

fn main() {
  let _html_text_dom: dom::Node = dom::Node::text("Hello World!".to_string());
  let root_dom_node: dom::Node = html_parser::Parser::parse("<div class='container-1'></div><div class='container-2'><p class='paragraph'>Hello World!</p></div>".to_string());
  dom::Node::print_node_tree(root_dom_node, 0);
  let _css_stylesheet: css::Stylesheet =
    css_parser::Parser::parse(".container{width:200px}".to_string());
}
