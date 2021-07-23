use tablam::function::Function;
use tablam::interpreter::prelude::Mod;

pub mod basic;
pub mod io;
pub mod math;

pub fn std_functions() -> Vec<Mod> {
    let mut funs = Vec::new();

    //funs.extend_from_slice(&basic::functions());
    funs.push(math::functions());
    //funs.extend_from_slice(&io::functions());

    funs
}
