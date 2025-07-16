use crate::grammar::attributes::Attributes;
use rand::prelude::*;

pub fn gen_type(attributes: &Attributes) -> String {
    let mut rng = rand::rng();
    let mut types = vec![
        "Int".to_string(),
        "Float".to_string(),
        "Bool".to_string(),
        "String".to_string(),
        "Nothing".to_string(),
    ];

    types.append(&mut attributes.get_union_types());
    types.append(&mut attributes.get_struct_types());
    types.choose(&mut rng).unwrap().clone()
}
