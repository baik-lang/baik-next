use crate::parser::ast::{Location, Value};
use crate::parser::grammar::Rule;
use crate::parser::input_location::InputLocation;
use pest::iterators::Pair;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Unary {
    value: Operator,
    location: InputLocation,
}

impl Unary {
    pub fn is_arithmetic(&self) -> bool {
        self.value.is_arithmetic()
    }

    pub fn is_logical(&self) -> bool {
        self.value.is_logical()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    LogicalNot,
    Minus,
    Plus,
}

impl Operator {
    pub fn is_arithmetic(&self) -> bool {
        match self {
            Operator::Minus => true,
            Operator::Plus => true,
            _ => false,
        }
    }

    pub fn is_logical(&self) -> bool {
        match self {
            Operator::LogicalNot => true,
            _ => false,
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operator::LogicalNot => write!(f, "!"),
            Operator::Plus => write!(f, "+"),
            Operator::Minus => write!(f, "-"),
        }
    }
}

impl Value for Unary {
    type Item = Operator;

    fn value(self) -> Self::Item {
        self.value
    }

    fn value_ref(&self) -> &Self::Item {
        &self.value
    }
}

impl Location for Unary {
    fn location(&self) -> &InputLocation {
        &self.location
    }
}

impl<'a> From<Pair<'a, Rule>> for Unary {
    fn from(pair: Pair<'a, Rule>) -> Self {
        match pair.as_rule() {
            Rule::logical_not => Unary {
                value: Operator::LogicalNot,
                location: InputLocation::from(pair.as_span()),
            },
            Rule::minus => Unary {
                value: Operator::Minus,
                location: InputLocation::from(pair.as_span()),
            },
            Rule::plus => Unary {
                value: Operator::Plus,
                location: InputLocation::from(pair.as_span()),
            },
            _ => unreachable!("Expected pair to be an Unary"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::grammar::BaikLexer;
    use pest::Parser;

    #[test]
    fn it_parses_logical_not() {
        let pair = BaikLexer::parse(Rule::unary_operator, "!")
            .unwrap()
            .next()
            .unwrap();
        let unary = Unary::from(pair);
        assert_eq!(unary.value, Operator::LogicalNot);
    }

    #[test]
    fn it_parses_minus() {
        let pair = BaikLexer::parse(Rule::unary_operator, "-")
            .unwrap()
            .next()
            .unwrap();
        let unary = Unary::from(pair);
        assert_eq!(unary.value, Operator::Minus);
    }

    #[test]
    fn it_parses_plus() {
        let pair = BaikLexer::parse(Rule::unary_operator, "+")
            .unwrap()
            .next()
            .unwrap();
        let unary = Unary::from(pair);
        assert_eq!(unary.value, Operator::Plus);
    }

    #[test]
    fn it_parses_braced() {
        let pair = BaikLexer::parse(Rule::unary_operator, "+(1 + 3)")
            .unwrap()
            .next()
            .unwrap();
        let unary = Unary::from(pair);
        assert_eq!(unary.value, Operator::Plus)
    }
}
