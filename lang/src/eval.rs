use tablam::prelude::*;

use crate::prelude::*;

pub struct Program {
    env: Environment,
}

impl Program {
    pub fn eval_expr(&mut self, expr: Expression) -> Return {
        let expr = match expr {
            Expression::Pass => expr,
            Expression::Value(_) => expr,
            Expression::Eof => return Ok(expr),
            Expression::Variable(name, value) => {
                self.env.add_variable(name, *value);
                Expression::Pass
            }
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

pub fn value(of: Scalar) -> Expression {
    Expression::Value(of)
}
