use super::expression::expression;
use crate::grammar::attributes::Attributes;
use crate::grammar::nodes::AstNode;
use crate::grammar::nodes::Node;
use crate::grammar::nodes::NonTerminalInfo;
use crate::grammar::nodes::TerminalInfo;

pub fn struct_constructor_expression_guard(attributes: &Attributes) -> bool {
    let return_type = attributes.type_context.last().unwrap();
    let no_zero_value = attributes.no_zero_value;
    !no_zero_value
        && attributes
            .struct_map
            .iter()
            .fold(false, |acc, (k, _)| acc || (k == return_type))
}

pub fn struct_constructor_expression(attributes: &mut Attributes) -> AstNode {
    // attributes.max_expr_depth -= 1;
    let struct_type = attributes.type_context.last().unwrap();
    let new_lines = if attributes.is_end_expression { 1 } else { 0 };
    let tabs = if attributes.is_start_expression {
        attributes.tab_level
    } else {
        0
    };
    attributes.is_start_expression = false;
    attributes.is_end_expression = false;
    attributes.let_expr_allowed = false;
    let mut children: Vec<AstNode> = vec![
        Node::Terminal(TerminalInfo {
            tabs: tabs,
            token: struct_type.clone(),
            new_lines: 0,
        }),
        Node::Terminal(TerminalInfo {
            tabs: 0,
            token: "(".to_string(),
            new_lines: 0,
        }),
    ];
    let member_map = attributes.struct_map.get(struct_type).unwrap();
    let mut member_keys: Vec<String> = member_map.keys().map(|k| k.clone()).collect();
    member_keys.sort();
    let members: Vec<String> = member_keys
        .iter()
        .map(|k| member_map.get(k).unwrap().clone())
        .collect();
    if members.len() > 0 {
        let is_end_save = attributes.is_end_expression;
        attributes.is_end_expression = false;
        let mut iter = members.iter().peekable();
        attributes.match_expr_valid = false;
        while let Some(member_type) = iter.next() {
            attributes.match_expr_valid = false;
            attributes.type_context.push(member_type.clone());
            children.push(expression(attributes));
            attributes.type_context.pop();
            if matches!(iter.peek(), Some(_)) {
                children.push(Node::Terminal(TerminalInfo {
                    tabs: 0,
                    token: ", ".to_string(),
                    new_lines: 0,
                }));
            }
        }
        attributes.match_expr_valid = true;
        attributes.is_end_expression = is_end_save;
    }
    children.push(Node::Terminal(TerminalInfo {
        tabs: 0,
        token: ")".to_string(),
        new_lines: new_lines,
    }));
    // attributes.max_expr_depth += 1;
    Node::NonTerminal(NonTerminalInfo { children })
}
