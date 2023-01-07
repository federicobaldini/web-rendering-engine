mod dom;
mod parser;

fn main() {
  let _html_text_dom = dom::Node::text("Hello World!".to_string());
  let _root_dom_node = parser::Parser::parse("<div><span>Hello World!</span></div>".to_string());
}
