use std::collections::HashMap;
use tablam::prelude::*;

use crate::scanner::{Scanner, Token, TokenData};

#[derive(Debug, Clone)]
pub enum Expression {
    Value(Scalar),
    Variable(String, Scalar),
    //BinaryOp(BinaryOperator),
    Error(String),
}

#[derive(Debug, Clone)]
pub struct Environment {
    vars: HashMap<String, Expression>,
    functions: HashMap<String, Expression>,
    parent: Option<Box<Environment>>,
}

impl Environment {
    pub fn new(parent: Option<Box<Environment>>) -> Self {
        dbg!(&parent);
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
                Some(env) => env.find_var(name),
                None => None,
            },
        }
    }

    pub fn find_function(&self, k: &str) -> Option<&Expression> {
        match self.functions.get(k) {
            Some(function) => Some(function),
            None => match &self.parent {
                Some(env) => env.find_fun(k),
                None => None,
            },
        }
    }

    pub fn create_child(self) -> Self {
        Environment::new(Some(Box::new(self)))
    }
}

trait Evaluable {
    fn eval(&self, env: &mut Environment) -> Result<Expression, String>;
}

#[derive(Debug)]
pub struct BinaryOperator {
    operator: Token,
    left: Box<Expression>,
    right: Box<Expression>,
}

/*impl Evaluable for BinaryOperator {
    fn eval(&self, env: &mut Environment) -> Result<Expression, String> {
        match &self.operator {
            Token::Plus(data) => {
                let left: &Scalar = match &self.left {
                    Box<Expression::Value> => &scalar,
                    Expression::Variable(_, scalar) => &scalar,
                    Expression::BinaryOp(binary) => match binary.eval(env).unwrap() {
                        Expression::Value(scalar) => &scalar,
                    },
                    _ => panic!("test"),
                };
            }
            _ => panic!("is not a binary operator"),
        }

        Ok(Expression::Value(Scalar::Decimal(
            "3".parse::<Decimal>().unwrap(),
        )))
    }
} */

pub struct Parser<'source> {
    scanner: Scanner<'source>,
    environment: Environment,
    current_line: usize,
}

impl<'source> Parser<'source> {
    pub fn new(buffer: &'source str) -> Self {
        Parser {
            scanner: Scanner::new(buffer),
            environment: Environment::new(None),
            current_line: 1,
        }
    }

    fn parse(&mut self) -> Vec<Expression> {
        let mut line = Vec::new();
        loop {
            if let Some(token) = self.scanner.peek() {
                match token {
                    Token::Let(data) => {
                        if { data.line > self.current_line } {
                            break;
                        }

                        self.scanner.accept();
                        let result = self.consume(Token::Variable(TokenData { line: 0 }), "");
                    }
                    _ => continue,
                }
            }

            break;
        }
        line
    }

    fn consume<T>(&mut self, expected: Token, error: &str) -> Option<Expression> {
        if let Some(token) = self.scanner.peek() {
            let result = match token {
                expected => None,
                _ => Some(Expression::Error(error.to_string())),
            };

            return result;
        }

        None
    }
}

impl<'source> Iterator for Parser<'source> {
    type Item = Vec<Expression>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.parse())
    }
}
