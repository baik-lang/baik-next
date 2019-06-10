use crate::parser::ast::{Location, Value};
use crate::parser::grammar::Rule;
use crate::parser::input_location::InputLocation;
use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq)]
pub struct Local {
    value: String,
    location: InputLocation,
}

impl Value for Local {
    type Item = String;

    fn value(self) -> Self::Item {
        self.value
    }

    fn value_ref(&self) -> &Self::Item {
        &self.value
    }
}

impl Location for Local {
    fn location(&self) -> &InputLocation {
        &self.location
    }
}

impl<'a> From<Pair<'a, Rule>> for Local {
    fn from(pair: Pair<'a, Rule>) -> Self {
        match pair.as_rule() {
            Rule::local => {
                let value = pair.clone().as_span().as_str().to_string();
                Local {
                    value,
                    location: InputLocation::from(pair.as_span()),
                }
            }
            _ => unreachable!("Expected pair to be an Local"),
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
        let pair = BaikLexer::parse(Rule::local, "marty_lang")
            .unwrap()
            .next()
            .unwrap();
        let local = Local::from(pair);
        assert_eq!(local.value(), "marty_lang");
    }
}
