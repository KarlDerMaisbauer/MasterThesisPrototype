use super::expression::expression;
use crate::grammar::attributes::Attributes;
use crate::grammar::nodes::AstNode;
use crate::grammar::nodes::Node;
use crate::grammar::nodes::NonTerminalInfo;
use crate::grammar::nodes::TerminalInfo;

pub fn prefix_expression_guard(attributes: &Attributes) -> bool {
    let return_type = attributes.type_context.last().unwrap();
    let depth = attributes.max_expr_depth;
    depth > 0 && (return_type == "Int" || return_type == "Float" || return_type == "Bool")
}

pub fn prefix_expression(attributes: &mut Attributes) -> AstNode {
    let mut children = vec![prefix(attributes)];
    attributes.is_start_expression = false;
    let let_save = attributes.let_expr_allowed;
    attributes.let_expr_allowed = false;
    attributes.max_expr_depth -= 1;
    children.push(expression(attributes));
    attributes.max_expr_depth += 1;
    attributes.let_expr_allowed = let_save;
    Node::NonTerminal(NonTerminalInfo { children: children })
}

fn prefix(attributes: &Attributes) -> AstNode {
    let tabs = if attributes.is_start_expression {
        attributes.tab_level
    } else {
        0
    };
    let prefix_symbol = match attributes.type_context.last().unwrap().as_str() {
        "Int" | "Float" => "-".to_string(),
        "Bool" => "not ".to_string(),
        _ => panic!("Invalid type for prefix expression"),
    };
    Node::Terminal(TerminalInfo {
        tabs: tabs,
        token: prefix_symbol,
        new_lines: 0,
    })
}
