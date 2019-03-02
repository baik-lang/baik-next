extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod core;
pub mod expr;

use std::collections::HashMap;
use serde_json::to_value as json_to_value;
use serde::Serialize;

pub use serde_json::Value;
pub use std::error::Error;

pub fn to_value<S: Serialize>(v: S) -> Value {
    json_to_value(v).unwrap()
}