use std::collections::HashMap;

use tablam::derive_more::{Display, From};
use tablam::prelude::Scalar;

use crate::lexer::{Token, TokenData};
use tablam::function::Function;

pub type Identifier = String;

#[derive(Debug, Display, From)]
#[display(fmt = "Syntax error => {}")]
pub enum Error {
    #[display(fmt = "{}", _0)]
    CoreError(tablam::errors::Error),
    #[display(
        fmt = "Unexpected token. It found: {}, it was expected: {}. ({})",
        _0,
        _1,
        _2
    )]
    Unexpected(Token, Token, TokenData),
    #[display(fmt = "Unclosed group. It was expected: {}. ({})", _0, _1)]
    UnclosedGroup(Token, TokenData),
    #[display(fmt = "Variable '{}' not found in scope", _0)]
    VariableNotFound(String),
    #[display(fmt = "Unexpected EOF.")]
    Eof,
}

pub type Return = std::result::Result<Expression, Error>;
pub type ReturnT<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Display)]
pub enum Expression {
    //Values
    #[display(fmt = "{}", _0)]
    Value(Scalar),
    #[display(fmt = "{}", _0)]
    Variable(Identifier),

    //Variable definitions
    #[display(fmt = "var {:} := {}", _0, _1)]
    Mutable(Identifier, Box<Expression>),
    #[display(fmt = "let {:} := {}", _0, _1)]
    Immutable(Identifier, Box<Expression>),

    #[display(fmt = "{}", _0)]
    Function(Function),

    #[display(fmt = "{}", _0)]
    BinaryOp(BinaryOperation),
    #[display(fmt = "{}", _0)]
    ComparisonOp(BinaryOperation),

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

    pub fn find_variable(&self, name: &str) -> Result<&Expression, Error> {
        match self.vars.get(name) {
            Some(variable) => Ok(variable),
            None => match &self.parent {
                Some(env) => env.find_variable(name),
                None => Err(Error::VariableNotFound(name.to_string())),
            },
        }
    }

    pub fn find_function(&self, k: &str) -> Option<Function> {
        match self.functions.get(k) {
            Some(function) => {
                if let Expression::Function(f) = function {
                    Some(f.clone())
                } else {
                    None
                }
            }
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
