use std::collections::HashMap;
use tablam::prelude::*;

use crate::scanner::{Scanner, Token, TokenData};
use logos::Span;
use std::rc::Rc;

pub type Identifier = String;

#[derive(Debug, Clone)]
pub enum Expression {
    Value(Scalar),
    Variable(Identifier, Box<Expression>),
    Immutable(Identifier, Box<Expression>),
    BinaryOp(BinaryOperator),
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
    fn eval(&self, env: &mut Environment) -> Result<Expression, String>;
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

    pub fn parse(&mut self) -> Vec<Expression> {
        let ast = self.parse_ast(0);

        ast
    }

    fn parse_ast(&mut self, min_bindpower: u8) -> Vec<Expression> {
        let mut result = Vec::new();
        let op = self.scanner.peek();
        let mut lhs = match op {
            Some(Token::Integer(data)) => Expression::Value(Scalar::I64(data.value.unwrap())),
                Some(Token::Float(data))  => Expression::Value(Scalar::F64(data.value.unwrap())),
            Some(Token::Decimal(data)) => Expression::Value(Scalar::Decimal(data.value.unwrap())),
            variable_kind@ Some(Token::Var(_)) |  variable_kind@ Some(Token::Let(_)) => {
                dbg!(op);
                self.scanner.accept();
                let lhs = match self.scanner.peek(){
                    Some(Token::Variable(data)) => {
                        let lhs = data.value.unwrap();
                        self.scanner.accept();

                        let rhs = match self.scanner.peek() {
                            Some(Token::Assignment(_)) => {
                                let  (_, bind_power) = self.prefix_binding_power(self.scanner.peek().unwrap());
                                self.scanner.accept();
                                self.parse_ast(bind_power).pop().unwrap()
                            }
                            Some(wrong_token) => self.search_next_expression(wrong_token, "Assignment operator expected"),
                             None => Expression::Error("Unexpected final".to_string())
                        };

                        Expression::Variable(lhs, Box::new(rhs))
                    },
                    Some(wrong_token) => self.search_next_expression(wrong_token, "Identifier expected"),
                    None => Expression::Error("Unexpected final".to_string())
                };

                lhs
            }
            t => panic!("bad token: {:?}", t),
        };

        loop {
            let op = match lexer.peek() {
                Token::Eof => break,
                Token::Op(op) => op,
                t => panic!("bad token: {:?}", t),
            };

            if let Some((l_bp, ())) = postfix_binding_power(op) {
                8645
                if l_bp < min_bp {
                    break;
                }
                lexer.next();

                lhs = if op == '[' {
                    let rhs = expr_bp(lexer, 0);
                    assert_eq!(lexer.next(), Token::Op(']'));
                    S::Cons(op, vec![lhs, rhs])
                } else {
                    S::Cons(op, vec![lhs])
                };
                continue;
            }

            if let Some((l_bp, r_bp)) = infix_binding_power(op) {
                if l_bp < min_bp {
                    break;
                }
                lexer.next();

                lhs = if op == '?' {
                    let mhs = expr_bp(lexer, 0);
                    assert_eq!(lexer.next(), Token::Op(':'));
                    let rhs = expr_bp(lexer, r_bp);
                    S::Cons(op, vec![lhs, mhs, rhs])
                } else {
                    let rhs = expr_bp(lexer, r_bp);
                    S::Cons(op, vec![lhs, rhs])
                };
                continue;
            }

            break;
        }


        result
    }

    fn search_next_expression(&mut self, wrong_token:&Token, error:&str) -> Expression{

        //TODO: extract info from wrong_token
        let  data = TokenData{value: Some("dummyValue"), line:1, range_column: Span{start: 1, end: 2},
        line_range_column: Span{start:1, end: 2}};
        let feedback = Expression::Error(format!("{} at line {}, column {} : {}", error.to_string(), data.line,
        data.line_range_column.start, data.line_range_column.end));

        loop{
            if let Some(op) = self.scanner.peek() {
                match op {
                    Token::Let(_) | Token::Var(_) => break,
                    _ => {self.scanner.accept();}
                }
            }
        }

        feedback
    }

    fn prefix_binding_power(&self, token: &Token) -> ((), u8) {
        match token {
            Token::Let(_) | Token::Var(_) => ((), 15),
            _ => panic!("bad op: {:?}", token),
        }
    }

    fn infix_binding_power(token: Token) -> Option<(u8, u8)> {
        let res = match token {

            /*Token::Not(_) => (4,3),
            Token::Equal(_)
            | Token::Greater(_)
            | Token::GreaterEqual(_)
            | Token::Less(_)
            | Token::LessEqual(_)
            | Token::NotEqual(_) => (8, 6),*/
            Token:: => (4, 3),
            '+' | '-' => (5, 6),
            '*' | '/' => (7, 8),
            '.' => (14, 13),
            Token::Assignment(_) => (14, 13),
            _ => return None,
        };
        Some(res)
    }
}

impl<'source> Iterator for Parser<'source> {
    type Item = Vec<Expression>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.parse())
    }
}
