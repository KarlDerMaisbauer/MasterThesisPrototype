use super::expression::expression;
use crate::grammar::attributes::Attributes;
use crate::grammar::nodes::AstNode;
use crate::grammar::nodes::Node;
use crate::grammar::nodes::NonTerminalInfo;
use crate::grammar::nodes::TerminalInfo;
use crate::grammar::r#type::type_blacklisted::type_blacklisted;
use crate::grammar::r#type::type_whitelisted::type_whitelisted;

pub fn let_expression_quard(attributes: &Attributes) -> bool {
    attributes.let_expr_allowed && attributes.max_expr_depth > 0
}

pub fn let_expression(attributes: &mut Attributes) -> AstNode {
    let mut children = vec![Node::Terminal(TerminalInfo {
        tabs: if attributes.in_match_expr && attributes.first_match_let {
            0
        } else {
            attributes.tab_level
        },
        token: "let ".to_string(),
        new_lines: 0,
    })];
    let var_type = type_blacklisted(attributes, vec!["Nothing".to_string()], 0, 0).token;
    if attributes.in_match_expr {
        attributes.first_match_let = false;
    }
    let var_name = format!("var{}", attributes.current_var_id);
    attributes.current_var_id += 1;
    attributes.type_context.push(var_type.clone());
    children.push(Node::Terminal(TerminalInfo {
        tabs: 0,
        token: var_name.clone(),
        new_lines: 0,
    }));
    children.push(Node::Terminal(TerminalInfo {
        tabs: 0,
        token: " = ".to_string(),
        new_lines: 0,
    }));
    attributes.is_start_expression = false;
    attributes.is_end_expression = true;
    attributes.let_expr_allowed = false;
    attributes.max_expr_depth -= 1;
    children.push(expression(attributes));
    attributes.max_expr_depth += 1;
    if attributes.in_match_expr {
        attributes
            .match_expr_vars
            .last_mut()
            .unwrap()
            .insert(var_name.clone(), var_type);
    } else {
        attributes.current_vars.insert(var_name.clone(), var_type);
    }
    attributes.type_context.pop();
    attributes.is_start_expression = true;
    attributes.is_end_expression = true;
    attributes.let_expr_allowed = true;
    attributes.max_expr_depth -= 1;
    children.push(expression(attributes));
    attributes.max_expr_depth += 1;
    Node::NonTerminal(NonTerminalInfo {
        // tab_level: attributes.tab_level,
        children: children,
    })
}
