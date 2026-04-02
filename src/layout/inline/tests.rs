use crate::css;
use crate::dom;
use crate::hashmap;
use crate::layout::*;
use crate::style;

// Test that two display:inline-block elements are placed side by side inside a block container
#[test]
fn test_layout_inline_block() {
  let node_1: dom::Node = dom::Node::element(
    "div".to_string(),
    hashmap![String::from("class") => String::from("a")],
    vec![],
  );
  let node_2: dom::Node = dom::Node::element(
    "div".to_string(),
    hashmap![String::from("class") => String::from("b")],
    vec![],
  );
  let stylesheet: css::Stylesheet = css::Stylesheet::new(vec![
    css::Rule::new(
      vec![css::Selector::Simple(css::SimpleSelector::new(None, None, vec!["a".to_string()]))],
      vec![
        css::Declaration::new("display".to_string(), css::Value::Keyword("inline-block".to_string())),
        css::Declaration::new("width".to_string(), css::Value::Length(60.0, css::Unit::Px)),
        css::Declaration::new("height".to_string(), css::Value::Length(40.0, css::Unit::Px)),
      ],
    ),
    css::Rule::new(
      vec![css::Selector::Simple(css::SimpleSelector::new(None, None, vec!["b".to_string()]))],
      vec![
        css::Declaration::new("display".to_string(), css::Value::Keyword("inline-block".to_string())),
        css::Declaration::new("width".to_string(), css::Value::Length(80.0, css::Unit::Px)),
        css::Declaration::new("height".to_string(), css::Value::Length(30.0, css::Unit::Px)),
      ],
    ),
  ]);
  let mut values_1: style::PropertyMap = hashmap![];
  let mut values_2: style::PropertyMap = hashmap![];
  match node_1.node_type() {
    dom::NodeType::Element(element) => values_1 = style::specified_values(&element, &stylesheet),
    _ => {}
  }
  match node_2.node_type() {
    dom::NodeType::Element(element) => values_2 = style::specified_values(&element, &stylesheet),
    _ => {}
  }
  let style_node_1: style::StyledNode = style::StyledNode::new(&node_1, values_1, vec![]);
  let style_node_2: style::StyledNode = style::StyledNode::new(&node_2, values_2, vec![]);
  let child_box_1: LayoutBox = LayoutBox::new(BoxType::InlineBlockNode(&style_node_1));
  let child_box_2: LayoutBox = LayoutBox::new(BoxType::InlineBlockNode(&style_node_2));
  let mut anon_box: LayoutBox = LayoutBox::new(BoxType::AnonymousBlock);
  anon_box.add_child(child_box_1);
  anon_box.add_child(child_box_2);

  let containing_block: Dimensions = Dimensions::new(
    Rectangle::new(0.0, 0.0, 200.0, 0.0),
    EdgeSizes::new(0.0, 0.0, 0.0, 0.0),
    EdgeSizes::new(0.0, 0.0, 0.0, 0.0),
    EdgeSizes::new(0.0, 0.0, 0.0, 0.0),
  );

  anon_box.layout_anonymous_block(containing_block);

  // Both children fit on one line: child 1 at x=0, child 2 at x=60
  assert_eq!(anon_box.children()[0].dimensions().content().x(), 0.0);
  assert_eq!(anon_box.children()[0].dimensions().content().y(), 0.0);
  assert_eq!(anon_box.children()[0].dimensions().content().width(), 60.0);
  assert_eq!(anon_box.children()[0].dimensions().content().height(), 40.0);
  assert_eq!(anon_box.children()[1].dimensions().content().x(), 60.0);
  assert_eq!(anon_box.children()[1].dimensions().content().y(), 0.0);
  assert_eq!(anon_box.children()[1].dimensions().content().width(), 80.0);
  assert_eq!(anon_box.children()[1].dimensions().content().height(), 30.0);
  // Anonymous block height equals the tallest child
  assert_eq!(anon_box.dimensions().content().height(), 40.0);
}

// Test that descendants of an InlineBlockNode get their positions correctly offset
// after the parent anonymous block finalizes the inline-block's position.
// Regression test: children were laid out with x=0,y=0 as the base and never updated.
#[test]
fn test_inline_block_descendant_positions() {
  // Inner text node inside the second inline-block (offset x=60)
  let inner_node: dom::Node = dom::Node::element(
    "span".to_string(),
    hashmap![String::from("class") => String::from("inner")],
    vec![],
  );
  let node_1: dom::Node = dom::Node::element(
    "div".to_string(),
    hashmap![String::from("class") => String::from("a")],
    vec![],
  );
  let node_2: dom::Node = dom::Node::element(
    "div".to_string(),
    hashmap![String::from("class") => String::from("b")],
    vec![inner_node.clone()],
  );
  let stylesheet: css::Stylesheet = css::Stylesheet::new(vec![
    css::Rule::new(
      vec![css::Selector::Simple(css::SimpleSelector::new(None, None, vec!["a".to_string()]))],
      vec![
        css::Declaration::new("display".to_string(), css::Value::Keyword("inline-block".to_string())),
        css::Declaration::new("width".to_string(), css::Value::Length(60.0, css::Unit::Px)),
        css::Declaration::new("height".to_string(), css::Value::Length(40.0, css::Unit::Px)),
      ],
    ),
    css::Rule::new(
      vec![css::Selector::Simple(css::SimpleSelector::new(None, None, vec!["b".to_string()]))],
      vec![
        css::Declaration::new("display".to_string(), css::Value::Keyword("inline-block".to_string())),
        css::Declaration::new("width".to_string(), css::Value::Length(80.0, css::Unit::Px)),
        css::Declaration::new("height".to_string(), css::Value::Length(30.0, css::Unit::Px)),
      ],
    ),
    css::Rule::new(
      vec![css::Selector::Simple(css::SimpleSelector::new(None, None, vec!["inner".to_string()]))],
      vec![
        css::Declaration::new("display".to_string(), css::Value::Keyword("inline-block".to_string())),
        css::Declaration::new("width".to_string(), css::Value::Length(20.0, css::Unit::Px)),
        css::Declaration::new("height".to_string(), css::Value::Length(10.0, css::Unit::Px)),
      ],
    ),
  ]);
  let mut values_1: style::PropertyMap = hashmap![];
  let mut values_2: style::PropertyMap = hashmap![];
  let mut values_inner: style::PropertyMap = hashmap![];
  match node_1.node_type() {
    dom::NodeType::Element(element) => values_1 = style::specified_values(&element, &stylesheet),
    _ => {}
  }
  match node_2.node_type() {
    dom::NodeType::Element(element) => values_2 = style::specified_values(&element, &stylesheet),
    _ => {}
  }
  match inner_node.node_type() {
    dom::NodeType::Element(element) => values_inner = style::specified_values(&element, &stylesheet),
    _ => {}
  }
  let style_inner: style::StyledNode = style::StyledNode::new(&inner_node, values_inner, vec![]);
  let style_node_1: style::StyledNode = style::StyledNode::new(&node_1, values_1, vec![]);
  let style_node_2: style::StyledNode = style::StyledNode::new(&node_2, values_2, vec![style_inner]);

  let child_box_1: LayoutBox = LayoutBox::new(BoxType::InlineBlockNode(&style_node_1));
  let mut child_box_2: LayoutBox = LayoutBox::new(BoxType::InlineBlockNode(&style_node_2));
  // Build the layout subtree for node_2's child manually
  let inner_style = &style_node_2.children()[0];
  let inner_box: LayoutBox = LayoutBox::new(BoxType::InlineBlockNode(inner_style));
  let mut anon_for_2: LayoutBox = LayoutBox::new(BoxType::AnonymousBlock);
  anon_for_2.add_child(inner_box);
  child_box_2.add_child(anon_for_2);

  let mut anon_box: LayoutBox = LayoutBox::new(BoxType::AnonymousBlock);
  anon_box.add_child(child_box_1);
  anon_box.add_child(child_box_2);

  let containing_block: Dimensions = Dimensions::new(
    Rectangle::new(0.0, 0.0, 200.0, 0.0),
    EdgeSizes::new(0.0, 0.0, 0.0, 0.0),
    EdgeSizes::new(0.0, 0.0, 0.0, 0.0),
    EdgeSizes::new(0.0, 0.0, 0.0, 0.0),
  );

  anon_box.layout_anonymous_block(containing_block);

  // node_2 is positioned at x=60; its anonymous block child must reflect that offset
  let node_2_box = &anon_box.children()[1];
  assert_eq!(node_2_box.dimensions().content().x(), 60.0);
  let anon_child = &node_2_box.children()[0]; // anonymous block inside node_2
  assert_eq!(anon_child.dimensions().content().x(), 60.0,
    "anonymous block inside inline-block should inherit the parent's x offset");
  let inner = &anon_child.children()[0]; // inner inline-block
  assert_eq!(inner.dimensions().content().x(), 60.0,
    "descendant of inline-block should have x offset propagated, not stuck at 0");
}

// Test the method layout_inline of the LayoutBox struct implementation
#[test]
fn test_layout_inline() {
  let tag_name: String = String::from("span");
  let attributes: dom::AttributeMap =
    hashmap![String::from("class") => String::from("inline-1")];
  let node: dom::Node = dom::Node::element(tag_name, attributes, vec![]);
  let simple_selector: css::SimpleSelector =
    css::SimpleSelector::new(None, None, vec!["inline-1".to_string()]);
  let selector: css::Selector = css::Selector::Simple(simple_selector);
  let rule: css::Rule = css::Rule::new(
    vec![selector],
    vec![
      css::Declaration::new("width".to_string(), css::Value::Length(80.0, css::Unit::Px)),
      css::Declaration::new("height".to_string(), css::Value::Length(20.0, css::Unit::Px)),
      css::Declaration::new("padding".to_string(), css::Value::Length(4.0, css::Unit::Px)),
      css::Declaration::new("border-width".to_string(), css::Value::Length(2.0, css::Unit::Px)),
      css::Declaration::new("margin".to_string(), css::Value::Length(3.0, css::Unit::Px)),
    ],
  );
  let stylesheet: css::Stylesheet = css::Stylesheet::new(vec![rule]);
  let mut values: style::PropertyMap = hashmap![];
  match node.node_type() {
    dom::NodeType::Element(element) => {
      values = style::specified_values(&element, &stylesheet);
    }
    _ => {}
  }
  let style_node: style::StyledNode = style::StyledNode::new(&node, values, vec![]);
  let mut layout_box: LayoutBox = LayoutBox::new(BoxType::InlineNode(&style_node));
  let containing_block: Dimensions = Dimensions::new(
    Rectangle::new(0.0, 0.0, 200.0, 100.0),
    EdgeSizes::new(0.0, 0.0, 0.0, 0.0),
    EdgeSizes::new(0.0, 0.0, 0.0, 0.0),
    EdgeSizes::new(0.0, 0.0, 0.0, 0.0),
  );

  layout_box.layout_inline(containing_block);

  assert_eq!(layout_box.dimensions().content().width(), 80.0);
  assert_eq!(layout_box.dimensions().content().height(), 20.0);
  assert_eq!(layout_box.dimensions().padding().top(), 4.0);
  assert_eq!(layout_box.dimensions().padding().left(), 4.0);
  assert_eq!(layout_box.dimensions().border().top(), 2.0);
  assert_eq!(layout_box.dimensions().border().left(), 2.0);
  assert_eq!(layout_box.dimensions().margin().top(), 3.0);
  assert_eq!(layout_box.dimensions().margin().left(), 3.0);
}

// Test the method layout_anonymous_block of the LayoutBox struct implementation.
// Two inline children fit side by side on a single line.
#[test]
fn test_layout_anonymous_block_single_line() {
  let node_1: dom::Node = dom::Node::element(
    "span".to_string(),
    hashmap![String::from("class") => String::from("a")],
    vec![],
  );
  let node_2: dom::Node = dom::Node::element(
    "span".to_string(),
    hashmap![String::from("class") => String::from("b")],
    vec![],
  );
  let stylesheet: css::Stylesheet = css::Stylesheet::new(vec![
    css::Rule::new(
      vec![css::Selector::Simple(css::SimpleSelector::new(None, None, vec!["a".to_string()]))],
      vec![
        css::Declaration::new("width".to_string(), css::Value::Length(50.0, css::Unit::Px)),
        css::Declaration::new("height".to_string(), css::Value::Length(30.0, css::Unit::Px)),
      ],
    ),
    css::Rule::new(
      vec![css::Selector::Simple(css::SimpleSelector::new(None, None, vec!["b".to_string()]))],
      vec![
        css::Declaration::new("width".to_string(), css::Value::Length(60.0, css::Unit::Px)),
        css::Declaration::new("height".to_string(), css::Value::Length(20.0, css::Unit::Px)),
      ],
    ),
  ]);
  let mut values_1: style::PropertyMap = hashmap![];
  let mut values_2: style::PropertyMap = hashmap![];
  match node_1.node_type() {
    dom::NodeType::Element(element) => values_1 = style::specified_values(&element, &stylesheet),
    _ => {}
  }
  match node_2.node_type() {
    dom::NodeType::Element(element) => values_2 = style::specified_values(&element, &stylesheet),
    _ => {}
  }
  let style_node_1: style::StyledNode = style::StyledNode::new(&node_1, values_1, vec![]);
  let style_node_2: style::StyledNode = style::StyledNode::new(&node_2, values_2, vec![]);
  let child_box_1: LayoutBox = LayoutBox::new(BoxType::InlineNode(&style_node_1));
  let child_box_2: LayoutBox = LayoutBox::new(BoxType::InlineNode(&style_node_2));
  let mut anon_box: LayoutBox = LayoutBox::new(BoxType::AnonymousBlock);
  anon_box.add_child(child_box_1);
  anon_box.add_child(child_box_2);

  let containing_block: Dimensions = Dimensions::new(
    Rectangle::new(0.0, 0.0, 200.0, 0.0),
    EdgeSizes::new(0.0, 0.0, 0.0, 0.0),
    EdgeSizes::new(0.0, 0.0, 0.0, 0.0),
    EdgeSizes::new(0.0, 0.0, 0.0, 0.0),
  );

  anon_box.layout_anonymous_block(containing_block);

  // Both children fit on one line: child 1 at x=0, child 2 at x=50
  assert_eq!(anon_box.children()[0].dimensions().content().x(), 0.0);
  assert_eq!(anon_box.children()[0].dimensions().content().y(), 0.0);
  assert_eq!(anon_box.children()[1].dimensions().content().x(), 50.0);
  assert_eq!(anon_box.children()[1].dimensions().content().y(), 0.0);
  // Height equals the tallest child on the line
  assert_eq!(anon_box.dimensions().content().height(), 30.0);
}

// Test that layout_anonymous_block wraps to a new line when a child no longer fits.
#[test]
fn test_layout_anonymous_block_wrapping() {
  let node_1: dom::Node = dom::Node::element(
    "span".to_string(),
    hashmap![String::from("class") => String::from("a")],
    vec![],
  );
  let node_2: dom::Node = dom::Node::element(
    "span".to_string(),
    hashmap![String::from("class") => String::from("b")],
    vec![],
  );
  let stylesheet: css::Stylesheet = css::Stylesheet::new(vec![
    css::Rule::new(
      vec![css::Selector::Simple(css::SimpleSelector::new(None, None, vec!["a".to_string()]))],
      vec![
        css::Declaration::new("width".to_string(), css::Value::Length(50.0, css::Unit::Px)),
        css::Declaration::new("height".to_string(), css::Value::Length(30.0, css::Unit::Px)),
      ],
    ),
    css::Rule::new(
      vec![css::Selector::Simple(css::SimpleSelector::new(None, None, vec!["b".to_string()]))],
      vec![
        css::Declaration::new("width".to_string(), css::Value::Length(60.0, css::Unit::Px)),
        css::Declaration::new("height".to_string(), css::Value::Length(20.0, css::Unit::Px)),
      ],
    ),
  ]);
  let mut values_1: style::PropertyMap = hashmap![];
  let mut values_2: style::PropertyMap = hashmap![];
  match node_1.node_type() {
    dom::NodeType::Element(element) => values_1 = style::specified_values(&element, &stylesheet),
    _ => {}
  }
  match node_2.node_type() {
    dom::NodeType::Element(element) => values_2 = style::specified_values(&element, &stylesheet),
    _ => {}
  }
  let style_node_1: style::StyledNode = style::StyledNode::new(&node_1, values_1, vec![]);
  let style_node_2: style::StyledNode = style::StyledNode::new(&node_2, values_2, vec![]);
  let child_box_1: LayoutBox = LayoutBox::new(BoxType::InlineNode(&style_node_1));
  let child_box_2: LayoutBox = LayoutBox::new(BoxType::InlineNode(&style_node_2));
  let mut anon_box: LayoutBox = LayoutBox::new(BoxType::AnonymousBlock);
  anon_box.add_child(child_box_1);
  anon_box.add_child(child_box_2);

  // Container is only 60px wide: child 1 (50px) fits, child 2 (60px) wraps
  let containing_block: Dimensions = Dimensions::new(
    Rectangle::new(0.0, 0.0, 60.0, 0.0),
    EdgeSizes::new(0.0, 0.0, 0.0, 0.0),
    EdgeSizes::new(0.0, 0.0, 0.0, 0.0),
    EdgeSizes::new(0.0, 0.0, 0.0, 0.0),
  );

  anon_box.layout_anonymous_block(containing_block);

  // Child 1 stays on line 0
  assert_eq!(anon_box.children()[0].dimensions().content().x(), 0.0);
  assert_eq!(anon_box.children()[0].dimensions().content().y(), 0.0);
  // Child 2 wraps to line 1 (y = height of line 0 = 30)
  assert_eq!(anon_box.children()[1].dimensions().content().x(), 0.0);
  assert_eq!(anon_box.children()[1].dimensions().content().y(), 30.0);
  // Total height = line 0 (30) + line 1 (20)
  assert_eq!(anon_box.dimensions().content().height(), 50.0);
}
