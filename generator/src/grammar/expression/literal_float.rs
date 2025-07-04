use crate::grammar::attributes::Attributes;
use crate::grammar::nodes::{AstNode, LeafNode, Node};
use rand::Rng;

pub fn literal_float_guard(attributes: &Attributes) -> bool {
    attributes.type_context.last().unwrap() == "Float"
}

pub fn literal_float(attributes: &mut Attributes) -> AstNode {
    let mut rng = rand::rng();
    let mut value = rng.random::<f64>();
    while value == 0f64 {
        value = rng.random::<f64>();
    }
    let tabs = if attributes.is_start_expression {
        attributes.tab_level
    } else {
        0
    };
    let new_lines = if attributes.is_end_expression { 1 } else { 0 };
    Node::Leaf(LeafNode {
        tabs: tabs,
        token: value.to_string(),
        new_lines: new_lines,
    })
}
