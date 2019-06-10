use crate::parser::ast::{Location, Value};
use crate::parser::grammar::Rule;
use crate::parser::input_location::InputLocation;
use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq)]
pub struct Boolean {
    value: bool,
    location: InputLocation,
}

impl Value for Boolean {
    type Item = bool;

    fn value(self) -> Self::Item {
        self.value
    }

    fn value_ref(&self) -> &Self::Item {
        &self.value
    }
}

impl Location for Boolean {
    fn location(&self) -> &InputLocation {
        &self.location
    }
}

impl<'a> From<Pair<'a, Rule>> for Boolean {
    fn from(pair: Pair<'a, Rule>) -> Self {
        match pair.as_rule() {
            Rule::boolean => {
                let inner = pair.into_inner().next().unwrap();

                match inner.as_rule() {
                    Rule::boolean_true => Boolean {
                        value: true,
                        location: InputLocation::from(inner.as_span()),
                    },
                    Rule::boolean_false => Boolean {
                        value: false,
                        location: InputLocation::from(inner.as_span()),
                    },
                    _ => unreachable!("Unexpected {:?} token inside boolean", inner),
                }
            }
            _ => unreachable!("Expected pair to be an Boolean"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::grammar::BaikLexer;
    use pest::Parser;

    #[test]
    fn test_true() {
        let pair = BaikLexer::parse(Rule::boolean, "benar")
            .unwrap()
            .next()
            .unwrap();
        let boolean = Boolean::from(pair);
        assert_eq!(boolean.value(), true);
    }

    #[test]
    fn test_false() {
        let pair = BaikLexer::parse(Rule::boolean, "salah")
            .unwrap()
            .next()
            .unwrap();
        let boolean = Boolean::from(pair);
        assert_eq!(boolean.value(), false);
    }
}
