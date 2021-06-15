use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use tablam::prelude::*;

use crate::parser::Parser;
use crate::prelude::*;
use tablam::function::FunCall;

pub struct Program {
    env: Rc<RefCell<Environment>>,
}

impl Program {
    pub fn new() -> Self {
        let mut env = Environment::new(None);

        for f in stdlib::std_functions() {
            env.add_function(f.head.name.clone(), Expression::Function(f.into()));
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
            BoolOperation::Cmp(cmp) => {
                let a = self.eval_value(&cmp.left)?;
                let b = self.eval_value(&cmp.right)?;
                Ok(match cmp.operator {
                    LogicOp::Equal => a == b,
                    LogicOp::NotEqual => a != b,
                    LogicOp::Less => a < b,
                    LogicOp::LessEqual => a <= b,
                    LogicOp::Greater => a > b,
                    LogicOp::GreaterEqual => a >= b,
                    LogicOp::And => unreachable!(),
                    LogicOp::Or => unreachable!(),
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
            expr => self.eval_value(&self.eval_expr(expr.clone())?),
        }
    }

    pub fn eval_expr(&self, expr: Expression) -> Return {
        //dbg!(&expr);
        let expr = match expr {
            Expression::Pass => expr,
            Expression::Value(_) => expr,
            Expression::Eof => return Ok(expr),
            Expression::Block(lines) => {
                let mut last = None;
                for line in lines.0 {
                    last = Some(self.eval_expr(line)?);
                }

                last.unwrap_or(Expression::Pass)
            }
            Expression::Mutable(name, value) => {
                let value = self.eval_expr(*value)?;
                self.env_mut().add_variable(name, value);
                Expression::Pass
            }
            Expression::Immutable(name, value) => {
                let value = self.eval_expr(*value)?;
                self.env_mut().add_variable(name, value);
                Expression::Pass
            }
            Expression::Variable(name) => self.env().find_variable(name.as_str())?.clone(),
            Expression::BinaryOp(op) => {
                let name = match op.operator {
                    BinOp::Add => "add",
                    BinOp::Minus => "minus",
                    BinOp::Mul => "mul",
                    BinOp::Div => "div",
                };
                let f = self.env().find_function(name)?;

                let lhs = self.eval_value(&op.left)?;
                let rhs = self.eval_value(&op.right)?;

                Expression::Value(f.call(FunCall::Binary(&lhs, &rhs))?)
            }
            Expression::FunctionCall(call) => {
                let f = self.env().find_function(&call.name)?;
                let mut params = Vec::with_capacity(call.params.len());
                //TODO: Check validity of params
                for p in call.params {
                    let expr = self.eval_value(&p.value)?;
                    params.push(expr);
                }
                let result = f.call(FunCall::Many(params.as_slice()))?;
                match result {
                    Scalar::Unit => Expression::Pass,
                    expr => Expression::Value(expr),
                }
            }
            Expression::If(check, if_true, if_false) => {
                if self.decode_bool(&check)? {
                    self.eval_expr(*if_true)?
                } else {
                    self.eval_expr(*if_false)?
                }
            }
            Expression::While(check, body) => {
                while self.decode_bool(&check)? {
                    self.eval_expr(*body.clone())?;
                }
                Expression::Pass
            }
            Expression::ForIn(range, body) => {
                let RangeOperation::StartEnd(name, start, end) = *range;

                for i in start..end {
                    self.env_mut()
                        .add_variable(name.clone(), Expression::Value(Scalar::I64(i)));
                    self.eval_expr(*body.clone())?;
                }
                Expression::Pass
            }
            Expression::QueryOperation(query) => {
                let rel = self.eval_value(&query.collection)?;
                let mut q = query.query;
                q.schema = rel.schema();
                let q = q.execute(rel.rows());
                let rel = Vector::from_query(q);
                Expression::Value(Scalar::Vector(Rc::new(rel)))
            }
            x => unimplemented!("{}", x),
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
