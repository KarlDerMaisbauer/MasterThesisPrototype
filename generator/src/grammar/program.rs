use crate::grammar::attributes::Attributes;
use crate::grammar::nodes::{AstNode, InnerNode, Node};
use crate::grammar::toplevel::toplevel;
use rand::Rng;

pub fn program() -> AstNode {
    let mut rng = rand::rng();
    let mut attributes = Attributes::default();
    let mut children: Vec<AstNode> = vec![];
    while (rng.random::<u32>() % 5) != 0 || !attributes.main_func_generated {
        children.push(toplevel(&mut attributes));
    }
    Node::Inner(InnerNode { children })
}
