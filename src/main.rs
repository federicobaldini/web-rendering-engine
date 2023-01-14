mod dom;
mod html_parser;

fn main() {
  let _html_text_dom = dom::Node::text("Hello World!".to_string());
  let root_dom_node = html_parser::Parser::parse("<div class='container-1'></div><div class='container-2'><p class='paragraph'>Hello World!</p></div>".to_string());
  dom::Node::print_node_tree(root_dom_node, 0);
}
