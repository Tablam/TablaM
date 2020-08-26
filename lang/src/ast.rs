use std::collections::HashMap;

use tablam::derive_more::{Display, From};
use tablam::prelude::{BinOp, LogicOp, Scalar};

use crate::lexer::{Token, TokenData};
use tablam::function::Function;

pub type Identifier = String;

#[derive(Debug, Display, From)]
#[display(fmt = "Syntax error => {}")]
pub enum Error {
    #[from]
    #[display(fmt = "{}", _0)]
    CoreError(tablam::errors::Error),
    #[from]
    #[display(
        fmt = "Unexpected token. It found: {}, it was expected: {}. ({})",
        _0,
        _1,
        _2
    )]
    Unexpected(Token, Token, TokenData),
    #[from]
    #[display(fmt = "Unclosed group. It was expected: {}. ({})", _0, _1)]
    UnclosedGroup(Token, TokenData),
    #[display(fmt = "Variable '{}' not found in scope", _0)]
    VariableNotFound(String),
    #[display(fmt = "Function '{}' not found in scope", _0)]
    FunctionNotFound(String),
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
    ComparisonOp(ComparisonOperator),

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
    pub operator: BinOp,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

impl BinaryOperation {
    pub fn new(token: Token, left: Box<Expression>, right: Box<Expression>) -> Self {
        let operator = match token {
            Token::Plus => BinOp::Add,
            Token::Minus => BinOp::Minus,
            Token::Multiplication => BinOp::Mul,
            Token::Division => BinOp::Div,
            _ => unreachable!("Binary operator not implemented."),
        };

        BinaryOperation {
            operator,
            left,
            right,
        }
    }
}

#[derive(Debug, Clone, Display)]
#[display(fmt = "{} {} {}", left, operator, right)]
pub struct ComparisonOperator {
    pub operator: LogicOp,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

impl ComparisonOperator {
    pub fn new(token: Token, left: Box<Expression>, right: Box<Expression>) -> Self {
        let operator = match token {
            Token::Equal => LogicOp::Equal,
            Token::NotEqual => LogicOp::NotEqual,
            Token::Greater => LogicOp::Greater,
            Token::GreaterEqual => LogicOp::GreaterEqual,
            Token::Less => LogicOp::Less,
            Token::LessEqual => LogicOp::LessEqual,
            Token::And => LogicOp::And,
            Token::Or => LogicOp::Or,
            _ => unreachable!("Binary operator not implemented."),
        };

        ComparisonOperator {
            operator,
            left,
            right,
        }
    }
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

    pub fn find_function(&self, name: &str) -> Result<Function, Error> {
        match self.functions.get(name) {
            Some(function) => {
                if let Expression::Function(f) = function {
                    Ok(f.clone())
                } else {
                    Err(Error::FunctionNotFound(name.to_string()))
                }
            }
            None => match &self.parent {
                Some(env) => env.find_function(name),
                None => Err(Error::FunctionNotFound(name.to_string())),
            },
        }
    }

    pub fn create_child(self) -> Self {
        Environment::new(Some(Box::new(self)))
    }
}
