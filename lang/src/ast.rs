use std::collections::HashMap;

use logos::Span;

use tablam::derive_more::{Display, From};
use tablam::prelude::*;

use crate::scanner::{Scanner, Token, TokenData};

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

pub type Result = std::result::Result<Expression, Error>;

#[derive(Debug, Clone)]
pub enum Expression {
    Value(Scalar),
    Variable(Identifier, Box<Expression>),
    Immutable(Identifier, Box<Expression>),
    BinaryOp(BinaryOperator),
    Block(Vec<Expression>),
    Error(String),
}

#[derive(Debug, Clone)]
pub struct Environment {
    vars: HashMap<Identifier, Expression>,
    functions: HashMap<Identifier, Expression>,
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

trait Evaluable {
    fn eval(&self, env: &mut Environment) -> Result;
}

#[derive(Debug, Clone)]
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

    pub fn parse(&mut self) -> Result {
        let ast = self.parse_ast(0);

        ast
    }

    fn parse_let(&mut self) -> Result {
        self.accept();
        if let Some(Token::Variable(data)) = self.peek() {
            let lhs = data.value.clone().unwrap();
            self.accept();
            if let Some(Token::Assignment(_)) = self.peek() {
                self.accept();
                return self.parse_ast(0);
            }
        };

        Err(Error::Unexpected)
    }

    fn accept(&mut self) -> Option<Token> {
        self.scanner.accept()
    }
    fn peek(&mut self) -> Option<&Token> {
        self.scanner.peek()
    }

    fn parse_ast(&mut self, min_bindpower: u8) -> Result {
        let op = self.peek();
        let mut lhs = match op {
            Some(Token::Integer(data)) => Expression::Value(Scalar::I64(data.value.unwrap())),
            Some(Token::Float(data)) => Expression::Value(Scalar::F64(data.value.unwrap())),
            Some(Token::Decimal(data)) => Expression::Value(Scalar::Decimal(data.value.unwrap())),
            variable_kind @ Some(Token::Var(_)) | variable_kind @ Some(Token::Let(_)) => {
                self.parse_let()?
            }
            t => panic!("bad token: {:?}", t),
        };

        while let Some(token) = self.peek() {
            if let Some((l_bp, ())) = Self::postfix_binding_power(token) {
                if l_bp < min_bindpower {
                    break;
                }
                let token = self.accept();

                lhs = match token {
                    Some(Token::LeftParentheses(_)) => {
                        let rhs = self.parse_ast(0)?;
                        if let Some(Token::RightParentheses(_)) = self.peek() {
                            //Expression::Block(vec![lhs.clone(), lhs])
                            unimplemented!();
                        } else {
                            return Err(Error::UnclosedGroup);
                        }
                    }
                    _ => continue,
                };
            }

            if let Some((l_bp, r_bp)) = Self::infix_binding_power(token) {
                if l_bp < min_bindpower {
                    break;
                }
                self.accept();
                let rhs = self.parse_ast(0);

                lhs = Expression::Block(vec![lhs, rhs?]);
                continue;
            }

            break;
        }

        Ok(lhs)
    }

    fn search_next_expression(&mut self, wrong_token: &Token, error: &str) -> Result {
        //TODO: extract info from wrong_token
        let data = TokenData {
            value: Some("dummyValue".to_string()),
            line: 1,
            range_column: Span { start: 1, end: 2 },
            line_range_column: Span { start: 1, end: 2 },
        };
        let feedback = Error::Unexpected;

        loop {
            if let Some(op) = self.scanner.peek() {
                match op {
                    Token::Let(_) | Token::Var(_) => break,
                    _ => {
                        self.scanner.accept();
                    }
                }
            }
        }

        Err(feedback)
    }

    fn prefix_binding_power(token: &Token) -> ((), u8) {
        match token {
            Token::Let(_) | Token::Var(_) => ((), 15),
            _ => panic!("bad op: {:?}", token),
        }
    }

    fn postfix_binding_power(token: &Token) -> Option<(u8, ())> {
        let res = match token {
            Token::RightParentheses(_) => (11, ()),
            _ => return None,
        };
        Some(res)
    }

    fn infix_binding_power(token: &Token) -> Option<(u8, u8)> {
        let res = match token {
            Token::Equal(_) => (2, 1),
            Token::NotEqual(_) => (4, 3),
            Token::Plus(_) | Token::Minus(_) => (5, 6),
            _ => return None,
        };
        Some(res)
    }
}
