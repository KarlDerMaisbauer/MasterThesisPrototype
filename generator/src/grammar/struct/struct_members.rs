use crate::grammar::attributes::{Attributes, MemberMap};
use crate::grammar::nodes::{AstNode, LeafNode, Node};
use crate::grammar::r#type::type_blacklisted::type_blacklisted;
use rand::Rng;

pub fn struct_members(attributes: &mut Attributes) -> (Vec<AstNode>, MemberMap) {
    let mut rng = rand::rng();
    let mut members = MemberMap::new();
    let mut curr_members = struct_members_inner(0, attributes, &mut members);
    let mut curr_member_id = 1;

    while (rng.random::<u32>() % 3) != 0 {
        curr_members.append(&mut struct_members_inner(
            curr_member_id,
            attributes,
            &mut members,
        ));
        curr_member_id += 1;
    }
    (curr_members, members)
}

fn struct_members_inner(
    member_id: usize,
    attributes: &mut Attributes,
    members: &mut MemberMap,
) -> Vec<AstNode> {
    let member_name = format!("member{}", member_id);
    let member_type = type_blacklisted(attributes, vec!["Nothing".to_string()], 0, 0);
    members.insert(member_name.clone(), member_type.token.clone());
    vec![
        Node::Leaf(LeafNode {
            tabs: 1,
            token: member_name,
            new_lines: 0,
        }),
        Node::Leaf(LeafNode {
            tabs: 0,
            token: ": ".to_string(),
            new_lines: 0,
        }),
        Node::Leaf(member_type),
        Node::Leaf(LeafNode {
            tabs: 0,
            token: "".to_string(),
            new_lines: 1,
        }),
    ]
}
