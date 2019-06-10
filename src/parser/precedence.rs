use crate::parser::ast::*;
use crate::parser::grammar::Rule;

use pest::{
    iterators::Pair,
    prec_climber::{Assoc, Operator, PrecClimber},
};

lazy_static! {
    pub static ref PREC_CLIMBER: PrecClimber<Rule> = build_precedence_climber();
}

fn build_precedence_climber() -> PrecClimber<Rule> {
    PrecClimber::new(vec![
        Operator::new(Rule::logical_or, Assoc::Left),
        Operator::new(Rule::logical_and, Assoc::Left),
        Operator::new(Rule::equal, Assoc::Right) | Operator::new(Rule::not_equal, Assoc::Right),
        Operator::new(Rule::greater_than_or_equal, Assoc::Left)
            | Operator::new(Rule::less_than_or_equal, Assoc::Left)
            | Operator::new(Rule::greater_than, Assoc::Left)
            | Operator::new(Rule::less_than, Assoc::Left),
        Operator::new(Rule::bitwise_xor, Assoc::Left)
            | Operator::new(Rule::bitwise_or, Assoc::Left),
        Operator::new(Rule::bitwise_and, Assoc::Left),
        Operator::new(Rule::shift_right, Assoc::Left)
            | Operator::new(Rule::shift_left, Assoc::Left),
        Operator::new(Rule::plus, Assoc::Left) | Operator::new(Rule::minus, Assoc::Left),
        Operator::new(Rule::modulus, Assoc::Left)
            | Operator::new(Rule::divide, Assoc::Left)
            | Operator::new(Rule::multiply, Assoc::Left),
        Operator::new(Rule::exponent, Assoc::Right),
    ])
}

fn build_binary(lhs: Term, op: Pair<'_, Rule>, rhs: Term) -> Term {
    let binary = Binary::from(op);
    Term::create_infix(binary, lhs, rhs)
}

pub fn climb(pair: Pair<'_, Rule>) -> Term {
    PREC_CLIMBER.climb(pair.into_inner(), Term::from, build_binary)
}
