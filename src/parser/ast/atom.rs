use crate::parser::ast::{Location, Value};
use crate::parser::grammar::Rule;
use crate::parser::input_location::InputLocation;
use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq)]
pub struct Atom {
    value: String,
    location: InputLocation,
}

impl Value for Atom {
    type Item = String;

    fn value(self) -> Self::Item {
        self.value
    }

    fn value_ref(&self) -> &Self::Item {
        &self.value
    }
}

impl Location for Atom {
    fn location(&self) -> &InputLocation {
        &self.location
    }
}

impl<'a> From<Pair<'a, Rule>> for Atom {
    fn from(pair: Pair<'a, Rule>) -> Self {
        match pair.as_rule() {
            Rule::atom => {
                let value = pair.clone().as_span().as_str();
                let len = value.len();
                let value = value.get(1..len).unwrap().to_string();
                Atom {
                    value,
                    location: InputLocation::from(pair.as_span()),
                }
            }
            Rule::keyword => {
                let value = pair.clone().as_span().as_str();
                let len = value.len();
                let value = value.get(0..len - 1).unwrap().to_string();
                Atom {
                    value,
                    location: InputLocation::from(pair.as_span()),
                }
            }
            Rule::ident => {
                let value = pair.clone().as_span().as_str().to_string();
                Atom {
                    value,
                    location: InputLocation::from(pair.as_span()),
                }
            }
            _ => unreachable!("Expected pair to be an Atom"),
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
        let pair = BaikLexer::parse(Rule::atom, ":marty_lang")
            .unwrap()
            .next()
            .unwrap();
        let atom = Atom::from(pair);
        assert_eq!(atom.value(), "marty_lang");
    }
}
