#![allow(dead_code)]

use std::mem::discriminant;
use std::rc::Rc;

use crate::ast::*;
use crate::lexer::*;
use tablam::prelude::{DataType, Field, Param, Scalar, Schema, Vector};

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

    fn check_next_token(&mut self, expected: Token) -> std::result::Result<Token, Error> {
        if let Some((found, data)) = self.peek_both() {
            return if discriminant(&found) == discriminant(&expected) {
                self.accept();
                Ok(found)
            } else {
                let feedback = Error::Unexpected(found, expected, data);
                Err(feedback)
            };
        }

        Err(Error::Eof)
    }

    fn check_and_accept_next(&mut self, expected: Token) -> std::result::Result<Token, Error> {
        let result = self.check_next_token(expected);
        match result {
            Ok(token) => Ok(token),
            Err(error) => {
                self.accept();
                Err(error)
            }
        }
    }

    fn match_at_least_one(&mut self, conditions: Vec<Token>) -> std::result::Result<Token, Error> {
        let mut result = Err(Error::Eof);
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

    fn parse_ast(&mut self, min_bindpower: u8) -> Return {
        let op = self.accept();
        //dbg!(&op);
        let mut lhs = match op {
            Some((Token::True, _)) => Expression::Value(Scalar::Bool(true)),
            Some((Token::False, _)) => Expression::Value(Scalar::Bool(false)),
            Some((Token::Integer(number), _)) => Expression::Value(Scalar::I64(number)),
            Some((Token::Float(number), _)) => Expression::Value(Scalar::F64(number)),
            Some((Token::Decimal(decimal), _)) => Expression::Value(Scalar::Decimal(decimal)),
            Some((Token::String(text), _)) => Expression::Value(Scalar::UTF8(Rc::new(text))),
            Some((Token::Var, _)) => self.parse_var()?,
            Some((Token::Let, _)) => self.parse_let()?,
            Some((Token::Variable(name), _)) => {
                match self.peek() {
                    //Check if is a function name...
                    Some(Token::LeftParentheses) => self.parse_function_call(&name)?,
                    Some(Token::TypeDefiner) => self.parse_parameter_definition(&name)?,
                    _ => Expression::Variable(name),
                }
            }
            Some((Token::StartVector, _)) => self.parse_vector()?,
            t => panic!("bad token: {:?}", t),
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
                            return Err(Error::UnclosedGroup(Token::LeftParentheses, data));
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

    fn parse_var(&mut self) -> Return {
        let result = self.check_next_token(Token::Variable("".to_string()));
        if let Ok(Token::Variable(name)) = result {
            self.check_next_token(Token::Assignment)?;

            return Ok(Expression::Mutable(name, Box::new(self.parse_ast(0)?)));
        }

        Err(result.err().unwrap())
    }

    fn parse_let(&mut self) -> Return {
        let dummy = String::from("");
        let result =
            self.match_at_least_one(vec![Token::Variable(dummy.clone()), Token::Constant(dummy)]);
        if let Ok(Token::Variable(name)) | Ok(Token::Constant(name)) = result {
            self.check_next_token(Token::Assignment)?;

            return Ok(Expression::Immutable(name, Box::new(self.parse_ast(0)?)));
        }

        Err(result.err().unwrap())
    }

    fn parse_parameter_definition(&mut self, name: &str) -> Return {
        self.accept();
        let result = self.check_and_accept_next(Token::Type(String::from("")));
        if let Ok(Token::Type(type_param)) = result {
            return Ok(Expression::ParameterDefinition(Param::from_str(
                name,
                type_param.as_str(),
            )));
        }

        Err(result.err().unwrap())
    }

    fn parse_param_call(&mut self) -> std::result::Result<Option<ParamCall>, Error> {
        if let Some((token, _data)) = self.accept() {
            //dbg!(&token);
            if token.is_literal_or_value() {
                let expr = self.parse_ast(0)?;
                Ok(Some(ParamCall::new("", expr)))
            } else {
                Err(Error::Eof)
            }
        } else {
            Err(Error::Eof)
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

    fn parse_vector(&mut self) -> Return {
        let mut fields = Vec::<Field>::new();
        let mut data = Vec::<Scalar>::new();
        loop {
            //if empty vector or ends in ; or ,
            match self.peek() {
                Some(Token::EndVector) => {
                    self.accept();
                    break;
                }
                _ => (),
            }

            //dbg!("other");
            let cell = self.parse_ast(0)?;
            match cell.clone() {
                Expression::ParameterDefinition(field) => fields.push(field.into()),
                Expression::Value(scalar) => data.push(scalar),
                _ => return Err(Error::UnexpectedItem(cell)),
            }
            //dbg!(cell);
            if Token::EndVector
                == self.match_at_least_one(vec![
                    Token::Separator,
                    Token::RowSeparator,
                    Token::EndVector,
                ])?
            {
                break;
            };
        }

        if data.is_empty() {
            return Ok(Expression::Value(Scalar::Vector(Rc::new(
                Vector::new_empty(DataType::ANY),
            ))));
        }

        if fields.len() > 1 {
            let schema = Schema::new(fields, None);
            return Ok(Expression::Value(Scalar::Vector(Rc::new(
                Vector::new_table(data, schema),
            ))));
        }

        let kind = data.first().expect("empty vector").kind();
        Ok(Expression::Value(Scalar::Vector(Rc::new(
            Vector::new_vector(data, kind),
        ))))
    }
}
