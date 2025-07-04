use super::expression::expression;
use crate::grammar::attributes::Attributes;
use crate::grammar::nodes::AstNode;
use crate::grammar::nodes::InnerNode;
use crate::grammar::nodes::LeafNode;
use crate::grammar::nodes::Node;

pub fn prefix_expression_guard(attributes: &Attributes) -> bool {
    let return_type = attributes.type_context.last().unwrap();
    return_type == "Int" || return_type == "Float" || return_type == "Bool"
}

pub fn prefix_expression(attributes: &mut Attributes) -> AstNode {
    let mut children = vec![prefix(attributes)];
    attributes.is_start_expression = false;
    let let_save = attributes.let_expr_allowed;
    attributes.let_expr_allowed = false;
    children.push(expression(attributes));
    attributes.let_expr_allowed = let_save;
    Node::Inner(InnerNode {
        // tab_level: attributes.tab_level,
        children: children,
    })
}

fn prefix(attributes: &Attributes) -> AstNode {
    let tabs = if attributes.is_start_expression {
        attributes.tab_level
    } else {
        0
    };
    let prefix_symbol = match attributes.type_context.last().unwrap().as_str() {
        "Int" | "Float" => "-".to_string(),
        "Bool" => "!".to_string(),
        _ => panic!("Invalid type for prefix expression"),
    };
    Node::Leaf(LeafNode {
        tabs: tabs,
        token: prefix_symbol,
        new_lines: 0,
    })
}
