use crate::parser::ast::{Location, Value};
use crate::parser::grammar::Rule;
use crate::parser::input_location::InputLocation;
use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    value: String,
    location: InputLocation,
    has_predicate: bool,
}

impl Identifier {
    pub fn has_predicate(&self) -> bool {
        self.has_predicate
    }
}

impl Value for Identifier {
    type Item = String;

    fn value(self) -> Self::Item {
        self.value
    }

    fn value_ref(&self) -> &Self::Item {
        &self.value
    }
}

impl Location for Identifier {
    fn location(&self) -> &InputLocation {
        &self.location
    }
}

impl<'a> From<Pair<'a, Rule>> for Identifier {
    fn from(pair: Pair<'a, Rule>) -> Self {
        match pair.as_rule() {
            Rule::ident => {
                let value = pair.clone().as_span().as_str().to_string();
                Identifier {
                    value,
                    location: InputLocation::from(pair.as_span()),
                    has_predicate: false,
                }
            }
            Rule::keyword => {
                let value = pair.clone().as_span().as_str();
                let len = value.len();
                Identifier {
                    value: value.get(0..len - 1).unwrap().to_string(),
                    location: InputLocation::from(pair.as_span()),
                    has_predicate: false,
                }
            }
            Rule::methodname => {
                let value = pair.clone().as_span().as_str().to_string();
                Identifier {
                    value,
                    location: InputLocation::from(pair.as_span()),
                    has_predicate: false,
                }
            }
            Rule::methodnamewithpredicate => {
                let value = pair.clone().as_span().as_str().to_string();
                Identifier {
                    value,
                    location: InputLocation::from(pair.as_span()),
                    has_predicate: true,
                }
            }
            Rule::property_get => {
                let value = pair.clone().as_span().as_str();
                let len = value.len();
                let value = value.get(1..len).unwrap().to_string();
                Identifier {
                    value,
                    location: InputLocation::from(pair.as_span()),
                    has_predicate: false,
                }
            }
            _ => unreachable!(
                "Expected pair to be an Identifier (received {:?})",
                pair.as_rule()
            ),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::grammar::BaikLexer;
    use pest::Parser;

    #[test]
    fn it_parses() {
        let pair = BaikLexer::parse(Rule::ident, "marty_lang")
            .unwrap()
            .next()
            .unwrap();
        let ident = Identifier::from(pair);
        assert_eq!(ident.value(), "marty_lang");
    }
}
