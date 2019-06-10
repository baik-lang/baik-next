use crate::parser::ast::{Location, Value};
use crate::parser::grammar::Rule;
use crate::parser::input_location::InputLocation;
use pest::iterators::Pair;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Binary {
    value: Operator,
    location: InputLocation,
}

impl Binary {
    pub fn is_arithmetic(&self) -> bool {
        self.value.is_arithmetic()
    }

    pub fn is_logical(&self) -> bool {
        self.value.is_logical()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    Divide,
    Equal,
    Exponent,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    LogicalAnd,
    LogicalOr,
    Minus,
    Modulus,
    Multiply,
    NotEqual,
    Plus,
    ShiftLeft,
    ShiftRight,
}

impl Operator {
    pub fn is_arithmetic(&self) -> bool {
        match self {
            Operator::BitwiseAnd => true,
            Operator::BitwiseOr => true,
            Operator::BitwiseXor => true,
            Operator::Divide => true,
            Operator::Exponent => true,
            Operator::Minus => true,
            Operator::Modulus => true,
            Operator::Multiply => true,
            Operator::Plus => true,
            Operator::ShiftLeft => true,
            Operator::ShiftRight => true,
            _ => false,
        }
    }

    pub fn is_logical(&self) -> bool {
        match self {
            Operator::Equal => true,
            Operator::GreaterThan => true,
            Operator::GreaterThanOrEqual => true,
            Operator::LessThan => true,
            Operator::LessThanOrEqual => true,
            Operator::LogicalAnd => true,
            Operator::LogicalOr => true,
            Operator::NotEqual => true,
            _ => false,
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operator::BitwiseAnd => write!(f, "&"),
            Operator::BitwiseOr => write!(f, "|"),
            Operator::BitwiseXor => write!(f, "^"),
            Operator::Divide => write!(f, "/"),
            Operator::Equal => write!(f, "=="),
            Operator::Exponent => write!(f, "**"),
            Operator::GreaterThan => write!(f, ">"),
            Operator::GreaterThanOrEqual => write!(f, ">="),
            Operator::LessThan => write!(f, "<"),
            Operator::LessThanOrEqual => write!(f, "<="),
            Operator::LogicalAnd => write!(f, "&&"),
            Operator::LogicalOr => write!(f, "||"),
            Operator::Minus => write!(f, "-"),
            Operator::Modulus => write!(f, "%"),
            Operator::Multiply => write!(f, "*"),
            Operator::NotEqual => write!(f, "!="),
            Operator::Plus => write!(f, "+"),
            Operator::ShiftLeft => write!(f, ">>"),
            Operator::ShiftRight => write!(f, "<<"),
        }
    }
}

impl Value for Binary {
    type Item = Operator;

    fn value(self) -> Self::Item {
        self.value
    }

    fn value_ref(&self) -> &Self::Item {
        &self.value
    }
}

impl Location for Binary {
    fn location(&self) -> &InputLocation {
        &self.location
    }
}

impl<'a> From<Pair<'a, Rule>> for Binary {
    fn from(pair: Pair<'a, Rule>) -> Self {
        match pair.as_rule() {
            Rule::bitwise_and => Binary {
                value: Operator::BitwiseAnd,
                location: InputLocation::from(pair.as_span()),
            },
            Rule::bitwise_or => Binary {
                value: Operator::BitwiseOr,
                location: InputLocation::from(pair.as_span()),
            },
            Rule::bitwise_xor => Binary {
                value: Operator::BitwiseXor,
                location: InputLocation::from(pair.as_span()),
            },
            Rule::divide => Binary {
                value: Operator::Divide,
                location: InputLocation::from(pair.as_span()),
            },
            Rule::equal => Binary {
                value: Operator::Equal,
                location: InputLocation::from(pair.as_span()),
            },
            Rule::exponent => Binary {
                value: Operator::Exponent,
                location: InputLocation::from(pair.as_span()),
            },
            Rule::greater_than => Binary {
                value: Operator::GreaterThan,
                location: InputLocation::from(pair.as_span()),
            },
            Rule::greater_than_or_equal => Binary {
                value: Operator::GreaterThanOrEqual,
                location: InputLocation::from(pair.as_span()),
            },
            Rule::less_than => Binary {
                value: Operator::LessThan,
                location: InputLocation::from(pair.as_span()),
            },
            Rule::less_than_or_equal => Binary {
                value: Operator::LessThanOrEqual,
                location: InputLocation::from(pair.as_span()),
            },
            Rule::logical_and => Binary {
                value: Operator::LogicalAnd,
                location: InputLocation::from(pair.as_span()),
            },
            Rule::logical_or => Binary {
                value: Operator::LogicalOr,
                location: InputLocation::from(pair.as_span()),
            },
            Rule::minus => Binary {
                value: Operator::Minus,
                location: InputLocation::from(pair.as_span()),
            },
            Rule::modulus => Binary {
                value: Operator::Modulus,
                location: InputLocation::from(pair.as_span()),
            },
            Rule::multiply => Binary {
                value: Operator::Multiply,
                location: InputLocation::from(pair.as_span()),
            },
            Rule::not_equal => Binary {
                value: Operator::NotEqual,
                location: InputLocation::from(pair.as_span()),
            },
            Rule::plus => Binary {
                value: Operator::Plus,
                location: InputLocation::from(pair.as_span()),
            },
            Rule::shift_left => Binary {
                value: Operator::ShiftLeft,
                location: InputLocation::from(pair.as_span()),
            },
            Rule::shift_right => Binary {
                value: Operator::ShiftRight,
                location: InputLocation::from(pair.as_span()),
            },
            _ => unreachable!("Expected pair to be an Binary"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::grammar::BaikLexer;
    use pest::Parser;

    #[test]
    fn it_parses_bitwise_and() {
        let pair = BaikLexer::parse(Rule::binary_operator, "&")
            .unwrap()
            .next()
            .unwrap();
        let binary = Binary::from(pair);
        assert_eq!(binary.value(), Operator::BitwiseAnd);
    }

    #[test]
    fn it_parses_bitwise_or() {
        let pair = BaikLexer::parse(Rule::binary_operator, "|")
            .unwrap()
            .next()
            .unwrap();
        let binary = Binary::from(pair);
        assert_eq!(binary.value(), Operator::BitwiseOr);
    }

    #[test]
    fn it_parses_bitwise_xor() {
        let pair = BaikLexer::parse(Rule::binary_operator, "^")
            .unwrap()
            .next()
            .unwrap();
        let binary = Binary::from(pair);
        assert_eq!(binary.value(), Operator::BitwiseXor);
    }

    #[test]
    fn it_parses_divide() {
        let pair = BaikLexer::parse(Rule::binary_operator, "/")
            .unwrap()
            .next()
            .unwrap();
        let binary = Binary::from(pair);
        assert_eq!(binary.value(), Operator::Divide);
    }

    #[test]
    fn it_parses_equal() {
        let pair = BaikLexer::parse(Rule::binary_operator, "==")
            .unwrap()
            .next()
            .unwrap();
        let binary = Binary::from(pair);
        assert_eq!(binary.value(), Operator::Equal);
    }

    #[test]
    fn it_parses_exponent() {
        let pair = BaikLexer::parse(Rule::binary_operator, "**")
            .unwrap()
            .next()
            .unwrap();
        let binary = Binary::from(pair);
        assert_eq!(binary.value(), Operator::Exponent);
    }

    #[test]
    fn it_parses_greater_than() {
        let pair = BaikLexer::parse(Rule::binary_operator, ">")
            .unwrap()
            .next()
            .unwrap();
        let binary = Binary::from(pair);
        assert_eq!(binary.value(), Operator::GreaterThan);
    }

    #[test]
    fn it_parses_greater_than_or_equal() {
        let pair = BaikLexer::parse(Rule::binary_operator, ">=")
            .unwrap()
            .next()
            .unwrap();
        let binary = Binary::from(pair);
        assert_eq!(binary.value(), Operator::GreaterThanOrEqual);
    }

    #[test]
    fn it_parses_less_than() {
        let pair = BaikLexer::parse(Rule::binary_operator, "<")
            .unwrap()
            .next()
            .unwrap();
        let binary = Binary::from(pair);
        assert_eq!(binary.value(), Operator::LessThan);
    }

    #[test]
    fn it_parses_less_than_or_equal() {
        let pair = BaikLexer::parse(Rule::binary_operator, "<=")
            .unwrap()
            .next()
            .unwrap();
        let binary = Binary::from(pair);
        assert_eq!(binary.value(), Operator::LessThanOrEqual);
    }

    #[test]
    fn it_parses_logical_and() {
        let pair = BaikLexer::parse(Rule::binary_operator, "&&")
            .unwrap()
            .next()
            .unwrap();
        let binary = Binary::from(pair);
        assert_eq!(binary.value(), Operator::LogicalAnd);
    }

    #[test]
    fn it_parses_logical_or() {
        let pair = BaikLexer::parse(Rule::binary_operator, "||")
            .unwrap()
            .next()
            .unwrap();
        let binary = Binary::from(pair);
        assert_eq!(binary.value(), Operator::LogicalOr);
    }

    #[test]
    fn it_parses_minus() {
        let pair = BaikLexer::parse(Rule::binary_operator, "-")
            .unwrap()
            .next()
            .unwrap();
        let binary = Binary::from(pair);
        assert_eq!(binary.value(), Operator::Minus);
    }

    #[test]
    fn it_parses_modulus() {
        let pair = BaikLexer::parse(Rule::binary_operator, "%")
            .unwrap()
            .next()
            .unwrap();
        let binary = Binary::from(pair);
        assert_eq!(binary.value(), Operator::Modulus);
    }

    #[test]
    fn it_parses_multiply() {
        let pair = BaikLexer::parse(Rule::binary_operator, "*")
            .unwrap()
            .next()
            .unwrap();
        let binary = Binary::from(pair);
        assert_eq!(binary.value(), Operator::Multiply);
    }

    #[test]
    fn it_parses_not_equal() {
        let pair = BaikLexer::parse(Rule::binary_operator, "!=")
            .unwrap()
            .next()
            .unwrap();
        let binary = Binary::from(pair);
        assert_eq!(binary.value(), Operator::NotEqual);
    }

    #[test]
    fn it_parses_plus() {
        let pair = BaikLexer::parse(Rule::binary_operator, "+")
            .unwrap()
            .next()
            .unwrap();
        let binary = Binary::from(pair);
        assert_eq!(binary.value(), Operator::Plus);
    }

    #[test]
    fn it_parses_shift_left() {
        let pair = BaikLexer::parse(Rule::binary_operator, "<<")
            .unwrap()
            .next()
            .unwrap();
        let binary = Binary::from(pair);
        assert_eq!(binary.value(), Operator::ShiftLeft);
    }

    #[test]
    fn it_parses_shift_right() {
        let pair = BaikLexer::parse(Rule::binary_operator, ">>")
            .unwrap()
            .next()
            .unwrap();
        let binary = Binary::from(pair);
        assert_eq!(binary.value(), Operator::ShiftRight);
    }
}
