use crate::grammar::attributes::Attributes;
use crate::grammar::attributes::MemberMap;
use crate::grammar::nodes::{AstNode, LeafNode, Node};
use crate::grammar::r#type::r#type::r#type;
use rand::Rng;

pub fn union_members(attributes: &mut Attributes) -> (Vec<AstNode>, MemberMap) {
    let mut rng = rand::rng();
    let mut members = MemberMap::new();
    let mut curr_members = union_members_inner(0, attributes, &mut members);
    let mut curr_member_id = 1;
    while (rng.random::<u32>() % 3) != 0 {
        curr_members.append(&mut union_members_inner(
            curr_member_id,
            attributes,
            &mut members,
        ));
        curr_member_id += 1;
    }
    (curr_members, members)
}

fn union_members_inner(
    member_id: usize,
    attributes: &mut Attributes,
    members: &mut MemberMap,
) -> Vec<AstNode> {
    let member_name = format!("member{}", member_id);
    let member_type = r#type(attributes, 0, 0);
    members.insert(member_name.clone(), member_type.token.clone());

    vec![
        Node::Leaf(LeafNode {
            tabs: 1,
            token: member_name,
            new_lines: 0,
        }),
        Node::Leaf(LeafNode {
            tabs: 0,
            token: "(".to_string(),
            new_lines: 0,
        }),
        Node::Leaf(member_type),
        Node::Leaf(LeafNode {
            tabs: 0,
            token: ")".to_string(),
            new_lines: 1,
        }),
    ]
}
