use crate::for_impl::*;
use crate::function::{FunCall, Function, FunctionDec};
use crate::prelude::*;

use derivative::Derivative;
use enum_map::EnumMap;

pub trait RelCallable: fmt::Debug {
    fn name(&self) -> &str;
    fn path(&self) -> &str;
    fn call(&self, named: &str, params: FunCall) -> ResultT<Scalar>;
    fn get(&self, named: &str) -> Option<&FunctionDec>;
    fn functions(&self) -> Box<dyn Iterator<Item = &FunctionDec> + '_>;
}

pub trait CallableStaticTraits:
    Clone + fmt::Debug + std::str::FromStr + enum_map::Enum<FunctionDec>
{
}

pub trait StaticCall<T: CallableStaticTraits> {
    fn call(&self, f: T, params: FunCall) -> ResultT<Scalar>;
}

// A registry for static functions imported from Rust
#[derive(Debug, Derivative)]
#[derivative(Hash, PartialEq, PartialOrd, Ord, Eq)]
pub struct CallableStatic<T: CallableStaticTraits> {
    name: String,
    path: String,
    #[derivative(
        Hash = "ignore",
        PartialEq = "ignore",
        PartialOrd = "ignore",
        Ord = "ignore"
    )]
    functions: EnumMap<T, FunctionDec>,
    #[derivative(
        Hash = "ignore",
        PartialEq = "ignore",
        PartialOrd = "ignore",
        Ord = "ignore"
    )]
    caller: Box<dyn StaticCall<T>>,
}

impl<T: CallableStaticTraits> CallableStatic<T> {
    pub fn new(
        name: &str,
        path: &str,
        functions: EnumMap<T, FunctionDec>,
        caller: Box<dyn StaticCall<T>>,
    ) -> Self {
        CallableStatic {
            name: name.to_string(),
            path: path.to_string(),
            functions,
            caller,
        }
    }

    pub fn get_fn(&self, named: &str) -> Option<(T, &FunctionDec)> {
        match T::from_str(named) {
            Ok(key) => Some((key.clone(), &self.functions[key])),
            _ => None,
        }
    }
}

impl<T: CallableStaticTraits> RelCallable for CallableStatic<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn path(&self) -> &str {
        &self.path
    }

    fn call(&self, named: &str, params: FunCall) -> ResultT<Scalar> {
        match self.get_fn(named) {
            Some((key, f)) => {
                f.check_call(&params)?;
                self.caller.call(key, params)
            }
            _ => Err(Error::FunctionNotFound(FunctionDec::new_for_not_found(
                named, params,
            ))),
        }
    }

    fn get(&self, named: &str) -> Option<&FunctionDec> {
        self.get_fn(named).map(|x| x.1)
    }

    fn functions(&self) -> Box<dyn Iterator<Item = &FunctionDec> + '_> {
        Box::new(self.functions.values())
    }
}

// A registry for functions made at runtime
#[derive(Debug)]
pub struct CallablDyn {
    name: String,
    path: String,
    functions: HashMap<String, Function>,
}

impl CallablDyn {
    pub fn new(name: &str, path: &str, functions: HashMap<String, Function>) -> Self {
        CallablDyn {
            name: name.to_string(),
            path: path.to_string(),
            functions,
        }
    }

    pub fn get_fn(&self, named: &str) -> Option<&Function> {
        self.functions.get(named)
    }
}

impl RelCallable for CallablDyn {
    fn name(&self) -> &str {
        &self.name
    }

    fn path(&self) -> &str {
        &self.path
    }

    fn call(&self, named: &str, params: FunCall) -> ResultT<Scalar> {
        match self.get_fn(named) {
            Some(f) => {
                f.head.check_call(&params)?;
                f.call(params)
            }
            _ => Err(Error::FunctionNotFound(FunctionDec::new_for_not_found(
                named, params,
            ))),
        }
    }

    fn get(&self, named: &str) -> Option<&FunctionDec> {
        self.get_fn(named).map(|x| &x.head)
    }

    fn functions(&self) -> Box<dyn Iterator<Item = &FunctionDec> + '_> {
        Box::new(self.functions.values().map(|x| &x.head))
    }
}

impl<T> fmt::Debug for dyn StaticCall<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PkgTraits")
    }
}
