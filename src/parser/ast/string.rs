use crate::parser::ast::{Location, Value};
use crate::parser::grammar::Rule;
use crate::parser::input_location::InputLocation;
use pest::iterators::Pair;
use std::string;

#[derive(Debug, Clone, PartialEq)]
pub struct String {
    value: string::String,
    location: InputLocation,
}

impl<'a> From<Pair<'a, Rule>> for String {
    fn from(pair: Pair<'a, Rule>) -> Self {
        match pair.as_rule() {
            Rule::string => {
                let value = pair
                    .clone()
                    .into_inner()
                    .next()
                    .unwrap()
                    .as_span()
                    .as_str()
                    .to_string();
                String {
                    value,
                    location: InputLocation::from(pair.as_span()),
                }
            }
            _ => unreachable!("Expected pair to be an String"),
        }
    }
}

impl Value for String {
    type Item = string::String;

    fn value(self) -> Self::Item {
        self.value
    }

    fn value_ref(&self) -> &Self::Item {
        &self.value
    }
}

impl Location for String {
    fn location(&self) -> &InputLocation {
        &self.location
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::grammar::BaikLexer;
    use pest::Parser;

    #[test]
    fn it_parses() {
        let pair = BaikLexer::parse(Rule::string, "\"Baik Lang\"")
            .unwrap()
            .next()
            .unwrap();
        let string = String::from(pair);
        assert_eq!(string.value, "Baik Lang");
    }
}
