use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use tablam::prelude::*;

use crate::parser::Parser;
use crate::prelude::*;

pub struct Program {
    env: Rc<RefCell<Environment>>,
}

impl Program {
    pub fn new() -> Self {
        let mut env = Environment::new(None);

        for f in tablam::stdlib::std_functions() {
            env.add_function(f.name.clone(), Expression::Function(f));
        }

        Program {
            env: Rc::new(RefCell::new(env)),
        }
    }

    fn env(&self) -> Ref<'_, Environment> {
        self.env.borrow()
    }

    fn env_mut(&self) -> RefMut<'_, Environment> {
        self.env.borrow_mut()
    }

    fn decode_bool(&self, value: &BoolOperation) -> ReturnT<bool> {
        match value {
            BoolOperation::Bool(x) => Ok(*x),
            BoolOperation::Var(name) => {
                let x = self.eval_value(&Expression::Variable(name.into()))?;
                if let Scalar::Bool(x) = x {
                    Ok(x)
                } else {
                    Err(ErrorLang::Eof)
                }
            }
            BoolOperation::Cmp(cmp, lhs, rhs) => {
                let a = self.eval_value(&lhs)?;
                let b = self.eval_value(&rhs)?;
                Ok(match cmp {
                    CmOp::Eq => a == b,
                    CmOp::NotEq => a != b,
                    CmOp::Less => a < b,
                    CmOp::LessEq => a <= b,
                    CmOp::Greater => a > b,
                    CmOp::GreaterEq => a >= b,
                })
            }
        }
    }
    pub fn execute_str(&self, source: &str) -> Return {
        let mut parser = Parser::new(source);
        self.eval_expr(parser.parse()?)
    }

    pub fn eval_value(&self, expr: &Expression) -> ReturnT<Scalar> {
        match expr {
            Expression::Value(x) => Ok(x.clone()),
            Expression::Variable(name) => {
                let expr = self.env().find_variable(name.as_str())?.clone();
                let expr = &self.eval_expr(expr)?;
                self.eval_value(expr)
            }
            err => unreachable!("{}", err),
        }
    }

    pub fn eval_expr(&self, expr: Expression) -> Return {
        let expr = match expr {
            Expression::Pass => expr,
            Expression::Value(_) => expr,
            Expression::Eof => return Ok(expr),
            Expression::Mutable(name, value) => {
                self.env_mut().add_variable(name, *value);
                Expression::Pass
            }
            Expression::Immutable(name, value) => {
                self.env_mut().add_variable(name, *value);
                Expression::Pass
            }
            Expression::Variable(name) => self.env().find_variable(name.as_str())?.clone(),
            Expression::BinaryOp(op) => {
                let name = match op.operator {
                    Token::Plus => "add",
                    Token::Minus => "minus",
                    Token::Multiplication => "mul",
                    Token::Division => "div",
                    _ => unreachable!(),
                };
                let f = self.env().find_function(name)?;

                let lhs = self.eval_value(&op.left)?;
                let rhs = self.eval_value(&op.right)?;

                Expression::Value(f.call(&[lhs, rhs])?)
            }
            Expression::FunctionCall(call) => {
                let f = self.env().find_function(&call.name)?;
                let mut params = Vec::with_capacity(call.params.len());
                //TODO: Check validity of params
                for p in call.params {
                    let expr = self.eval_value(&p.value)?;
                    params.push(expr);
                }
                let result = f.call(params.as_slice())?;
                match result {
                    Scalar::None => Expression::Pass,
                    expr => Expression::Value(expr),
                }
            }
            Expression::If(check, if_true, if_false) => {
                if self.decode_bool(&check)? {
                    *if_true
                } else {
                    *if_false
                }
            }
            Expression::While(_check, _body) => unimplemented!(),
            _x => unimplemented!(),
        };
        Ok(expr)
    }

    pub fn eval(&self, ast: impl Iterator<Item = Expression>) -> Return {
        let mut last = Expression::Eof;
        for expr in ast {
            last = self.eval_expr(expr)?
        }
        Ok(last)
    }
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}

pub fn value(of: Scalar) -> Expression {
    Expression::Value(of)
}
