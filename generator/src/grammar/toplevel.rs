use crate::grammar::attributes::Attributes;
use crate::grammar::function::function;
use crate::grammar::nodes::AstNode;
use crate::grammar::r#struct::r#struct::r#struct;
use crate::grammar::union::union::union;
use rand::Rng;

pub fn toplevel(attributes: &mut Attributes) -> AstNode {
    let mut rng = rand::rng();
    match rng.random::<u32>() % 3 {
        0 => union(attributes),
        1 => r#struct(attributes),
        _ => function(attributes),
    }
}
