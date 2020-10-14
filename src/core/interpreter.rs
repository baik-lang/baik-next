use pest::Parser;
use pest::iterators::{Pairs};
use crate::{eval};

#[derive(Parser)]
#[grammar = "grammar/baik.pest"]
struct BaikLexer;
//use Rule::*;

fn pair(expr : Pairs<Rule>){
    for inner_pair in expr {
        match inner_pair.as_rule() {
            function => {
                println!("Digit:   {}", inner_pair.as_str());
                pair(inner_pair.into_inner());
            },
            EOI => {
                println!("EOI:   {}", inner_pair.as_str());
                pair(inner_pair.into_inner());
            },
            _ => {
                println!("----");
            }
        };
    }
}

pub fn interpreter(baik_script: String) {
    let pairs = BaikLexer::parse(Rule::input, &baik_script).unwrap_or_else(|e| panic!("{}", e));
    pair(pairs);
}