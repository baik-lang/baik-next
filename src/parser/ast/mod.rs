mod atom;
pub mod binary;
mod boolean;
mod float;
mod function;
mod identifier;
mod integer;
mod local;
mod string;
mod term;
mod ty;
mod typespec;
pub mod unary;

pub use atom::Atom;
pub use binary::Binary;
pub use boolean::Boolean;
pub use float::Float;
pub use function::Function;
pub use identifier::Identifier;
pub use integer::Integer;
pub use local::Local;
pub use string::String;
pub use term::{NodeType, Term};
pub use ty::Ty;
pub use typespec::TypeSpec;
pub use unary::Unary;

use crate::parser::input_location::InputLocation;

pub trait Value: Sized {
    type Item;

    fn value(self) -> Self::Item;

    fn value_ref(&self) -> &Self::Item;
}

pub trait Location {
    fn location(&self) -> &InputLocation;
}
