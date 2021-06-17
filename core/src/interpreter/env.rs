use crate::interpreter::ast::Expression;
use crate::interpreter::Identifier;
use crate::prelude::{Error, Function};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Environment {
    vars: HashMap<Identifier, Expression>,
    functions: HashMap<Identifier, Expression>,
    parent: Option<Box<Environment>>,
}

impl Environment {
    pub fn new(parent: Option<Box<Environment>>) -> Self {
        Environment {
            vars: HashMap::new(),
            functions: HashMap::new(),
            parent,
        }
    }

    pub fn add_variable(&mut self, name: String, value: Expression) {
        self.vars.insert(name, value);
    }

    pub fn add_function(&mut self, name: String, def: Expression) {
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
        Environment::new(Some(Box::new(self)))
    }
}
