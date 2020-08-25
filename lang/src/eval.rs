use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use tablam::prelude::*;

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

    pub fn execute_str(&self, _source: &str) -> Return {
        Ok(Expression::Pass)
    }

    pub fn eval_value<'a>(&self, expr: &'a Expression) -> ReturnT<&'a Scalar> {
        match expr {
            Expression::Value(x) => Ok(x),
            _ => unreachable!(),
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
            Expression::BinaryOp(op) => match op.operator {
                Token::Plus => {
                    let f = self
                        .env()
                        .find_function("math.add_Int_Int")
                        .expect("Fail std");

                    let lhs = self.eval_value(&op.left)?;
                    let rhs = self.eval_value(&op.right)?;

                    Expression::Value(f.call(&[lhs, rhs])?)
                }
                _ => unimplemented!(),
            },
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
