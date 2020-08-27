#![allow(dead_code)]

use std::mem::discriminant;
use std::rc::Rc;

use crate::ast::*;
use crate::lexer::*;
use tablam::prelude::Scalar;

pub struct Parser<'source> {
    scanner: Scanner<'source>,
}

impl<'source> Parser<'source> {
    pub fn new(buffer: &'source str) -> Self {
        Parser {
            scanner: Scanner::new(buffer),
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

    pub fn parse(&mut self) -> Return {
        self.parse_ast(0)
    }

    fn check_next_token(&mut self, expected: Token) -> std::result::Result<Token, ErrorLang> {
        if let Some((found, data)) = self.peek_both() {
            return if discriminant(&found) == discriminant(&expected) {
                self.accept();
                Ok(found)
            } else {
                let feedback = ErrorLang::Unexpected(found, expected, data);
                Err(feedback)
            };
        }

        Err(ErrorLang::Eof)
    }

    fn accept_and_check_next(&mut self, expected: Token) -> std::result::Result<Token, ErrorLang> {
        let result = self.check_next_token(expected);
        match result {
            Ok(token) => Ok(token),
            Err(error) => {
                self.accept();
                Err(error)
            }
        }
    }

    fn match_at_least_one(
        &mut self,
        conditions: Vec<Token>,
    ) -> std::result::Result<Token, ErrorLang> {
        let mut result = Err(ErrorLang::Eof);
        for expected in conditions {
            result = self.check_next_token(expected);
            match &result {
                Err(_) => continue,
                Ok(_) => return result,
            }
        }

        result
    }

    fn continue_until_expression(&mut self, conditions: Vec<Token>) -> Return {
        let mut result;
        for expected in conditions {
            result = self.check_next_token(expected);
            match result {
                Err(error) => return Err(error),
                Ok(_) => continue,
            }
        }

        self.parse_ast(0)
    }

    fn prefix_binding_power(token: &Token) -> ((), u8) {
        match token {
            Token::Minus => ((), 11),
            Token::Not => ((), 12),
            _ => panic!("bad op: {:?}", token),
        }
    }

    fn postfix_binding_power(token: &Token) -> Option<(u8, ())> {
        let res = match token {
            Token::RightParentheses => (13, ()),
            _ => return None,
        };
        Some(res)
    }

    fn infix_binding_power(token: &Token) -> Option<(u8, u8)> {
        let res = match token {
            Token::Or => (1, 2),
            Token::And => (3, 4),
            Token::Equal
            | Token::NotEqual
            | Token::Greater
            | Token::GreaterEqual
            | Token::Less
            | Token::LessEqual => (6, 5),
            Token::Plus | Token::Minus => (7, 8),
            Token::Multiplication | Token::Division => (9, 10),
            _ => return None,
        };
        Some(res)
    }

    fn parse_var(&mut self) -> Return {
        let mut result = self.check_next_token(Token::Variable("".to_string()));
        if let Ok(Token::Variable(name)) = result {
            result = self.check_next_token(Token::Assignment);
            if result.is_ok() {
                return Ok(Expression::Mutable(name, Box::new(self.parse_ast(0)?)));
            }
        }

        Err(result.err().unwrap())
    }

    fn parse_param_call(&mut self) -> std::result::Result<Option<ParamCall>, ErrorLang> {
        if let Some((token, _data)) = self.accept() {
            //dbg!(&token);
            if token.is_literal_or_value() {
                let expr = self.parse_ast(0)?;
                Ok(Some(ParamCall::new("", expr)))
            } else {
                Err(ErrorLang::Eof)
            }
        } else {
            Err(ErrorLang::Eof)
        }
    }

    fn parse_function_call(&mut self, name: &str) -> Return {
        //Eat '('
        self.accept();
        let expr = self.parse_ast(0)?;
        // dbg!(&expr);
        let mut params = Vec::new();

        params.push(ParamCall::new("", expr));

        Ok(FunctionCall::new(name, &params).into())
    }

    fn parse_let(&mut self) -> Return {
        let dummy = String::from("");
        let mut result =
            self.match_at_least_one(vec![Token::Variable(dummy.clone()), Token::Constant(dummy)]);
        if let Ok(Token::Variable(name)) | Ok(Token::Constant(name)) = result {
            result = self.check_next_token(Token::Assignment);
            if result.is_ok() {
                return Ok(Expression::Immutable(name, Box::new(self.parse_ast(0)?)));
            }
        }

        Err(result.err().unwrap())
    }

    fn parse_if(&mut self) -> Return {
        if let Some(expr) = self.peek() {
            let op = match expr {
                Token::True => BoolOperation::Bool(true),
                Token::False => BoolOperation::Bool(false),
                _ => return Err(ErrorLang::Eof),
            };
            self.accept();
            self.accept_and_check_next(Token::Start)?;

            let mut if_true = Vec::new();
            let mut if_else = Vec::new();
            let mut is_else = false;
            while let Some(t) = self.peek() {
                if t == Token::Else {
                    is_else = true;
                    self.accept();
                    continue;
                }
                if t == Token::End {
                    break;
                }
                if is_else {
                    if_else.push(self.parse_ast(0)?);
                } else {
                    if_true.push(self.parse_ast(0)?);
                }
            }

            if self.peek() == Some(Token::End) {
                self.accept();
            } else {
                return Err(ErrorLang::Eof);
            }

            Ok(Expression::If(
                Box::new(op),
                Box::new(Expression::Block(if_true)),
                Box::new(Expression::Block(if_else)),
            ))
        } else {
            Ok(Expression::Eof)
        }
    }

    fn parse_lhs(&mut self, op: &Token) -> Return {
        let expr = match op {
            Token::True => Expression::Value(Scalar::Bool(true)),
            Token::False => Expression::Value(Scalar::Bool(false)),
            Token::Integer(number) => Expression::Value(Scalar::I64(*number)),
            Token::Float(number) => Expression::Value(Scalar::F64(*number)),
            Token::Decimal(decimal) => Expression::Value(Scalar::Decimal(*decimal)),
            Token::String(text) => Expression::Value(Scalar::UTF8(Rc::new(text.into()))),
            Token::Var => self.parse_var()?,
            Token::Let => self.parse_let()?,
            Token::Variable(name) => {
                //Check if is a function name...
                if self.peek() == Some(Token::LeftParentheses) {
                    self.parse_function_call(&name)?
                } else {
                    Expression::Variable(name.into())
                }
            }
            Token::If => self.parse_if()?,
            t => panic!("bad token: {:?}", t),
        };
        Ok(expr)
    }

    fn parse_ast(&mut self, min_bindpower: u8) -> Return {
        let op = self.accept();
        //dbg!(&op);
        let mut lhs = if let Some((op, _)) = op {
            self.parse_lhs(&op)?
        } else {
            return Ok(Expression::Eof);
        };

        //dbg!(&lhs);
        while let Some((token, data)) = self.peek_both() {
            //dbg!(&token);

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
                        } else {
                            return Err(ErrorLang::UnclosedGroup(Token::LeftParentheses, data));
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

                if token.is_binary_operator() {
                    lhs = Expression::BinaryOp(BinaryOperation::new(
                        token,
                        Box::new(lhs),
                        Box::new(rhs),
                    ));
                    continue;
                }

                if token.is_comparison_operator() {
                    lhs = Expression::ComparisonOp(ComparisonOperator::new(
                        token,
                        Box::new(lhs),
                        Box::new(rhs),
                    ));
                    continue;
                }

                lhs = Expression::Block(vec![lhs, rhs]);
                continue;
            }

            break;
        }

        Ok(lhs)
    }
}
