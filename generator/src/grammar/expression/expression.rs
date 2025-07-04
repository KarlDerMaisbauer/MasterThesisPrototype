use crate::grammar::attributes::Attributes;
use crate::grammar::nodes::AstNode;
use rand::distr::Distribution;
use rand::distr::weighted::WeightedIndex;

use crate::grammar::expression::EXPRESSIONS;
use crate::grammar::expression::Expression;

pub fn expression(attributes: &mut Attributes) -> AstNode {
    let expression = &*EXPRESSIONS
        .iter()
        .filter(|&&(guard, _, _)| guard(attributes))
        .map(|&(_, expr, weight)| (expr, weight))
        .collect::<Vec<(Expression, f64)>>();
    choose_expression(&expression.to_vec())(attributes)
}

fn choose_expression(expressions: &Vec<(Expression, f64)>) -> Expression {
    let (expr, weights): (Vec<Expression>, Vec<f64>) = expressions.clone().into_iter().unzip();
    let weights_normalized: Vec<f64> = weights.iter().map(|w| (1f64 / w)).collect();
    let dist = WeightedIndex::new(weights_normalized).ok().unwrap();
    let mut rng = rand::rng();
    let index = dist.sample(&mut rng);
    expr[index]
}
