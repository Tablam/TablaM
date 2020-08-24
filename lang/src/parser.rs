#![allow(dead_code)]

use crate::ast::*;
use crate::lexer::*;
use tablam::prelude::Scalar;

pub struct Parser<'source> {
    scanner: Scanner<'source>,
    _environment: Environment,
}

impl<'source> Parser<'source> {
    pub fn new(buffer: &'source str) -> Self {
        Parser {
            scanner: Scanner::new(buffer),
            _environment: Environment::new(None),
        }
    }

    fn accept(&mut self) -> Option<(Token, TokenData)> {
        self.scanner.accept()
    }

    fn peek(&mut self) -> Option<Token> {
        self.scanner.peek()
    }

    pub fn peek_both(&mut self) -> Option<(Token, TokenData)> {
        self.scanner.peek_both()
    }

    fn parse_let(&mut self) -> Return {
        if let Some(Token::Variable(name)) = self.peek() {
            self.accept();
            if let Some(Token::Assignment) = self.peek() {
                self.accept();
                return Ok(Expression::Variable(name, Box::new(self.parse_ast(0)?)));
            };
        };

        Err(Error::Unexpected)
    }

    pub fn parse(&mut self) -> Return {
        self.parse_ast(0)
    }

    /*fn search_next_expression(&mut self, wrong_token: &Token, error: &str) -> Return {
            let feedback = Error::Unexpected;

            loop {
                if let Some(op) = self.scanner.peek() {
                    match op {
                        Token::Let | Token::Var => break,
                        _ => {
                            self.scanner.accept();
                        }
                    }
                }
            }

            Err(feedback)
        }
    */

    fn prefix_binding_power(token: &Token) -> ((), u8) {
        match token {
            Token::Let | Token::Var => ((), 15),
            _ => panic!("bad op: {:?}", token),
        }
    }

    fn postfix_binding_power(token: &Token) -> Option<(u8, ())> {
        let res = match token {
            Token::RightParentheses => (11, ()),
            _ => return None,
        };
        Some(res)
    }

    fn infix_binding_power(token: &Token) -> Option<(u8, u8)> {
        let res = match token {
            Token::Equal => (2, 1),
            Token::NotEqual => (4, 3),
            Token::Plus | Token::Minus => (5, 6),
            _ => return None,
        };
        Some(res)
    }

    fn parse_ast(&mut self, min_bindpower: u8) -> Return {
        let op = self.accept();
        dbg!(&op);
        let mut lhs = match op {
            Some((Token::Integer(number), _)) => Expression::Value(Scalar::I64(number)),
            Some((Token::Float(number), _)) => Expression::Value(Scalar::F64(number)),
            Some((Token::Decimal(decimal), _)) => Expression::Value(Scalar::Decimal(decimal)),
            Some((Token::Var, _)) | Some((Token::Let, _)) => self.parse_let()?,
            t => panic!("bad token: {:?}", t),
        };

        while let Some(token) = self.peek() {
            dbg!(&token);

            if let Some((l_bp, ())) = Self::postfix_binding_power(&token) {
                if l_bp < min_bindpower {
                    break;
                }
                let token = self.accept();

                lhs = match token {
                    Some((Token::LeftParentheses, _)) => {
                        let rhs = self.parse_ast(0)?;
                        if let Some(Token::RightParentheses) = self.peek() {
                            Expression::Block(vec![lhs, rhs])
                        //unimplemented!();
                        } else {
                            return Err(Error::UnclosedGroup);
                        }
                    }
                    _ => continue,
                };
            }

            if let Some((l_bp, r_bp)) = Self::infix_binding_power(&token) {
                if l_bp < min_bindpower {
                    break;
                }
                self.accept();
                let rhs = self.parse_ast(r_bp)?;

                lhs = Expression::BinaryOp(BinaryOperation {
                    operator: token,
                    left: Box::new(lhs),
                    right: Box::new(rhs),
                });
                //lhs = Expression::Block(vec![lhs, rhs?]);
                continue;
            }

            break;
        }

        Ok(lhs)
    }
}
