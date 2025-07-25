use super::expression::expression;
use crate::grammar::attributes::Attributes;
use crate::grammar::attributes::VarMap;
use crate::grammar::expression::Acceptor;
use crate::grammar::expression::Expression;
use crate::grammar::expression::var_call_expression::var_call_expression;
use crate::grammar::expression::var_call_expression::var_call_expression_guard;
use crate::grammar::literal::literal;
use crate::grammar::literal::literal_guard;
use crate::grammar::nodes::AstNode;
use crate::grammar::nodes::Node;
use crate::grammar::nodes::NonTerminalInfo;
use crate::grammar::nodes::TerminalInfo;
use crate::grammar::utils::gen_type::gen_type_blacklisted::gen_type_blacklisted;
use rand::Rng;
use rand::prelude::IndexedMutRandom;
use rand::prelude::IndexedRandom;

pub fn match_expression_guard(attributes: &Attributes) -> bool {
    let depth = attributes.max_expr_depth;
    let matcher = attributes.match_arm_expr;
    !matcher && depth > 0 && attributes.match_expr_valid && !attributes.no_zero_value
}

pub fn match_expression(attributes: &mut Attributes) -> AstNode {
    attributes.generic_match_arm_generated.push(false);
    attributes.match_arms.push(0);
    attributes.first_caturing_expression.push(true);
    attributes.first_match_let.push(true);
    attributes.in_match_expr = true;
    let tabs_start = if attributes.is_start_expression {
        attributes.tab_level
    } else {
        0
    };

    let tabs_end = attributes.tab_level;

    attributes.match_expr_valid = false;
    attributes.let_expr_allowed = false;
    // let destructuring_type = gen_type_whitelisted(attributes, vec!["Int".to_string()]);
    let mut blacklist = attributes
        .struct_map
        .iter()
        .map(|(k, _v)| k.clone())
        .collect::<Vec<String>>();
    blacklist.push("Nothing".to_string());
    let destructuring_type = gen_type_blacklisted(attributes, blacklist);
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
    *attributes.match_arms.last_mut().unwrap() = 0;
    let mut rng = rand::rng();
    while (rng.random::<u32>() % 2) != 0
        || !attributes.generic_match_arm_generated.last().unwrap()
        || *attributes.match_arms.last().unwrap() <= 2
    {
        *attributes.first_match_let.last_mut().unwrap() = true;
        attributes.in_match_expr = true;
        *attributes.match_arms.last_mut().unwrap() += 1;
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
    // attributes.in_match_expr = false;
    attributes.generic_match_arm_generated.pop();
    attributes.match_arms.pop();
    attributes.first_caturing_expression.pop();
    attributes.first_match_let.pop();
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
    *attributes.first_caturing_expression.last_mut().unwrap() = true;
    attributes.match_arm_union_used = false;
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
    *attributes.first_match_let.last_mut().unwrap() = true;
    attributes.tab_level += 1;
    let generic_match_arm_save = *attributes.generic_match_arm_generated.last().unwrap();
    children.push(expression(attributes));
    *attributes.generic_match_arm_generated.last_mut().unwrap() = generic_match_arm_save;
    attributes.tab_level -= 1;
    attributes.match_expr_vars.pop();
    attributes.let_expr_allowed = let_expr_save;
    Node::NonTerminal(NonTerminalInfo { children })
}

fn destructuring_expression(attributes: &mut Attributes) -> AstNode {
    attributes.match_arm_expr = true;
    attributes.match_expr_valid = false;
    // let matcher = vec![
    //     (literal_guard, literal),
    //     (capturing_expression_guard, capturing_expression),
    //     (var_call_expression_guard, var_call_expression),
    //     (
    //         union_constructor_expression_guard,
    //         union_constructor_expression,
    //     ),
    // ]
    let expressions: Vec<(Acceptor, Expression)> = vec![
        (literal_guard, literal),
        (capturing_expression_guard, capturing_expression),
        (var_call_expression_guard, var_call_expression),
        (
            destructuring_union_expression_guard,
            destructuring_union_expression,
        ),
    ];

    let matcher = expressions
        .iter()
        .filter(|(guard, _f)| guard(attributes))
        .map(|(_g, expression)| expression)
        .collect::<Vec<&Expression>>()
        .choose_mut(&mut rand::rng())
        .unwrap()(attributes);
    // let matcher = expression(attributes);
    attributes.match_expr_valid = true;
    attributes.match_arm_expr = false;
    matcher
}

fn capturing_expression_guard(_attributes: &Attributes) -> bool {
    true
}

fn capturing_expression(attributes: &mut Attributes) -> AstNode {
    let var_type = attributes.type_context.last().unwrap();
    let mut children: Vec<AstNode> = Vec::new();
    let ignore_var = rand::rng().random::<bool>();
    let var_name = if ignore_var {
        let name = format!("var{}", attributes.current_var_id);
        attributes
            .match_expr_vars
            .last_mut()
            .unwrap()
            .insert(name.clone(), var_type.clone());
        name
    } else {
        "_".to_string()
    };
    attributes.current_var_id += 1;
    let tabs = if attributes.match_arm_union_used {
        0
    } else {
        attributes.tab_level
    };
    children.push(Node::Terminal(TerminalInfo {
        tabs,
        token: "let ".to_string(),
        new_lines: 0,
    }));
    children.push(Node::Terminal(TerminalInfo {
        tabs: 0,
        token: var_name,
        new_lines: 0,
    }));
    if *attributes.first_match_let.last().unwrap() {
        *attributes.generic_match_arm_generated.last_mut().unwrap() = true;
    }

    Node::NonTerminal(NonTerminalInfo { children })
}

fn destructuring_union_expression_guard(attributes: &Attributes) -> bool {
    let data_type = attributes.type_context.last().unwrap();
    attributes
        .union_map
        .iter()
        .map(|(k, _v)| k)
        .collect::<Vec<&String>>()
        .contains(&data_type)
        && !attributes.match_arm_union_used
}

fn destructuring_union_expression(attributes: &mut Attributes) -> AstNode {
    let union_type = attributes.type_context.last().unwrap();
    let mut children: Vec<AstNode> = Vec::new();
    let tabs = if attributes.is_start_expression {
        attributes.tab_level
    } else {
        0
    };
    attributes.is_start_expression = false;
    attributes.match_arm_union_used = true;
    let struct_types = attributes
        .struct_map
        .iter()
        .map(|(type_name, _members)| type_name.clone())
        .collect::<Vec<String>>();
    let (union_member_name, union_member_type): (String, String) = attributes
        .union_map
        .get(union_type)
        .unwrap()
        .iter()
        .filter(|(_, data_type)| !struct_types.contains(data_type))
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect::<Vec<(String, String)>>()
        .choose(&mut rand::rng())
        .unwrap()
        .clone();
    children.push(Node::Terminal(TerminalInfo {
        tabs: tabs,
        token: union_type.clone(),
        new_lines: 0,
    }));
    children.push(Node::Terminal(TerminalInfo {
        tabs: 0,
        token: "::".to_string(),
        new_lines: 0,
    }));
    children.push(Node::Terminal(TerminalInfo {
        tabs: 0,
        token: union_member_name,
        new_lines: 0,
    }));
    children.push(Node::Terminal(TerminalInfo {
        tabs: 0,
        token: "(".to_string(),
        new_lines: 0,
    }));
    if union_member_type != "Nothing".to_string() {
        *attributes.first_caturing_expression.last_mut().unwrap() = false;
        attributes.type_context.push(union_member_type);
        children.push(destructuring_expression(attributes));
        attributes.type_context.pop();
    }
    children.push(Node::Terminal(TerminalInfo {
        tabs: 0,
        token: ")".to_string(),
        new_lines: 0,
    }));
    Node::NonTerminal(NonTerminalInfo { children })
}
