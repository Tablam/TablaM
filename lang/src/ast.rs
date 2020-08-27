use std::collections::HashMap;
use std::mem::discriminant;

use tablam::derive_more::{Display, From};
use tablam::prelude::{BinOp, LogicOp, Scalar};

use crate::lexer::{Token, TokenData};
use std::fmt;
use tablam::function::{Function, Param};
use tablam::types::format_list;

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

    #[display(fmt = "{}", _0)]
    UnexpectedItem(Expression),
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

#[derive(Debug, Clone, Display, From)]
pub enum Expression {
    //Values
    #[from]
    #[display(fmt = "{}", _0)]
    Value(Scalar),
    #[display(fmt = "{}", _0)]
    Variable(Identifier),

    //Variable definitions
    #[display(fmt = "var {:} := {}", _0, _1)]
    Mutable(Identifier, Box<Expression>),
    #[display(fmt = "let {:} := {}", _0, _1)]
    Immutable(Identifier, Box<Expression>),

    #[from]
    #[display(fmt = "{}", _0)]
    Function(Function),
    #[from]
    #[display(fmt = "{}", _0)]
    FunctionCall(FunctionCall),

    #[from]
    #[display(fmt = "{}", _0)]
    BinaryOp(BinaryOperation),

    #[from]
    #[display(fmt = "{}", _0)]
    ComparisonOp(ComparisonOperator),

    #[from]
    #[display(fmt = "{}", _0)]
    ParameterDefinition(Param),

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

impl Expression {
    pub fn is(variant: &Self, expected: &Self) -> bool {
        discriminant(variant) == discriminant(expected)
    }
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

#[derive(Debug, Clone, Display)]
#[display(fmt = "{} := {}", name, value)]
pub struct ParamCall {
    pub name: String,
    pub value: Expression,
}

impl ParamCall {
    pub fn new(name: &str, value: Expression) -> Self {
        ParamCall {
            name: name.to_string(),
            value,
        }
    }
}

#[derive(Debug, Clone, From)]
pub struct FunctionCall {
    pub name: String,
    pub params: Vec<ParamCall>,
}

impl FunctionCall {
    pub fn new(name: &str, params: &[ParamCall]) -> Self {
        FunctionCall {
            name: name.into(),
            params: params.to_vec(),
        }
    }
}

impl fmt::Display for FunctionCall {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)?;
        format_list(&self.params, self.params.len(), "(", ")", f)?;
        Ok(())
    }
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
