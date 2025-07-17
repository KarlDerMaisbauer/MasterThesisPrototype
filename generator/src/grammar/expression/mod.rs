mod bracket_expression;
pub mod expression;
mod function_call_expression;
mod infix_expression;
mod let_expression;
mod match_expression;
mod member_access_expression;
mod prefix_expression;
mod struct_constructor_expression;
mod union_constructor_expression;
mod var_call_expression;

use crate::grammar::attributes::Attributes;
use crate::grammar::expression::bracket_expression::*;
use crate::grammar::expression::function_call_expression::*;
use crate::grammar::expression::infix_expression::*;
use crate::grammar::expression::let_expression::*;
use crate::grammar::expression::match_expression::*;
use crate::grammar::expression::member_access_expression::*;
use crate::grammar::expression::prefix_expression::*;
use crate::grammar::expression::struct_constructor_expression::*;
use crate::grammar::expression::union_constructor_expression::*;
use crate::grammar::expression::var_call_expression::*;
use crate::grammar::literal::*;
use crate::grammar::nodes::AstNode;
use std::sync::LazyLock;

type Acceptor = fn(&Attributes) -> bool;
type Expression = fn(&mut Attributes) -> AstNode;

static EXPRESSIONS: LazyLock<Vec<(Acceptor, Expression, f64)>> = LazyLock::new(|| {
    vec![
        (literal_guard, literal, 1f64),
        (bracket_expression_guard, bracket_expression, 1f64),
        (prefix_expression_guard, prefix_expression, 1f64),
        (var_call_expression_guard, var_call_expression, 1f64),
        (match_expression_guard, match_expression, 1f64),
        (
            function_call_expression_guard,
            function_call_expression,
            1f64,
        ),
        (
            member_access_expression_guard,
            member_access_expression,
            1f64,
        ),
        (let_expression_quard, let_expression, 2f64),
        (infix_expression_guard, infix_expression, 2f64),
        (
            struct_constructor_expression_guard,
            struct_constructor_expression,
            2f64,
        ),
        (
            union_constructor_expression_guard,
            union_constructor_expression,
            2f64,
        ),
    ]
});
