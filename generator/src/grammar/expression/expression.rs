use crate::grammar::attributes::Attributes;
use crate::grammar::nodes::AstNode;
use rand::seq::IndexedRandom;

use crate::grammar::expression::EXPRESSIONS;
use crate::grammar::expression::Expression;

pub fn expression(attributes: &mut Attributes) -> AstNode {
    let expression = &*EXPRESSIONS
        .iter()
        .filter(|&&(guard, _, _)| guard(attributes))
        .map(|&(_, expr, weight)| (expr, weight))
        .collect::<Vec<(Expression, f64)>>();
    choose_expression(&expression.to_vec(), attributes)(attributes)
}

fn choose_expression(
    expressions: &Vec<(Expression, f64)>,
    attributes: &mut Attributes,
) -> Expression {
    let (expr, _weights): (Vec<Expression>, Vec<f64>) = expressions.clone().into_iter().unzip();
    // let weights_normalized: Vec<f64> = weights.iter().map(|w| (1f64 / w)).collect();
    // // let dist = WeightedIndex::new(weights_normalized).ok().unwrap();
    // let dist = match WeightedIndex::new(weights_normalized) {
    //     // Ok(val) => match val {
    //     //     Some(d) => d,
    //     //     None => panic!("none returned for dist"),
    //     // },
    //     Ok(val) => val,
    //     Err(e) => {
    //         println!("An error occurred: {}", e);
    //         panic!("Operation failed");
    //     }
    // };
    if expr.len() == 0 {
        println!(
            "choose expr context: {}",
            attributes.type_context.last().unwrap()
        );
        panic!("expression vector empty this should not happen")
    }
    let mut rng = rand::rng();
    // let index = dist.sample(&mut rng);
    // expr[index]
    expr.choose(&mut rng).unwrap().clone()
}
