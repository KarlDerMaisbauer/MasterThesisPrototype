use super::expression::expression;
use crate::grammar::attributes::Attributes;
use crate::grammar::attributes::MemberMap;
use crate::grammar::nodes::AstNode;
use crate::grammar::nodes::Node;
use crate::grammar::nodes::NonTerminalInfo;
use crate::grammar::nodes::TerminalInfo;
use rand::prelude::*;

pub fn member_access_expression_guard(attributes: &Attributes) -> bool {
    let return_type = attributes.type_context.last().unwrap();
    let no_zero_value = attributes.no_zero_value;
    let matcher = attributes.match_arm_expr;
    !no_zero_value
        && !matcher
        && attributes.max_expr_depth > 0
        && attributes
            .struct_map
            .iter()
            .fold(false, |acc, (_k, members)| {
                acc || members.iter().fold(false, |acc, (_name, member_type)| {
                    acc || member_type == return_type
                })
            })
}

pub fn member_access_expression(attributes: &mut Attributes) -> AstNode {
    attributes.max_expr_depth -= 1;
    let member_type = attributes.type_context.last().unwrap();
    // let struct_type: String = attributes
    //     .struct_map
    //     .iter()
    //     .filter(|(_k, members)| {
    //         members.iter().fold(false, |acc, (_name, data_type)| {
    //             acc || member_type == data_type
    //         })
    //     })
    //     .map(|(name, _members)| name.clone())
    //     .collect::<Vec<String>>()
    //     .choose(&mut rand::rng())
    //     .unwrap()
    //     .clone();

    let possible_stucts = attributes
        .struct_map
        .iter()
        .filter(|(_k, members)| {
            members.iter().fold(false, |acc, (_name, data_type)| {
                acc || member_type == data_type
            })
        })
        .map(|(name, members)| (name.clone(), members.clone()))
        .collect::<Vec<(String, MemberMap)>>();
    let (struct_type, struct_members) = possible_stucts.choose(&mut rand::rng()).unwrap();
    let member: String = struct_members
        .iter()
        .filter(|(_name, data_type)| *data_type == member_type)
        .map(|(name, _type)| name.clone())
        .collect::<Vec<String>>()
        .choose(&mut rand::rng())
        .unwrap()
        .clone();

    let mut children: Vec<AstNode> = Vec::new();
    attributes.type_context.push(struct_type.clone());
    let end_expr_save = attributes.is_end_expression;
    attributes.let_expr_allowed = false;
    attributes.is_end_expression = false;
    attributes.match_expr_valid = false;
    children.push(expression(attributes));
    attributes.match_expr_valid = true;
    attributes.type_context.pop();
    children.push(Node::Terminal(TerminalInfo {
        tabs: 0,
        token: ".".to_string(),
        new_lines: 0,
    }));
    attributes.is_start_expression = false;
    let new_lines = if end_expr_save { 1 } else { 0 };
    children.push(Node::Terminal(TerminalInfo {
        tabs: 0,
        token: member,
        new_lines: new_lines,
    }));
    attributes.max_expr_depth += 1;
    Node::NonTerminal(NonTerminalInfo { children })
}
