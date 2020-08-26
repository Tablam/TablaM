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
            env.add_function(f.key(), Expression::Function(f));
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
                    BinOp::Add => "add_Int_Int",
                    BinOp::Minus => "minus_Int_Int",
                    BinOp::Mul => "mul_Int_Int",
                    BinOp::Div => "div_Int_Int",
                };
                let f = self.env().find_function(name).expect("Fail std");

                let lhs = self.eval_value(&op.left)?;
                let rhs = self.eval_value(&op.right)?;

                Expression::Value(f.call(&[&lhs, &rhs])?)
            }
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
