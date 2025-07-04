use super::union_members::union_members;
use crate::grammar::attributes::Attributes;
use crate::grammar::nodes::{AstNode, InnerNode, LeafNode, Node};

pub fn union(attributes: &mut Attributes) -> AstNode {
    let union_name = format!("Union{}", attributes.union_id);
    let (mut member_nodes, member_map) = union_members(attributes);
    attributes.union_map.insert(union_name.clone(), member_map);
    let mut children = vec![
        Node::Leaf(LeafNode {
            tabs: 0,
            token: "union ".to_string(),
            new_lines: 0,
        }),
        Node::Leaf(LeafNode {
            tabs: 0,
            token: union_name,
            new_lines: 1,
        }),
    ];
    children.append(&mut member_nodes);
    children.push(Node::Leaf(LeafNode {
        tabs: 0,
        token: "end".to_string(),
        new_lines: 2,
    }));
    attributes.union_id += 1;
    Node::Inner(InnerNode { children })
}
