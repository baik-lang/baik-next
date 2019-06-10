use crate::parser::grammar::Rule;
use pest;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum InputLocation {
    Pos(usize),
    Span(usize, usize),
}

impl InputLocation {
    pub fn pos(pos: usize) -> InputLocation {
        InputLocation::Pos(pos)
    }

    pub fn span(start: usize, end: usize) -> InputLocation {
        InputLocation::Span(start, end)
    }
}

impl From<pest::error::InputLocation> for InputLocation {
    fn from(pest: pest::error::InputLocation) -> InputLocation {
        match pest {
            pest::error::InputLocation::Pos(pos) => InputLocation::Pos(pos),
            pest::error::InputLocation::Span((start, end)) => InputLocation::Span(start, end),
        }
    }
}

impl<'a> From<pest::Position<'a>> for InputLocation {
    fn from(pest: pest::Position<'a>) -> InputLocation {
        InputLocation::Pos(pest.pos())
    }
}

impl<'a> From<pest::Span<'a>> for InputLocation {
    fn from(pest: pest::Span<'a>) -> InputLocation {
        InputLocation::Span(pest.start(), pest.end())
    }
}

impl<'a> From<pest::iterators::Pair<'a, Rule>> for InputLocation {
    fn from(pair: pest::iterators::Pair<'a, Rule>) -> InputLocation {
        InputLocation::from(pair.as_span())
    }
}

impl fmt::Display for InputLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InputLocation::Pos(pos) => write!(f, "{}", pos),
            InputLocation::Span(start, end) => write!(f, "{}:{}", start, end),
        }
    }
}
