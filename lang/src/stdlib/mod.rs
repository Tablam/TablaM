use tablam::function::Function;

pub mod basic;
pub mod io;
pub mod math;

pub fn std_functions() -> Vec<Function> {
    let mut funs = Vec::new();

    // funs.extend_from_slice(&basic::functions());
    // funs.extend_from_slice(&math::functions());
    // funs.extend_from_slice(&io::functions());

    funs
}
