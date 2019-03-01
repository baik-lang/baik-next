use pest::Parser;

#[derive(Parser)]
#[grammar = "grammer/baik.pest"]
struct BaikLexer;
use Rule::*;

pub fn lexer() {
    let pairs = BaikLexer::parse(hitung, "1234+1+(123+2+(1-2))").unwrap_or_else(|e| panic!("{}", e));

    for pair in pairs {
        println!("Rule:    {:?}", pair.as_rule());
        // println!("Span:    {:?}", pair.as_span());
        println!("Text:    {}", pair.as_str());
        println!("----------ops---------");

        // A pair can be converted to an iterator of the tokens which make it up:
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                whitespace => println!("Letter:  {}", inner_pair.as_str()),
                int => println!("Digit:   {}", inner_pair.as_str()),
                unicode => println!("Unicode:   {}", inner_pair.as_str()),
                string => println!("String:   {}", inner_pair.as_str()),
                hex => println!("Hex:   {}", inner_pair.as_str()),
                escape => println!("Escape:   {}", inner_pair.as_str()),
                tulis => println!("Tulist:   {}", inner_pair.as_str()),
                document => println!("Document:   {}", inner_pair.as_str()),
                letters => println!("Letters:   {}", inner_pair.as_str()),
                hitung => println!("Expr:   {}", inner_pair.as_str()),
                operation => println!("Operation:   {}", inner_pair.as_str()),
                term => println!("Term:   {}", inner_pair.as_str()),
                add => println!("aritmatik:   {}", inner_pair.as_str()),
                subtract => println!("aritmatik:   {}", inner_pair.as_str()),
                multiply => println!("aritmatik:   {}", inner_pair.as_str()),
                divide => println!("aritmatik:   {}", inner_pair.as_str()),
                power => println!("aritmatik:   {}", inner_pair.as_str()),
                float => println!("Float    :   {}", inner_pair.as_str()),
                _ => unreachable!()
            };
        }
    }
}