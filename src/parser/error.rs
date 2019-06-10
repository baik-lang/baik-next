use crate::parser::grammar::Rule;
use crate::parser::input_location::InputLocation;
use pest;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum ParseError {
    PestError {
        positives: Vec<Rule>,
        negatives: Vec<Rule>,
        location: InputLocation,
    },
    AstGeneration {
        rule: Rule,
        location: InputLocation,
    },
}

impl<'a> From<pest::error::Error<Rule>> for ParseError {
    fn from(pest: pest::error::Error<Rule>) -> Self {
        match pest.variant {
            pest::error::ErrorVariant::ParsingError {
                ref positives,
                ref negatives,
            } => {
                // FIXME: Remove when we have real error formatting.
                println!("Pest Error: {}", pest);
                ParseError::PestError {
                    positives: positives.clone(),
                    negatives: negatives.clone(),
                    location: InputLocation::from(pest.location),
                }
            }
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<'a> Error for ParseError {}
