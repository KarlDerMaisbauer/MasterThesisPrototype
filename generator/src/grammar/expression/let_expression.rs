use super::expression::expression;
use crate::grammar::attributes::Attributes;
use crate::grammar::nodes::AstNode;
use crate::grammar::nodes::InnerNode;
use crate::grammar::nodes::LeafNode;
use crate::grammar::nodes::Node;
use crate::grammar::r#type::type_whitelisted::type_whitelisted;

pub fn let_expression_quard(attributes: &Attributes) -> bool {
    attributes.let_expr_allowed
}

pub fn let_expression(attributes: &mut Attributes) -> AstNode {
    let mut children = vec![Node::Leaf(LeafNode {
        tabs: attributes.tab_level,
        token: "let ".to_string(),
        new_lines: 0,
    })];
    let var_type = type_whitelisted(
        attributes,
        vec!["Int".to_string(), "Float".to_string(), "Bool".to_string()],
        0,
        0,
    )
    .token;
    let var_name = format!("var{}", attributes.current_var_id);
    attributes.current_var_id += 1;
    attributes.type_context.push(var_type.clone());
    children.push(Node::Leaf(LeafNode {
        tabs: 0,
        token: var_name.clone(),
        new_lines: 0,
    }));
    children.push(Node::Leaf(LeafNode {
        tabs: 0,
        token: " = ".to_string(),
        new_lines: 0,
    }));
    attributes.is_start_expression = false;
    attributes.is_end_expression = true;
    attributes.let_expr_allowed = false;
    children.push(expression(attributes));

    attributes.current_vars.insert(var_name.clone(), var_type);
    attributes.type_context.pop();
    attributes.is_start_expression = true;
    attributes.is_end_expression = true;
    attributes.let_expr_allowed = true;
    children.push(expression(attributes));
    Node::Inner(InnerNode {
        // tab_level: attributes.tab_level,
        children: children,
    })
}
