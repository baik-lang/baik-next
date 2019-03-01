// Needed by pest
#![recursion_limit = "300"]


#[macro_use]
extern crate pest;

use std::result::Result;
use std::collections::HashMap;

use pest::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    /// A type like Int
    Named(String),
    /// A non-nullable type like Int!
    NonNullNamed(String),
    /// A nullable type like [Int].
    /// The nullable part is the list, not Int in that example
    List(Vec<String>),
    /// A non-nullable list like [Int]!, the types inside can be null
    NonNullList(Vec<String>),
}

#[derive(Clone, Debug, PartialEq)]
#[allow(missing_docs)]
pub enum InputValue {
    Variable(String),
    Int(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
    Enum(String),
    List(Vec<Node>),
    Object,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Selection {
    Field(Box<Field>),
    FragmentSpread(Box<FragmentSpread>),
    InlineFragment(Box<InlineFragment>),
}


#[derive(Clone, Debug, PartialEq)]
pub struct Directive {
    pub name: String,
    pub arguments: HashMap<String, InputValue>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FragmentSpread {
    pub name: String,
    pub directives: Vec<Directive>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InlineFragment {
    pub type_condition: String,
    pub directives: Vec<Directive>,
    pub selection_set: Selection,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Field {
    pub alias: Option<String>,
    pub name: String,
    pub arguments: HashMap<String, InputValue>,
    pub directives: Vec<Directive>,
    pub selection_set: Selection,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    Name(String),
    Document,
}

impl_rdp! {
    grammar! {
        whitespace = _{ ([" "] | ["\t"] | ["\r"] | ["\n"]) + }

        comment = _{ ["#"] ~ (!(["\n"]) ~ any)* ~ ["\n"] }
        letters = _{ ['A'..'Z'] | ['a'..'z'] }
        exp     = _{ (["e"] | ["E"]) ~ (["+"] | ["-"])? ~ ['1'..'9']+ }
        hex     = _{ ['0'..'9'] | ['a'..'f'] | ['A'..'F'] }
        unicode = _{ ["u"] ~ hex ~ hex ~ hex ~ hex }
        escape  = _{ ["\\"] ~ (["\""] | ["\\"] | ["/"] | ["b"] | ["f"] | ["n"] | ["r"] | ["t"] | unicode) }

        op_true  = { ["true"] }
        op_false = { ["false"] }
        boolean  = _{ op_true | op_false }
        null     = { ["null"] }
        int      = @{ ["-"]? ~ (["0"] | ['1'..'9'] ~ ['0'..'9']*) }
        float    = @{
            ["-"]? ~
                (
                    ['1'..'9']+ ~ exp |
                    ["0"] ~ ["."] ~ ['0'..'9']+ ~ exp? |
                    ['1'..'9'] ~ ['0'..'9']* ~ ["."] ~ ['0'..'9']+ ~ exp?
                )
        }
        string   = @{ ["\""] ~ (escape | !(["\""] | ["\\"]) ~ any)* ~ ["\""] }
        variable = @{ ["$"] ~ name }
        enum_val = @{ !(boolean | null) ~ name }
        list     = @{ ["["] ~ value ~ ["]"] }
        arg      = { name ~ [":"] ~ value }
        object   = { ["{"] ~ (arg ~ ([","] ~ arg)*)? ~ ["}"] }

        name  = @{ (["_"] | letters) ~ (["_"] | letters | ['0'..'9'])* }
        value = @{ variable | float | int | string | boolean | null | enum_val | list | object }

        // More variables stuff
        named_type = { name }
        list_type = {["["] ~ types ~ ["]"]}
        non_null_type = { (named_type | list_type) ~ ["!"]}
        types = { named_type | list_type | non_null_type }
        default_value = { ["="] ~ value }
        variable_def = { variable ~ [":"] ~ types ~ default_value? }
        variable_defs = { ["("] ~ variable_def? ~ ([","] ~ variable_def)* ~ [")"] }

        // Directive
        directive = { ["@"] ~ name ~ args? }

        // Selections
        selection = { field | fragment_spread | fragment_inline }
        selection_set = { ["{"] ~ selection+ ~ ["}"] }

        // Field
        alias = { name ~ [":"]}
        args  = { ["("] ~ arg ~ ([","] ~ arg)* ~ [","]? ~ [")"]}
        field = { alias? ~ name ~ args? ~ directive? ~selection_set? }

        // Fragments
        fragment_name = { !["on"] ~ name }
        fragment_def = { ["fragment"] ~ fragment_name ~ ["on"] ~ name ~ directive? ~ selection_set }
        fragment_spread = @{ ["..."] ~ fragment_name ~ directive? }
        fragment_inline = { ["..."] ~ (["on"] ~ name)? ~ directive? ~ selection_set }

        query = { ["query"] ~ name? ~ variable_defs? ~ selection_set }
        mutation = { ["mutation"] ~ name? ~ variable_defs? ~ selection_set }
        operation = { query | mutation | selection_set }

        document = @{ soi ~ (operation | fragment_def)+ ~ eoi }
    }
}

pub fn parse(input: &str) -> Result<(), String> {
    let mut parser = Rdp::new(StringInput::new(input));

    if !parser.document() {
        let (_, pos) = parser.expected();
        let (line_no, col_no) = parser.input().line_col(pos);
        return Err(format!("syntax bermasalah {}, column {}", line_no, col_no));
    }

//    parser.main()
    Ok(())
}
