use crate::grammar::attributes::Attributes;
use crate::grammar::nodes::LeafNode;
use rand::prelude::*;

pub fn r#type(attributes: &mut Attributes, tabs: usize, new_lines: usize) -> LeafNode {
    let mut rng = rand::rng();
    let mut types = vec![
        "Int".to_string(),
        "Float".to_string(),
        "Bool".to_string(),
        "Nothing".to_string(),
    ];

    types.append(&mut attributes.get_union_types());
    types.append(&mut attributes.get_struct_types());
    LeafNode {
        tabs: tabs,
        token: types.choose(&mut rng).unwrap().clone(),
        new_lines: new_lines,
    }
}
