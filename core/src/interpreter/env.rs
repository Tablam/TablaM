use crate::interpreter::ast::Expr;
use crate::interpreter::Identifier;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Env {
    vars: HashMap<Identifier, Expr>,
    functions: HashMap<Identifier, Expr>,
    parent: Option<Box<Env>>,
}

impl Env {
    pub fn new(parent: Option<Box<Env>>) -> Self {
        Env {
            vars: HashMap::new(),
            functions: HashMap::new(),
            parent,
        }
    }

    pub fn add_variable(&mut self, name: String, value: Expr) {
        self.vars.insert(name, value);
    }

    pub fn add_function(&mut self, name: String, def: Expr) {
        self.functions.insert(name, def);
    }
    //
    // pub fn find_variable(&self, name: &str) -> Result<&Expression, Error> {
    //     match self.vars.get(name) {
    //         Some(variable) => Ok(variable),
    //         None => match &self.parent {
    //             Some(env) => env.find_variable(name),
    //             None => Err(Error::VariableNotFound(name.to_string())),
    //         },
    //     }
    // }
    //
    // pub fn find_function(&self, name: &str) -> Result<Function, Error> {
    //     match self.functions.get(name) {
    //         Some(function) => {
    //             if let Expression::Function(f) = function {
    //                 Ok(f.clone())
    //             } else {
    //                 Err(Error::FunctionNotFound(name.to_string()))
    //             }
    //         }
    //         None => match &self.parent {
    //             Some(env) => env.find_function(name),
    //             None => Err(Error::FunctionNotFound(name.to_string())),
    //         },
    //     }
    // }

    pub fn create_child(self) -> Self {
        Env::new(Some(Box::new(self)))
    }
}
