use super::struct_members::struct_members;
use crate::grammar::attributes::Attributes;
use crate::grammar::nodes::{AstNode, InnerNode, LeafNode, Node};

pub fn r#struct(attributes: &mut Attributes) -> AstNode {
    let struct_name = format!("Struct{}", attributes.struct_id);
    let (mut member_nodes, member_map) = struct_members(attributes);
    attributes
        .struct_map
        .insert(struct_name.clone(), member_map);
    let mut children = vec![
        Node::Leaf(LeafNode {
            tabs: 0,
            token: "struct ".to_string(),
            new_lines: 0,
        }),
        Node::Leaf(LeafNode {
            tabs: 0,
            token: struct_name,
            new_lines: 1,
        }),
    ];
    children.append(&mut member_nodes);
    children.push(Node::Leaf(LeafNode {
        tabs: 0,
        token: "end".to_string(),
        new_lines: 2,
    }));
    attributes.struct_id += 1;
    Node::Inner(InnerNode { children })
}
