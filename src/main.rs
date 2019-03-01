extern crate pest;
#[macro_use]
extern crate pest_derive;

mod lexer;

use lexer::lexer;

fn main() {
    lexer();
}