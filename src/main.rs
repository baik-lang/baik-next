extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "grammer.pest"]
struct BaikLexer;

fn main() {
    let pairs = BaikLexer::parse(Rule::document, "tulis 'hallo',1234,'yaya\t' ").unwrap_or_else(|e| panic!("{}", e));

    for pair in pairs {
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.as_span());
        println!("Text:    {}", pair.as_str());

        // A pair can be converted to an iterator of the tokens which make it up:
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::whitespace => println!("Letter:  {}", inner_pair.as_str()),
                Rule::int => println!("Digit:   {}", inner_pair.as_str()),
                Rule::unicode => println!("Unicode:   {}", inner_pair.as_str()),
                Rule::string => println!("String:   {}", inner_pair.as_str()),
                Rule::hex => println!("Hex:   {}", inner_pair.as_str()),
                Rule::escape => println!("Escape:   {}", inner_pair.as_str()),
                Rule::tulis => println!("Tulist:   {}", inner_pair.as_str()),
                Rule::document => println!("Document:   {}", inner_pair.as_str()),
                Rule::letters => println!("Letters:   {}", inner_pair.as_str()),
                _ => unreachable!()
            };
        }
    }
}