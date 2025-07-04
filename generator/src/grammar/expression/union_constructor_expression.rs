use super::expression::expression;
use crate::grammar::attributes::Attributes;
use crate::grammar::nodes::AstNode;
use crate::grammar::nodes::InnerNode;
use crate::grammar::nodes::LeafNode;
use crate::grammar::nodes::Node;
use rand::prelude::*;

pub fn union_constructor_expression_guard(attributes: &Attributes) -> bool {
    let return_type = attributes.type_context.last().unwrap();
    attributes
        .union_map
        .iter()
        .fold(false, |acc, (k, _)| acc || (k == return_type))
}

pub fn union_constructor_expression(attributes: &mut Attributes) -> AstNode {
    let union_type = attributes.type_context.last().unwrap();
    let tabs = if attributes.is_start_expression {
        attributes.tab_level
    } else {
        0
    };
    attributes.is_start_expression = false;
    let mut children: Vec<AstNode> = vec![
        Node::Leaf(LeafNode {
            tabs: tabs,
            token: union_type.clone(),
            new_lines: 0,
        }),
        Node::Leaf(LeafNode {
            tabs: tabs,
            token: "::".to_string(),
            new_lines: 0,
        }),
    ];

    let (member_name, member_type) = attributes
        .union_map
        .get(union_type)
        .unwrap()
        .iter()
        .choose(&mut rand::rng())
        .unwrap();
    children.push(Node::Leaf(LeafNode {
        tabs: 0,
        token: member_name.clone(),
        new_lines: 0,
    }));

    children.push(Node::Leaf(LeafNode {
        tabs: 0,
        token: "(".to_string(),
        new_lines: 0,
    }));
    if member_type.clone() != "Nothing".to_string() {
        attributes.type_context.push(member_type.clone());
        children.push(expression(attributes));
        attributes.type_context.pop();
    }

    // if members.len() > 0 {
    //     let is_end_save = attributes.is_end_expression;
    //     attributes.is_end_expression = false;
    //     let mut iter = members.iter().peekable();
    //     while let Some(member_type) = iter.next() {
    //         attributes.type_context.push(member_type.clone());
    //         children.push(expression(attributes));
    //         attributes.type_context.pop();
    //         if matches!(iter.peek(), Some(_)) {
    //             children.push(Node::Leaf(LeafNode {
    //                 tabs: 0,
    //                 token: ", ".to_string(),
    //                 new_lines: 0,
    //             }));
    //         }
    //     }
    //     attributes.is_end_expression = is_end_save;
    // }
    let new_lines = if attributes.is_end_expression { 1 } else { 0 };
    children.push(Node::Leaf(LeafNode {
        tabs: 0,
        token: ")".to_string(),
        new_lines: new_lines,
    }));
    Node::Inner(InnerNode { children })
}
