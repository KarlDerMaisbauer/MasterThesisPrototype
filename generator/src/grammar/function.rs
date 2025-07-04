use crate::grammar::attributes::Attributes;
use crate::grammar::attributes::{ParamMap, VarMap};
use crate::grammar::expression::expression;
use crate::grammar::function::expression::expression;
use crate::grammar::function_type_specification::function_type_specification;
use crate::grammar::nodes::AstNode;
use crate::grammar::nodes::InnerNode;
use crate::grammar::nodes::LeafNode;
use crate::grammar::nodes::Node;
use crate::grammar::r#type::type_blacklisted::type_blacklisted;
use crate::grammar::r#type::type_whitelisted::type_whitelisted;
use rand::distr::Bernoulli;
use rand::distr::Distribution;

pub fn function(attributes: &mut Attributes) -> AstNode {
    attributes.is_main_func =
        !attributes.main_func_generated && Bernoulli::new(0.3).unwrap().sample(&mut rand::rng());
    attributes.current_params = ParamMap::new();
    attributes.current_vars = VarMap::new();
    attributes.current_var_id = 0;
    let mut children: Vec<AstNode> = vec![];
    let return_value = if attributes.is_main_func {
        type_whitelisted(attributes, vec!["Int".to_string()], 0, 1)
    } else {
        type_blacklisted(attributes, vec!["Nothing".to_string()], 0, 1)
    };
    attributes.type_context.push(return_value.token.clone());
    children.push(Node::Leaf(LeafNode {
        tabs: 0,
        token: "fn ".to_string(),
        new_lines: 0,
    }));
    let function_name = if attributes.is_main_func {
        "main".to_string()
    } else {
        let id = attributes.function_id;
        attributes.function_id += 1;
        format!("function{}", id)
    };
    children.push(Node::Leaf(LeafNode {
        tabs: 0,
        token: function_name,
        new_lines: 0,
    }));
    children.push(function_type_specification(attributes));
    children.push(Node::Leaf(return_value));
    attributes.is_end_expression = true;
    attributes.is_start_expression = true;
    attributes.tab_level += 1;
    children.push(expression(attributes));
    attributes.tab_level -= 1;
    attributes.is_start_expression = false;
    attributes.is_end_expression = false;

    children.push(Node::Leaf(LeafNode {
        tabs: 0,
        token: "end".to_string(),
        new_lines: 2,
    }));
    attributes.main_func_generated = attributes.main_func_generated || attributes.is_main_func;
    attributes.is_main_func = false;
    attributes.type_context.pop();
    Node::Inner(InnerNode {
        // tab_level: 0,
        children: children,
    })
}
