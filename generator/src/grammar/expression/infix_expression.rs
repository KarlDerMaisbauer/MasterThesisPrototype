use super::expression::expression;
use crate::grammar::attributes::Attributes;
use crate::grammar::nodes::AstNode;
use crate::grammar::nodes::InnerNode;
use crate::grammar::nodes::LeafNode;
use crate::grammar::nodes::Node;
use rand::prelude::*;

pub fn infix_expression_guard(attributes: &Attributes) -> bool {
    let return_type = attributes.type_context.last().unwrap();
    let depth = attributes.max_expr_depth;
    depth > 0 && (return_type == "Int" || return_type == "Float" || return_type == "Bool")
}

pub fn infix_expression(attributes: &mut Attributes) -> AstNode {
    let mut children: Vec<AstNode> = vec![];
    let is_end_expression_save = attributes.is_end_expression;
    attributes.max_expr_depth -= 1;
    attributes.is_end_expression = false;
    let let_save = attributes.let_expr_allowed;
    attributes.let_expr_allowed = false;
    children.push(expression(attributes));
    attributes.max_expr_depth += 1;
    children.push(infix_operator(attributes));
    attributes.is_start_expression = false;
    attributes.is_end_expression = is_end_expression_save;
    attributes.max_expr_depth -= 1;
    children.push(expression(attributes));
    attributes.max_expr_depth += 1;

    attributes.let_expr_allowed = let_save;

    Node::Inner(InnerNode {
        // tab_level: attributes.tab_level,
        children: children,
    })
}

fn infix_operator(attributes: &Attributes) -> AstNode {
    let return_type = attributes.type_context.last().unwrap();
    let mut rng = rand::rng();
    let operators: Vec<&str> = match return_type.as_str() {
        "Int" => vec!["+", "-", "/", "*", "^", "%"], // "==", "!=", "<", "<=", ">", ">=", "%",
        "Float" => vec!["+", "-", "/", "*"],         // "^", "==", "!=", "<", "<=", ">", ">="],
        "Bool" => vec!["and", "or", "xor", "nand"],
        _ => panic!("invalid type for infix operator"),
    };
    let operator = format!(" {} ", operators.choose(&mut rng).unwrap());
    Node::Leaf(LeafNode {
        tabs: 0,
        token: operator,
        new_lines: 0,
    })
}
