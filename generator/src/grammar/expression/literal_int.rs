use crate::grammar::attributes::Attributes;
use crate::grammar::nodes::{AstNode, LeafNode, Node};
use rand::Rng;

pub fn literal_int_guard(attributes: &Attributes) -> bool {
    attributes.type_context.last().unwrap() == "Int"
}

pub fn literal_int(attributes: &mut Attributes) -> AstNode {
    let mut rng = rand::rng();
    let mut value = rng.random::<i16>();
    while value <= 0 {
        value = rng.random::<i16>();
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
