use crate::grammar::attributes::Attributes;
use crate::grammar::function_arg::function_arg;
use crate::grammar::nodes::{AstNode, InnerNode, LeafNode, Node};
use rand::Rng;

pub fn function_type_specification(attributes: &mut Attributes) -> AstNode {
    let mut children: Vec<AstNode> = vec![];
    let mut rng = rand::rng();
    children.push(Node::Leaf(LeafNode {
        tabs: 0,
        token: "(".to_string(),
        new_lines: 0,
    }));

    let mut param_id = 0usize;

    if rng.random::<u32>() % 2 != 0 && !attributes.is_main_func {
        children.push(function_arg(attributes, param_id));
        param_id += 1;

        while rng.random::<u32>() % 3 != 0 {
            children.push(Node::Leaf(LeafNode {
                tabs: 0,
                token: ", ".to_string(),
                new_lines: 0,
            }));
            children.push(function_arg(attributes, param_id));
            param_id += 1;
        }
    }

    children.push(Node::Leaf(LeafNode {
        tabs: 0,
        token: ") ".to_string(),
        new_lines: 0,
    }));
    Node::Inner(InnerNode {
        // tab_level: 0,
        children: children,
    })
}
