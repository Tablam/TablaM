use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::for_impl::*;
use crate::interpreter::Identifier;
use crate::prelude::*;

//use crate::interpreter::env::Env;
use crate::interpreter::core::mod_ops;
use crate::interpreter::modules::{CmdBox, Mod};

use crate::interpreter::ast::{BinaryOperation, Block};
use crate::interpreter::code::{BoolOp, Code, Lines};
use crate::interpreter::prelude::{BoolOperation, Expr};
use crate::interpreter::visitor::Visitor;
use std::ops::Deref;

#[derive(Clone)]
pub struct Env {
    vars: HashMap<Identifier, Code>,
    mods: Vec<String>,
    parent: Option<Box<Env>>,
}

impl Env {
    pub fn new(parent: Option<Box<Env>>) -> Self {
        Env {
            vars: HashMap::new(),
            mods: vec!["std.ops".into()],
            parent,
        }
    }

    pub fn add_variable(&mut self, name: String, value: Code) {
        self.vars.insert(name, value);
    }

    pub fn find_variable(&self, name: &str) -> ResultT<&Code> {
        match self.vars.get(name) {
            Some(variable) => Ok(variable),
            None => match &self.parent {
                Some(env) => env.find_variable(name),
                None => Err(Error::VariableNotFound(name.to_string())),
            },
        }
    }

    pub fn find_mod(&self, name: &str) -> (bool, String) {
        let parts: Vec<_> = name.split('.').collect();
        let name: String = parts[..parts.len() - 1].join(".");
        dbg!(&name);
        (self.mods.contains(&name), name)
    }

    pub fn find_function<'a, 'b>(
        &'a self,
        program: &'b Program,
        name: &'a str,
    ) -> ResultT<&'b CmdBox> {
        let (exist, pkg_name) = self.find_mod(name);
        if exist {
            let pkg = &program.mods[&pkg_name];
            if let Some(cmd) = pkg.get(name) {
                return Ok(cmd);
            }
        };
        Err(Error::VariableNotFound(name.to_string()))
    }
}

pub struct Program {
    /// A flat list of all modules, in each env is located the ones on scope...
    mods: HashMap<String, Mod>,
    env: Rc<RwLock<Env>>,
}

impl Program {
    pub fn new() -> Self {
        let env = Env::new(None);
        let prelude = mod_ops();
        let mut mods = HashMap::with_capacity(10);

        mods.insert(prelude.name.clone(), prelude);

        Program {
            mods,
            env: Rc::new(RwLock::new(env)),
        }
    }

    fn env(&self) -> RwLockReadGuard<'_, Env> {
        self.env.read().expect("Env Read is Poisoned")
    }

    fn env_mut(&self) -> RwLockWriteGuard<'_, Env> {
        self.env.write().expect("Env Write is Poisoned")
    }

    fn add_module(&mut self, pkg: Mod) {
        self.mods.insert(pkg.name.clone(), pkg);
    }

    fn decode_bool(&self, value: &BoolOperation) -> ResultT<BoolOp> {
        match value {
            BoolOperation::Bool(x) => Ok(BoolOp::Bool(*x)),
            BoolOperation::Var(name) => match self.env().find_variable(name)? {
                Code::Value(Scalar::Bool(x)) => Ok(BoolOp::Bool(*x)),
                Code::Bool(x) => Ok(x.clone()),
                _ => unreachable!(),
            },
            BoolOperation::Cmp(_cmp) => {
                // let a = self.eval_value(&cmp.left)?;
                // let b = self.eval_value(&cmp.right)?;
                // Ok(match cmp.operator {
                //     LogicOp::Equal => a == b,
                //     LogicOp::NotEqual => a != b,
                //     LogicOp::Less => a < b,
                //     LogicOp::LessEqual => a <= b,
                //     LogicOp::Greater => a > b,
                //     LogicOp::GreaterEqual => a >= b,
                //     LogicOp::And => unreachable!(),
                //     LogicOp::Or => unreachable!(),
                // })
                unimplemented!()
            }
        }
    }

    pub fn eval_value(&self, expr: &Expr) -> ResultT<Code> {
        match expr {
            Expr::Value(x) => Ok(Code::Value(x.clone())),
            Expr::Variable(name) => {
                let x = self.env().find_variable(name.as_str())?.clone();
                Ok(x)
            }
            _expr => unimplemented!(),
        }
    }

    pub fn eval_expr(&self, expr: Expr) -> ResultT<Code> {
        //dbg!(&expr);
        let expr = match expr {
            Expr::Pass => Code::Pass,
            Expr::Value(x) => Code::Value(x),
            Expr::Eof => Code::Pass,
            Expr::Block(lines) => {
                let mut last = None;
                for line in lines.0 {
                    last = Some(self.eval_expr(line)?);
                }

                last.unwrap_or(Code::Pass)
            }
            Expr::Mutable(name, value) => {
                let value = self.eval_expr(*value)?;
                self.env_mut().add_variable(name, value);
                Code::Pass
            }
            Expr::Immutable(name, value) => {
                let value = self.eval_expr(*value)?;
                self.env_mut().add_variable(name, value);
                Code::Pass
            }
            Expr::Variable(name) => self.env().find_variable(name.as_str())?.clone(),
            x => unimplemented!("{}", x),
        };
        Ok(expr)
    }

    pub fn _compile(&mut self, ast: Expr, code: &mut Vec<Code>) -> ResultT<()> {
        match ast {
            Expr::Value(x) => code.push(Code::Value(x)),
            Expr::Block(lines) => {
                let mut block = Vec::with_capacity(lines.0.len());

                for line in lines.0 {
                    self._compile(line, &mut block)?
                }

                code.push(Code::Block(Lines(block)));
            }
            Expr::If(test, if_true, if_false) => {
                let test = self.decode_bool(&test)?;
                let mut lhs = Vec::new();
                let mut rhs = Vec::new();
                self._compile(*if_true, &mut lhs)?;
                self._compile(*if_false, &mut rhs)?;

                code.push(Code::If(
                    test,
                    Box::new(lhs[0].clone()),
                    Box::new(rhs[0].clone()),
                ));
            }
            Expr::Variable(named) => {
                let x = self.env().find_variable(&named)?.clone();
                code.push(x);
            }
            Expr::Immutable(name, value) => {
                let mut block = Vec::with_capacity(1);
                self._compile(*value, &mut block)?;
                self.env_mut().add_variable(name, block[0].clone());
            }
            Expr::BinaryOp(op) => {
                let named = match op.operator {
                    BinOp::Add => "std.ops.add",
                    BinOp::Minus => "std.ops.minus",
                    BinOp::Mul => "std.ops.mul",
                    BinOp::Div => "std.ops.div",
                };
                let cmd = self.env().find_function(&self, named)?.clone();

                let mut lhs = Vec::new();
                let mut rhs = Vec::new();

                self._compile(*op.left, &mut lhs)?;
                self._compile(*op.right, &mut rhs)?;

                code.push(Code::BinOp(cmd, lhs, rhs));
            }
            _ => {
                unimplemented!()
            }
        }

        Ok(())
    }

    pub fn compile(&mut self, ast: Expr) -> ResultT<Code> {
        let mut code = Vec::with_capacity(1024);

        self._compile(ast, &mut code)?;

        Ok(Code::Block(Lines(code)))
    }

    pub fn execute_lines(&mut self, lines: Vec<Code>) -> ResultT<Scalar> {
        let mut last = Scalar::Unit;
        for line in lines {
            last = self.execute(line)?
        }
        Ok(last)
    }

    pub fn execute(&mut self, code: Code) -> ResultT<Scalar> {
        match code {
            Code::Value(x) => Ok(x),
            Code::If(test, if_true, if_false) => match test {
                BoolOp::Bool(x) => {
                    let code = if x { if_true } else { if_false };
                    self.execute_lines(vec![*code])
                }
                BoolOp::Cmp(x) => (x).execute(self.env().deref()),
            },
            Code::Block(lines) => self.execute_lines(lines.0),
            Code::Code(x) => (x).execute(self.env().deref()),
            Code::Pass => Ok(Scalar::Unit),
            Code::Bool(_) => self.execute(code),
            Code::BinOp(cmd, lhs, rhs) => {
                let x = self.execute_lines(lhs)?;
                let y = self.execute_lines(rhs)?;
                cmd.call(&[x, y])
            }
        }
    }
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Compiler<'a> {
    of: &'a Program,
}

impl<'a> Compiler<'a> {
    pub fn new(of: &'a Program) -> Self {
        Compiler { of }
    }
}

impl Visitor<ResultT<Code>> for Compiler<'_> {
    fn visit_scalar(&mut self, of: &Scalar) -> ResultT<Code> {
        Ok(Code::Value(of.clone()))
    }

    fn visit_block(&mut self, of: &Block) -> ResultT<Code> {
        let mut last = Code::Pass;
        for line in &of.0 {
            last = self.visit_expr(line)?;
        }
        Ok(last)
    }

    fn visit_get_var(&mut self, of: &Identifier) -> ResultT<Code> {
        let x = self.of.env().find_variable(of)?.clone();
        Ok(x)
    }

    fn visit_let(&mut self, named: &Identifier, of: &Expr) -> ResultT<Code> {
        let x = self.visit_expr(of)?;
        self.of.env_mut().add_variable(named.into(), x);
        Ok(Code::Pass)
    }

    fn visit_var(&mut self, named: &Identifier, of: &Expr) -> ResultT<Code> {
        self.visit_let(named, of)
    }

    fn visit_bin_op(&mut self, of: &BinaryOperation) -> ResultT<Code> {
        todo!()
    }

    fn visit_bool_op(&mut self, test: &BoolOperation) -> ResultT<Code> {
        match test {
            BoolOperation::Bool(x) => Ok(Code::Bool(BoolOp::Bool(*x))),
            BoolOperation::Var(named) => {
                unimplemented!()
            }
            BoolOperation::Cmp(_) => {
                unimplemented!()
            }
        }
    }

    fn visit_if(&mut self, test: &BoolOperation, if_true: &Expr, if_false: &Expr) -> ResultT<Code> {
        let test = self.visit_bool_op(test)?;
        if let Some(ok) = test.as_bool() {
            if ok {
                self.visit_expr(if_true)
            } else {
                self.visit_expr(if_false)
            }
        } else {
            // Ok(Code::If(
            //     BoolOp::Cmp(test),
            //     Box::new(self.visit_expr(if_true)?),
            //     Box::new(self.visit_expr(if_false)?),
            // ))
            unimplemented!()
        }
    }

    fn visit_expr(&mut self, of: &Expr) -> ResultT<Code> {
        todo!()
    }
}
mod test {
    use super::*;
    use crate::interpreter::dsl::*;

    #[test]
    fn test_compile() {
        let sum = plus(value(1), value(2));

        let start = set_i("x", sum);

        let check = if_(op_bool(true), value("hello"), value(2));
        let end = get("x");
        let ast = block(vec![start, check, end]);

        let mut exe = Program::new();

        let code = exe.compile(ast).unwrap();
        dbg!(&code);
        dbg!(exe.execute(code).unwrap());
    }
}
