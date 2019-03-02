use std::fmt;
use serde_json::Value;
use error::Error;

pub struct Function {
    pub max_args: Option<usize>,
    pub min_args: Option<usize>,
    pub compiled: Box<Fn(Vec<Value>) -> Result<Value, Error> + Sync + Send>,
}

impl Function {
    pub fn new<F>(closure: F) -> Function
        where F: 'static + Fn(Vec<Value>) -> Result<Value, Error> + Sync + Send
    {
        Function {
            max_args: None,
            min_args: None,
            compiled: Box::new(closure),
        }
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "Function {{ max_args: {:?}, min_args: {:?} }}",
               self.max_args,
               self.min_args)
    }
}