use crate::grammar::attributes::Attributes;
use crate::grammar::nodes::AstNode;
use crate::grammar::nodes::LeafNode;
use crate::grammar::nodes::Node;
use rand::prelude::*;

pub fn var_call_expression_guard(attributes: &Attributes) -> bool {
    let return_type = attributes.type_context.last().unwrap();
    let mut callable = attributes
        .current_params
        .iter()
        .fold(false, |callabe_acc, (_, v)| callabe_acc || v == return_type);
    callable = attributes
        .current_vars
        .iter()
        .fold(callable, |callable_acc, (_, v)| {
            callable_acc || v == return_type
        });
    callable
}

pub fn var_call_expression(attributes: &mut Attributes) -> AstNode {
    let return_type = attributes.type_context.last().unwrap();
    let mut rng = rand::rng();
    let mut possible_vars =
        attributes
            .current_params
            .iter()
            .fold(vec![], |mut possible, (k, v)| {
                if v == return_type {
                    possible.push(k);
                }
                possible
            });
    possible_vars = attributes
        .current_vars
        .iter()
        .fold(possible_vars, |mut possible, (k, v)| {
            if v == return_type {
                possible.push(k);
            }
            possible
        });
    let var = possible_vars.choose(&mut rng).unwrap().to_string();
    let tabs = if attributes.is_start_expression {
        attributes.tab_level
    } else {
        0
    };
    let new_lines = if attributes.is_end_expression { 1 } else { 0 };
    Node::Leaf(LeafNode {
        tabs: tabs,
        token: var,
        new_lines: new_lines,
    })
}
