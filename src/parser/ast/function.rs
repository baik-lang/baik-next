use crate::parser::ast::{Identifier, Location, Term, TypeSpec, Value};
use crate::parser::grammar::Rule;
use crate::parser::input_location::InputLocation;
use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    clauses: Vec<Clause>,
    location: InputLocation,
}

impl Value for Function {
    type Item = Vec<Clause>;

    fn value(self) -> Self::Item {
        self.clauses
    }

    fn value_ref(&self) -> &Self::Item {
        &self.clauses
    }
}

impl Location for Function {
    fn location(&self) -> &InputLocation {
        &self.location
    }
}

impl<'a> From<Pair<'a, Rule>> for Function {
    fn from(pair: Pair<'a, Rule>) -> Self {
        match pair.as_rule() {
            Rule::function => {
                let clauses = pair.clone().into_inner().map(Clause::from).collect();
                let location = InputLocation::from(pair.as_span());
                Function { clauses, location }
            }
            _ => unreachable!("Expected pair to be a Function"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Clause {
    arguments: Vec<(Identifier, TypeSpec)>,
    body: Vec<Term>,
    location: InputLocation,
}

impl Clause {
    pub fn arguments(&self) -> &Vec<(Identifier, TypeSpec)> {
        &self.arguments
    }

    pub fn body(&self) -> &Vec<Term> {
        &self.body
    }
}

impl Location for Clause {
    fn location(&self) -> &InputLocation {
        &self.location
    }
}

impl<'a> From<Pair<'a, Rule>> for Clause {
    fn from(pair: Pair<'a, Rule>) -> Self {
        match pair.as_rule() {
            Rule::function_clause => {
                let mut inner = pair.clone().into_inner();
                let arguments = inner
                    .next()
                    .unwrap()
                    .into_inner()
                    .map(|p| {
                        let mut inner = p.into_inner();
                        let keyword = Identifier::from(inner.next().unwrap());
                        let typespec = TypeSpec::from(inner.next().unwrap());
                        (keyword, typespec)
                    })
                    .collect();
                let body = inner.next().unwrap().into_inner().map(Term::from).collect();
                let location = InputLocation::from(pair);
                Clause {
                    arguments,
                    body,
                    location,
                }
            }
            _ => unreachable!("Expected pair to be a Function Clause"),
        }
    }
}
