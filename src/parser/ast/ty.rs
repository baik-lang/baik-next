use crate::parser::ast::{Location, Value};
use crate::parser::grammar::Rule;
use crate::parser::input_location::InputLocation;
use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq)]
pub struct Ty {
    value: String,
    location: InputLocation,
}

impl Ty {
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl Value for Ty {
    type Item = String;

    fn value(self) -> Self::Item {
        self.value
    }

    fn value_ref(&self) -> &Self::Item {
        &self.value
    }
}

impl Location for Ty {
    fn location(&self) -> &InputLocation {
        &self.location
    }
}

impl<'a> From<Pair<'a, Rule>> for Ty {
    fn from(pair: Pair<'a, Rule>) -> Self {
        match pair.as_rule() {
            Rule::typename => {
                let value = pair.clone().as_span().as_str().to_string();
                Ty {
                    value,
                    location: InputLocation::from(pair.as_span()),
                }
            }
            _ => unreachable!("Expected pair to be a Ty (received {:?})", pair.as_rule()),
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
        let pair = BaikLexer::parse(Rule::typename, "Baik.Lang")
            .unwrap()
            .next()
            .unwrap();
        let r#type = Ty::from(pair);
        assert_eq!(r#type.value(), "Baik.Lang");
    }
}
