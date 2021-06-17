use crate::for_impl::*;
use crate::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub struct CmdName<'a> {
    pkg: &'a str,
    name: &'a str,
}

impl<'a> CmdName<'a> {
    pub fn new(pkg: &'a str, name: &'a str) -> Self {
        CmdName { pkg, name }
    }

    pub fn full_name(&self) -> String {
        format!("{}.{}", self.pkg, self.name)
    }
}

pub trait Cmd {
    /// The full module + name which invoke this.
    /// Ej: CmdName('std.math', 'add')
    fn name(&self) -> CmdName;
    /// Clones the command in a box reference.
    fn clone_and_box(&self) -> CmdBox;
    /// Command documentation (In https://commonmark.org)
    fn help(&self) -> Option<String> {
        None
    }
    /// Define the matching types for multi-dispatch
    fn types(&self) -> &[&[DataType]];
    /// Verify if the params match the dispatch table and return false if not.
    fn is_dispatchable(&self, params: &[Scalar]) -> ResultT<()> {
        let types: Vec<_> = params
            .iter()
            .map(|x| x.kind_scalar().unwrap_or(DataType::Unit))
            .collect();
        if self.types().iter().find(|x| **x == &types).is_some() {
            Ok(())
        } else {
            unimplemented!()
        }
    }
    /// Call the cmd. WARNING: This is safe only after type-checking by the lang & using Self::is_dispatchable
    fn call(&self, params: &[Scalar]) -> ResultT<Scalar>;
}

/// Defines a box reference for a command.
pub type CmdBox = Box<dyn Cmd>;

impl Clone for Box<dyn Cmd> {
    fn clone(&self) -> Box<dyn Cmd> {
        self.clone_and_box()
    }
}

pub struct Mod {
    pub commands: HashMap<String, CmdBox>,
}

impl Mod {
    pub fn new(cmd: &[CmdBox]) -> Self {
        let commands = cmd
            .iter()
            .map(|x| (x.name().full_name(), x.clone()))
            .collect();

        Mod { commands }
    }

    /// Return the command based on the FULL name (ie: std.math.add)
    pub fn get(&self, name: &str) -> Option<&CmdBox> {
        self.commands.get(name)
    }

    /// Return the command based on the FULL name (ie: std.math.add)
    pub fn call(&self, name: &str, params: &[Scalar]) -> ResultT<Scalar> {
        if let Some(cmd) = self.commands.get(name) {
            cmd.is_dispatchable(&params)?;
            cmd.call(params)
        } else {
            unimplemented!()
        }
    }
}

#[macro_export]
macro_rules! cmd_impl {
    ($pkg:literal, $name:literal, $dispatch:ident) => {
        fn name(&self) -> CmdName<'_> {
            CmdName::new($pkg, $name)
        }

        fn clone_and_box(&self) -> CmdBox {
            Box::new((*self).clone())
        }

        fn types(&self) -> &[&[DataType]] {
            &$dispatch
        }
    };
}

/// This define the dispatch table for commands. Is assumed the last one is the return.
pub const BIN_I64: &'static [DataType] = &[DataType::I64, DataType::I64, DataType::I64];
pub const BIN_F64: &'static [DataType] = &[DataType::F64, DataType::F64, DataType::F64];
pub const BIN_D64: &'static [DataType] = &[DataType::Decimal, DataType::Decimal, DataType::Decimal];

/// The dispatch for binary operations like +, *, -, /
pub const BIN_MATH: &'static [&'static [DataType]] = &[BIN_I64, BIN_D64, BIN_F64];
