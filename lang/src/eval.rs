use tablam::prelude::*;

use crate::prelude::*;

pub struct Program {
    env: Environment,
}

impl Program {
    pub fn new() -> Self {
        let mut env = Environment::new(None);

        for f in tablam::stdlib::std_functions() {
            env.add_function(f.key(), Expression::Function(f));
        }

        Program { env }
    }

    pub fn execute_str(&mut self, _source: &str) -> Return {
        Ok(Expression::Pass)
    }

    pub fn eval_value(&mut self, _expr: &Expression) -> Result<&Scalar> {
        unimplemented!()
    }

    pub fn eval_expr(&mut self, expr: Expression) -> Return {
        let expr = match expr {
            Expression::Pass => expr,
            Expression::Value(_) => expr,
            Expression::Eof => return Ok(expr),
            Expression::Variable(name, value) => {
                self.env.add_variable(name, *value);
                Expression::Pass
            }
            Expression::BinaryOp(op) => match op.operator {
                Token::Plus => {
                    let f = self
                        .env
                        .find_function("math.add_Int_Int")
                        .expect("Fail std");
                    let _lhs = self.eval_value(&op.left)?;
                    let _rhs = self.eval_value(&op.right)?;

                    Expression::Value(f.call(&[])?)
                }
                _ => unimplemented!(),
            },
            _x => unimplemented!(),
        };
        Ok(expr)
    }

    pub fn eval(&mut self, ast: impl Iterator<Item = Expression>) -> Return {
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
