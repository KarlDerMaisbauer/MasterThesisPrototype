use rand::seq::IndexedRandom;

use super::expression::expression;
use crate::grammar::attributes::Attributes;
use crate::grammar::nodes::AstNode;
use crate::grammar::nodes::Node;
use crate::grammar::nodes::NonTerminalInfo;
use crate::grammar::nodes::TerminalInfo;

pub fn function_call_expression_guard(attributes: &Attributes) -> bool {
    let return_type = attributes.type_context.last().unwrap();
    let no_zero_value = attributes.no_zero_value;
    let matcher = attributes.match_arm_expr;
    !no_zero_value
        && !matcher
        && attributes
            .function_map
            .iter()
            .fold(false, |acc, (_k, (_params, ret))| acc || return_type == ret)
}

pub fn function_call_expression(attributes: &mut Attributes) -> AstNode {
    let mut children: Vec<AstNode> = Vec::new();
    let return_type = attributes.type_context.last().unwrap();
    let available_functions: Vec<(String, Vec<(String, String)>)> = attributes
        .function_map
        .iter()
        .filter(|(_k, (_params, ret))| ret == return_type)
        .map(|(k, (params, _ret))| (k.clone(), params.clone()))
        .collect();
    let tabs = if attributes.is_start_expression {
        attributes.tab_level
    } else {
        0
    };
    let new_lines = if attributes.is_end_expression { 1 } else { 0 };
    attributes.is_start_expression = false;
    attributes.is_end_expression = false;
    attributes.let_expr_allowed = false;
    let (f_name, f_params) = available_functions.choose(&mut rand::rng()).unwrap();
    children.push(Node::Terminal(TerminalInfo {
        tabs: tabs,
        token: (*f_name).clone(),
        new_lines: 0,
    }));

    children.push(Node::Terminal(TerminalInfo {
        tabs: 0,
        token: "(".to_string(),
        new_lines: 0,
    }));

    let mut param_iter = (*f_params).iter().peekable();
    attributes.match_expr_valid = false;
    while let Some((_param_name, param_type)) = param_iter.next() {
        attributes.type_context.push(param_type.clone());
        children.push(expression(attributes));
        attributes.type_context.pop();
        if matches!(param_iter.peek(), Some(_)) {
            children.push(Node::Terminal(TerminalInfo {
                tabs: 0,
                token: ", ".to_string(),
                new_lines: 0,
            }));
        }
    }
    attributes.match_expr_valid = true;

    children.push(Node::Terminal(TerminalInfo {
        tabs: 0,
        token: ")".to_string(),
        new_lines: new_lines,
    }));

    Node::NonTerminal(NonTerminalInfo { children: children })
}
