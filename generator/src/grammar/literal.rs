use std::char;

use crate::grammar::attributes::Attributes;
use crate::grammar::nodes::{AstNode, Node, TerminalInfo};
use rand::Rng;

pub fn literal_guard(attributes: &Attributes) -> bool {
    let return_type = attributes.type_context.last().unwrap();
    return_type == "Int"
        || return_type == "Float"
        || return_type == "Bool"
        || return_type == "String"
}

pub fn literal(attributes: &mut Attributes) -> AstNode {
    let return_type = attributes.type_context.last().unwrap().as_str();
    let mut rng = rand::rng();
    // let no_zero_value = attributes.no_zero_value
    let value = match return_type {
        "Int" => {
            let mut value_intermediate = rng.random::<i16>();
            while value_intermediate <= 0 {
                value_intermediate = rng.random::<i16>();
            }
            value_intermediate.to_string()
        }
        "Float" => {
            let mut value_intermediate = rng.random::<f32>();
            while value_intermediate <= 0f32 {
                value_intermediate = rng.random::<f32>();
            }
            value_intermediate.to_string()
        }
        "Bool" => rng.random::<bool>().to_string(),
        "String" => {
            let num_chars = rng.random::<u64>() % 10;
            format!(
                "\"{}\"",
                (rng.sample_iter(rand::distr::Alphanumeric))
                    .take(num_chars.try_into().unwrap())
                    .map(|v| v as char)
                    .collect::<String>()
            )
        }
        _ => panic!("Invalid type for literal"),
    };

    let tabs = if attributes.is_start_expression {
        attributes.tab_level
    } else {
        0
    };
    let new_lines = if attributes.is_end_expression { 1 } else { 0 };
    Node::Terminal(TerminalInfo {
        tabs: tabs,
        token: value,
        new_lines: new_lines,
    })
}
