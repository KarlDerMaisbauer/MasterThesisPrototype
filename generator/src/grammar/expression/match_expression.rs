use super::expression::expression;
use crate::grammar::attributes::Attributes;
use crate::grammar::attributes::VarMap;
use crate::grammar::nodes::AstNode;
use crate::grammar::nodes::Node;
use crate::grammar::nodes::NonTerminalInfo;
use crate::grammar::nodes::TerminalInfo;
use crate::grammar::utils::gen_type::gen_type::gen_type;
use crate::grammar::utils::gen_type::gen_type_blacklisted::gen_type_blacklisted;
use crate::grammar::utils::gen_type::gen_type_whitelisted::gen_type_whitelisted;

pub fn match_expression_guard(attributes: &Attributes) -> bool {
    let depth = attributes.max_expr_depth;
    let matcher = attributes.match_arm_expr;
    !matcher && depth > 0 && attributes.match_expr_valid
}

pub fn match_expression(attributes: &mut Attributes) -> AstNode {
    let tabs_start = if attributes.is_start_expression {
        attributes.tab_level
    } else {
        0
    };

    let tabs_end = attributes.tab_level;

    attributes.match_expr_valid = false;
    attributes.let_expr_allowed = false;
    attributes.in_match_expr = true;
    // let destructuring_type = gen_type_whitelisted(attributes, vec!["Int".to_string()]);
    let destructuring_type = gen_type_blacklisted(attributes, vec!["Nothing".to_string()]);
    let mut children = vec![Node::Terminal(TerminalInfo {
        tabs: tabs_start,
        token: "match ".to_string(),
        new_lines: 0,
    })];
    attributes.is_start_expression = false;
    attributes.is_end_expression = true;
    attributes.type_context.push(destructuring_type.clone());
    attributes.match_expr_valid = false;
    children.push(expression(attributes));
    attributes.match_expr_valid = true;
    attributes.type_context.pop();
    attributes.tab_level += 1;
    // match arms
    attributes.max_expr_depth -= 1;
    let mut num_match_arms = 5;
    while num_match_arms > 0 {
        num_match_arms -= 1;
        attributes.type_context.push(destructuring_type.clone());
        children.push(match_arm(attributes));
    }

    attributes.max_expr_depth += 1;
    children.push(Node::Terminal(TerminalInfo {
        tabs: tabs_end,
        token: "end".to_string(),
        new_lines: 1,
    }));
    attributes.tab_level -= 1;

    attributes.let_expr_allowed = true;
    attributes.match_expr_valid = true;
    attributes.in_match_expr = false;
    Node::NonTerminal(NonTerminalInfo { children: children })
}

fn match_arm(attributes: &mut Attributes) -> AstNode {
    let let_expr_save = attributes.let_expr_allowed;
    attributes.match_expr_vars.push(VarMap::new());
    let mut children: Vec<AstNode> = Vec::new();
    attributes.is_start_expression = true;
    attributes.is_end_expression = false;
    attributes.let_expr_allowed = false;
    children.push(destructuring_expression(attributes));
    attributes.type_context.pop();
    children.push(Node::Terminal(TerminalInfo {
        tabs: 0,
        token: " => ".to_string(),
        new_lines: 0,
    }));
    attributes.is_end_expression = true;
    attributes.is_start_expression = false;
    attributes.let_expr_allowed = true;
    attributes.first_match_let = true;
    attributes.tab_level += 1;
    children.push(expression(attributes));
    attributes.tab_level -= 1;
    attributes.match_expr_vars.pop();
    attributes.let_expr_allowed = let_expr_save;
    Node::NonTerminal(NonTerminalInfo { children })
}

fn destructuring_expression(attributes: &mut Attributes) -> AstNode {
    attributes.match_arm_expr = true;
    attributes.match_expr_valid = false;
    let matcher = expression(attributes);
    attributes.match_expr_valid = true;
    attributes.match_arm_expr = false;
    matcher
}
