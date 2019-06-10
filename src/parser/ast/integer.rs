use crate::parser::ast::{Location, Value};
use crate::parser::grammar::Rule;
use crate::parser::input_location::InputLocation;
use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq)]
pub struct Integer {
    value: i64,
    radix: usize,
    location: InputLocation,
}

impl Integer {
    pub fn radix(&self) -> usize {
        self.radix
    }
}

impl Value for Integer {
    type Item = i64;

    fn value(self) -> Self::Item {
        self.value
    }

    fn value_ref(&self) -> &Self::Item {
        &self.value
    }
}

impl Location for Integer {
    fn location(&self) -> &InputLocation {
        &self.location
    }
}

impl<'a> From<Pair<'a, Rule>> for Integer {
    fn from(pair: Pair<'a, Rule>) -> Self {
        match pair.as_rule() {
            Rule::integer => {
                let inner = pair.clone().into_inner().next().unwrap();
                let radix = match inner.as_rule() {
                    Rule::integer_decimal => 10,
                    Rule::integer_hexadecimal => 16,
                    Rule::integer_octal => 8,
                    Rule::integer_binary => 2,
                    Rule::integer_zero => 10,
                    _ => unreachable!("Expected pair to be a type of Integer"),
                };
                let value = inner.as_span().as_str().replace("_", "");
                Integer {
                    value: i64::from_str_radix(&value, radix).unwrap(),
                    radix: radix as usize,
                    location: InputLocation::from(pair.as_span()),
                }
            }
            _ => unreachable!("Expected pair to be an Integer"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::grammar::BaikLexer;
    use pest::Parser;

    #[test]
    fn decimal() {
        let pair = BaikLexer::parse(Rule::integer, "123")
            .unwrap()
            .next()
            .unwrap();
        let integer = Integer::from(pair);
        assert_eq!(integer.value, 123);
        assert_eq!(integer.radix, 10);
    }

    #[test]
    fn hexadecimal() {
        let pair = BaikLexer::parse(Rule::integer, "0x123")
            .unwrap()
            .next()
            .unwrap();
        let integer = Integer::from(pair);
        assert_eq!(integer.value, 291);
        assert_eq!(integer.radix, 16);
    }

    #[test]
    fn octal() {
        let pair = BaikLexer::parse(Rule::integer, "0o123")
            .unwrap()
            .next()
            .unwrap();
        let integer = Integer::from(pair);
        assert_eq!(integer.value, 83);
        assert_eq!(integer.radix, 8);
    }

    #[test]
    fn binary() {
        let pair = BaikLexer::parse(Rule::integer, "0b0101")
            .unwrap()
            .next()
            .unwrap();
        let integer = Integer::from(pair);
        assert_eq!(integer.value, 5);
        assert_eq!(integer.radix, 2);
    }
    #[test]
    fn zero() {
        let pair = BaikLexer::parse(Rule::integer, "0").unwrap().next().unwrap();
        let integer = Integer::from(pair);
        assert_eq!(integer.value, 0);
        assert_eq!(integer.radix, 10);
    }
}
