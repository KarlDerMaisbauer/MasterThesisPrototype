use crate::grammar::attributes::Attributes;
use crate::grammar::nodes::LeafNode;
use rand::prelude::*;

pub fn type_whitelisted(
    attributes: &mut Attributes,
    whitelist: Vec<String>,
    tabs: usize,
    new_lines: usize,
) -> LeafNode {
    let mut rng = rand::rng();
    let mut types = vec![
        "Int".to_string(),
        "Float".to_string(),
        "Bool".to_string(),
        "Nothing".to_string(),
    ];

    types.append(&mut attributes.get_union_types());
    types.append(&mut attributes.get_struct_types());

    types = types
        .into_iter()
        .filter(|type_id| whitelist.contains(type_id))
        .collect();
    LeafNode {
        tabs: tabs,
        token: types.choose(&mut rng).unwrap().clone(),
        new_lines: new_lines,
    }
}
