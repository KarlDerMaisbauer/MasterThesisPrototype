use super::expression::expression;
use crate::grammar::attributes::Attributes;
use crate::grammar::nodes::AstNode;
use crate::grammar::nodes::Node;
use crate::grammar::nodes::NonTerminalInfo;
use crate::grammar::nodes::TerminalInfo;

pub fn bracket_expression_guard(attributes: &Attributes) -> bool {
    let return_type = attributes.type_context.last().unwrap();
    return_type == "Int" || return_type == "Float" || return_type == "Bool"
}

pub fn bracket_expression(attributes: &mut Attributes) -> AstNode {
    let tabs = if attributes.is_start_expression {
        attributes.tab_level
    } else {
        0
    };
    let new_lines = if attributes.is_end_expression { 1 } else { 0 };
    let mut children = vec![Node::Terminal(TerminalInfo {
        tabs: tabs,
        token: "(".to_string(),
        new_lines: 0,
    })];
    attributes.is_start_expression = false;
    attributes.is_end_expression = false;
    let let_save = attributes.let_expr_allowed;
    attributes.let_expr_allowed = false;
    children.push(expression(attributes));
    children.push(Node::Terminal(TerminalInfo {
        tabs: 0,
        token: ")".to_string(),
        new_lines: new_lines,
    }));
    attributes.let_expr_allowed = let_save;

    Node::NonTerminal(NonTerminalInfo {
        // tab_level: attributes.tab_level,
        children: children,
    })
}
