use crate::*;
use {Function, Functions, Context, Contexts, Compiled, Value};
use tree::Tree;
use Error;
use serde::Serialize;
use to_value;
use std::fmt;

pub struct Expr {
    expression: String,
    compiled: Option<Compiled>,
    functions: Functions,
    contexts: Contexts,
}

impl Expr {
    pub fn new<T: Into<String>>(expr: T) -> Expr {
        Expr {
            expression: expr.into(),
            compiled: None,
            functions: Functions::new(),
            contexts: create_empty_contexts(),
        }
    }

    pub fn function<T, F>(mut self, name: T, function: F) -> Expr
        where T: Into<String>,
              F: 'static + Fn(Vec<Value>) -> Result<Value, Error> + Sync + Send
    {
        self.functions.insert(name.into(), Function::new(function));
        self
    }

    pub fn value<T, V>(mut self, name: T, value: V) -> Expr
        where T: Into<String>,
              V: Serialize
    {
        self.contexts.last_mut().unwrap().insert(name.into(), to_value(value));
        self
    }

    pub fn compile(mut self) -> Result<Expr, Error> {
        self.compiled = Some(Tree::new(self.expression.clone()).compile()?);
        Ok(self)
    }

    pub fn exec(&self) -> Result<Value, Error> {
        if self.compiled.is_none() {
            Tree::new(self.expression.clone()).compile()?(&self.contexts, &self.functions)
        } else {
            self.compiled.as_ref().unwrap()(&self.contexts, &self.functions)
        }
    }

    fn get_compiled(&self) -> Option<&Compiled> {
        self.compiled.as_ref()
    }
}

impl Clone for Expr {
    fn clone(&self) -> Expr {
        Expr {
            expression: self.expression.clone(),
            compiled: if self.compiled.is_some() {
                Some(Tree::new(self.expression.clone()).compile().unwrap())
            } else {
                None
            },
            contexts: self.contexts.clone(),
            functions: Functions::new(),
        }
    }
}

impl fmt::Debug for Expr {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(formatter, "{:?}", self.expression)
    }
}

pub struct ExecOptions<'a> {
    expr: &'a Expr,
    contexts: Option<&'a [Context]>,
    functions: Option<&'a Functions>,
}

impl<'a> ExecOptions<'a> {
    pub fn new(expr: &'a Expr) -> ExecOptions<'a> {
        ExecOptions {
            expr: expr,
            contexts: None,
            functions: None,
        }
    }

    pub fn contexts(&mut self, contexts: &'a [Context]) -> &'a mut ExecOptions {
        self.contexts = Some(contexts);
        self
    }

    pub fn functions(&mut self, functions: &'a Functions) -> &'a mut ExecOptions {
        self.functions = Some(functions);
        self
    }

    pub fn exec(&self) -> Result<Value, Error> {
        let empty_contexts = create_empty_contexts();
        let empty_functions = Functions::new();

        let contexts = if self.contexts.is_some() {
            self.contexts.unwrap()
        } else {
            &empty_contexts
        };

        let functions = if self.functions.is_some() {
            self.functions.unwrap()
        } else {
            &empty_functions
        };

        let compiled = self.expr.get_compiled();
        if compiled.is_none() {
            Tree::new(self.expr.expression.clone()).compile()?(contexts, functions)
        } else {
            compiled.unwrap()(contexts, functions)
        }
    }
}


fn create_empty_contexts() -> Contexts {
    let mut contexts = Contexts::new();
    contexts.push(Context::new());
    contexts
}