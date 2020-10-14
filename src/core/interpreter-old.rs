use pest::Parser;
use pest::iterators::{Pairs};
use crate::{eval};

#[derive(Parser)]
#[grammar = "grammar/baik.pest"]
struct BaikLexer;
use Rule::*;

fn pair(expr : Pairs<Rule>){
    for inner_pair in expr {
        match inner_pair.as_rule() {
            whitespace => println!("Letter:  {}", inner_pair.as_str()),
            int => {
                //println!("Digit:   {}", inner_pair.as_str());
                pair(inner_pair.into_inner());
            },
            unicode => println!("Unicode:   {}", inner_pair.as_str()),
            string => {
                println!("String:   {}", inner_pair.as_str());
                pair(inner_pair.into_inner());
            },
            hex => println!("Hex:   {}", inner_pair.as_str()),
            escape => println!("Escape:   {}", inner_pair.as_str()),
            tulis => {
                println!("Tulist:   {}", inner_pair.as_str());
                pair(inner_pair.into_inner());
            },
            document => {
                println!("Document:   {}", inner_pair.as_str());
                pair(inner_pair.into_inner());
            },
            letters => println!("Letters:   {}", inner_pair.as_str()),
            hitung => {
                println!("Expr:   {}", inner_pair.as_str());
                match eval(inner_pair.as_str()){
                    Ok(v) => println!("Result Expr: {}", v),
                    Err(e) => println!("error : {:?}", e),
                }
                // println!("---Details---");
                // pair(inner_pair.into_inner());
            },
            operation => println!("Operation:   {}", inner_pair.as_str()),
            term => println!("Term:   {}", inner_pair.as_str()),
            add => {
                println!("aritmatik:   {}", inner_pair.as_str());
                pair(inner_pair.into_inner());
            },
            subtract => {
                println!("aritmatik:   {}", inner_pair.as_str());
                pair(inner_pair.into_inner());
            },
            multiply => {
                println!("aritmatik:   {}", inner_pair.as_str());
                pair(inner_pair.into_inner());
            },
            divide => {
                println!("aritmatik:   {}", inner_pair.as_str());
                pair(inner_pair.into_inner());
            },
            power => {
                println!("aritmatik:   {}", inner_pair.as_str());
                pair(inner_pair.into_inner());
            },
            float => {
                println!("aritmatik:   {}", inner_pair.as_str());
                pair(inner_pair.into_inner());
            },
            variable => {
                println!("variable:   {}", inner_pair.as_str());
                pair(inner_pair.into_inner());
            },
            value => {
                println!("value:   {}", inner_pair.as_str());
                pair(inner_pair.into_inner());
            },
            name => {
                println!("name:   {}", inner_pair.as_str());
                pair(inner_pair.into_inner());
            },
            //Function
            func => {
                println!("F():   {}", inner_pair.as_str());
                pair(inner_pair.into_inner());
            },
            func_start => {
                println!("F(start):   {}", inner_pair.as_str());
                pair(inner_pair.into_inner());
            },
            func_name => {
                println!("F(name):   {}", inner_pair.as_str());
                pair(inner_pair.into_inner());
            },
            func_input => {
                println!("F(input):   {}", inner_pair.as_str());
                pair(inner_pair.into_inner());
            },
            func_begin => {
                println!("F(begin):   {}", inner_pair.as_str());
                pair(inner_pair.into_inner());
            },
            func_body => {
                println!("F(body):   {}", inner_pair.as_str());
                pair(inner_pair.into_inner());
            },
            func_end => {
                println!("F(end):   {}", inner_pair.as_str());
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
    let pairs = BaikLexer::parse(document, &baik_script).unwrap_or_else(|e| panic!("{}", e));
    pair(pairs);
}