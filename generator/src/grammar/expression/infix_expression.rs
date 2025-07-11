use super::expression::expression;
use crate::grammar::attributes::Attributes;
use crate::grammar::nodes::AstNode;
use crate::grammar::nodes::Node;
use crate::grammar::nodes::NonTerminalInfo;
use crate::grammar::nodes::TerminalInfo;
use crate::grammar::utils::gen_type::gen_type::gen_type;
use crate::grammar::utils::gen_type::gen_type_whitelisted::gen_type_whitelisted;
use rand::prelude::*;

pub fn infix_expression_guard(attributes: &Attributes) -> bool {
    let return_type = attributes.type_context.last().unwrap();
    let depth = attributes.max_expr_depth;
    let no_zero_value = attributes.no_zero_value;
    let matcher = attributes.match_arm_expr;
    !no_zero_value
        && !matcher
        && depth > 0
        && (return_type == "Int" || return_type == "Float" || return_type == "Bool")
}

pub fn infix_expression(attributes: &mut Attributes) -> AstNode {
    let mut children: Vec<AstNode> = vec![];
    let is_end_expression_save = attributes.is_end_expression;
    let (infix_node, context) = infix_operator(attributes);
    attributes.max_expr_depth -= 1;
    attributes.is_end_expression = false;
    let let_save = attributes.let_expr_allowed;
    attributes.let_expr_allowed = false;
    attributes.match_expr_valid = false;
    attributes.type_context.push(context.clone());
    children.push(expression(attributes));
    attributes.type_context.pop();
    attributes.max_expr_depth += 1;
    children.push(infix_node);
    attributes.is_start_expression = false;
    attributes.is_end_expression = is_end_expression_save;
    attributes.max_expr_depth -= 1;
    attributes.type_context.push(context);
    children.push(expression(attributes));
    attributes.match_expr_valid = true;
    attributes.no_zero_value = false;
    attributes.type_context.pop();
    attributes.max_expr_depth += 1;

    attributes.let_expr_allowed = let_save;

    Node::NonTerminal(NonTerminalInfo { children: children })
}

fn infix_operator(attributes: &mut Attributes) -> (AstNode, String) {
    let return_type = attributes.type_context.last().unwrap();
    let mut rng = rand::rng();
    let operators: Vec<&str> = match return_type.as_str() {
        "Int" => vec!["+", "-", "/", "*", "^", "%"],
        "Float" => vec!["+", "-", "/", "*"],
        "Bool" => vec!["and", "or", "xor", "nand", "==", "!=", "<", "<=", ">", ">="],
        _ => panic!("invalid type for infix operator"),
    };
    let operator_raw = operators.choose(&mut rng).unwrap();
    if *operator_raw == "/" && *operator_raw == "%" {
        attributes.no_zero_value = true;
    }
    let new_type_context = match *operator_raw {
        "==" => gen_type(attributes),
        "!=" => gen_type(attributes),
        "<" => gen_type_whitelisted(attributes, vec!["Int".to_string(), "FLoat".to_string()]),
        "<=" => gen_type_whitelisted(attributes, vec!["Int".to_string(), "FLoat".to_string()]),
        ">" => gen_type_whitelisted(attributes, vec!["Int".to_string(), "FLoat".to_string()]),
        ">=" => gen_type_whitelisted(attributes, vec!["Int".to_string(), "FLoat".to_string()]),
        _ => return_type.clone(),
    };
    let operator = format!(" {} ", operator_raw);
    (
        Node::Terminal(TerminalInfo {
            tabs: 0,
            token: operator,
            new_lines: 0,
        }),
        new_type_context,
    )
}
