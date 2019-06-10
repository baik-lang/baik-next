use crate::parser::ast::{Location, Ty, Value};
use crate::parser::grammar::Rule;
use crate::parser::input_location::InputLocation;
use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq)]
pub struct TypeSpec {
    value: Vec<Ty>,
    location: InputLocation,
}

impl Value for TypeSpec {
    type Item = Vec<Ty>;

    fn value(self) -> Self::Item {
        self.value
    }

    fn value_ref(&self) -> &Self::Item {
        &self.value
    }
}

impl Location for TypeSpec {
    fn location(&self) -> &InputLocation {
        &self.location
    }
}

impl<'a> From<Pair<'a, Rule>> for TypeSpec {
    fn from(pair: Pair<'a, Rule>) -> Self {
        match pair.as_rule() {
            Rule::typespec => {
                let value = pair.clone().into_inner().map(Ty::from).collect();
                TypeSpec {
                    value,
                    location: InputLocation::from(pair.as_span()),
                }
            }
            _ => unreachable!(
                "Expected pair to be an TypeSpec but was a {:?}",
                pair.as_rule()
            ),
        }
    }
}
