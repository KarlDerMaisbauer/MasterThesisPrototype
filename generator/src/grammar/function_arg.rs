use crate::grammar::attributes::Attributes;
use crate::grammar::nodes::{AstNode, InnerNode, LeafNode, Node};
use crate::grammar::r#type::type_blacklisted::type_blacklisted;

pub fn function_arg(attributes: &mut Attributes, param_id: usize) -> AstNode {
    let mut children: Vec<AstNode> = vec![];
    let param_name = format!("param{}", param_id);
    let param_type = type_blacklisted(attributes, vec!["Nothing".to_string()], 0, 0);
    attributes
        .current_params
        .insert(param_name.clone(), param_type.token.clone());
    children.push(Node::Leaf(LeafNode {
        tabs: 0,
        token: param_name,
        new_lines: 0,
    }));

    children.push(Node::Leaf(LeafNode {
        tabs: 0,
        token: ": ".to_string(),
        new_lines: 0,
    }));
    children.push(Node::Leaf(param_type));
    Node::Inner(InnerNode {
        // tab_level: 0,
        children: children,
    })
}
