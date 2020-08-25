use std::collections::HashMap;

use tablam::derive_more::{Display, From};
use tablam::prelude::Scalar;

use crate::lexer::Token;

pub type Identifier = String;

#[derive(Debug, Display, From)]
pub enum Error {
    #[display(fmt = "Unexpected token.)")]
    Unexpected,
    #[display(fmt = "Unclosed group.")]
    UnclosedGroup,
    #[display(fmt = "Unexpected EOF")]
    Eof,
}

pub type Return = std::result::Result<Expression, Error>;

#[derive(Debug, Clone, Display)]
pub enum Expression {
    #[display(fmt = "{}", _0)]
    Value(Scalar),

    #[display(fmt = "var {:} := {}", _0, _1)]
    Variable(Identifier, Box<Expression>),
    #[display(fmt = " let {:} := {} ", _0, _1)]
    Immutable(Identifier, Box<Expression>),

    #[display(fmt = "{}", _0)]
    BinaryOp(BinaryOperation),

    #[display(
        fmt = "{}",
        r#"_0.iter().map(|expr| expr.to_string())
        .fold(String::new(), |mut previous, current| { 
        previous.push_str(current.as_str()); previous.push('\n'); previous})"#
    )]
    Block(Vec<Expression>),

    #[display(fmt = "{}", _0)]
    Error(String),
    #[display(fmt = "pass")]
    Pass,
    #[display(fmt = "eof")]
    Eof,
}

#[derive(Debug, Clone, Display)]
#[display(fmt = "{} {} {}", left, operator, right)]
pub struct BinaryOperation {
    pub operator: Token,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Debug, Clone)]
pub struct Environment {
    vars: HashMap<Identifier, Expression>,
    functions: HashMap<Identifier, Expression>,
    parent: Option<Box<Environment>>,
}

impl Environment {
    pub fn new(parent: Option<Box<Environment>>) -> Self {
        Environment {
            vars: HashMap::new(),
            functions: HashMap::new(),
            parent,
        }
    }

    pub fn add_variable(&mut self, name: String, value: Expression) {
        self.vars.insert(name, value);
    }

    pub fn add_function(&mut self, name: String, def: Expression) {
        self.functions.insert(name, def);
    }

    pub fn find_variable(&self, name: &str) -> Option<&Expression> {
        match self.vars.get(name) {
            Some(variable) => Some(variable),
            None => match &self.parent {
                Some(env) => env.find_variable(name),
                None => None,
            },
        }
    }

    pub fn find_function(&self, k: &str) -> Option<&Expression> {
        match self.functions.get(k) {
            Some(function) => Some(function),
            None => match &self.parent {
                Some(env) => env.find_function(k),
                None => None,
            },
        }
    }

    pub fn create_child(self) -> Self {
        Environment::new(Some(Box::new(self)))
    }
}
