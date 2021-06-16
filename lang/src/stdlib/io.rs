// use std::fs;
// use std::io::prelude::*;
// use std::io::{BufReader, SeekFrom};
// use std::path::PathBuf;
//
// use console::Term;
// use tablam::derive_more::{Display, From};
//
//use crate::for_impl::*;
// use crate::stdlib::io;
//
// use crate::prelude::FunctionDef;
// use tablam::for_impl::*;
use tablam::function::Function;
// use tablam::prelude::*;
//
// pub struct Console {
//     _out: Term,
//     _err: Term,
// }
//
// impl Console {
//     pub fn new() -> Self {
//         Console {
//             _out: Term::buffered_stdout(),
//             _err: Term::buffered_stderr(),
//         }
//     }
//
//     pub fn write_line(&self, of: &str) -> ResultT<()> {
//         self._out.write_line(of)?;
//         Ok(())
//     }
//
//     pub fn write_str(&self, of: &str) -> ResultT<()> {
//         self._out.write_str(of)?;
//         Ok(())
//     }
// }
//
// impl Default for io::Console {
//     fn default() -> Self {
//         Self::new()
//     }
// }
//
// fn open(of: &[Scalar]) -> ResultT<Scalar> {
//     if let Scalar::Utf8(name) = &of[0] {
//         let f = File::new(name.as_str().into(), true, false, false)?;
//         Ok(Scalar::File(Box::from(f)))
//     } else {
//         Err(Error::ParamTypeMismatch("open".into()))
//     }
// }
//
// fn read_to_string(of: &[Scalar]) -> ResultT<Scalar> {
//     if let Scalar::File(mut f) = of[0].clone() {
//         let s = f.read_to_string()?;
//         return Ok(s.into());
//     };
//     Err(Error::ParamTypeMismatch("read_to_string".into()))
// }
//
// fn save_file(of: &[Scalar]) -> ResultT<Scalar> {
//     if of.len() != 2 {
//         return Err(Error::ParamCount(of.len(), 2));
//     }
//
//     if let Scalar::File(mut f) = of[0].clone() {
//         let s = format!("{}", &of[1]);
//         f.write_string(&s)?;
//
//         return Ok(Scalar::Unit);
//     };
//     Err(Error::ParamTypeMismatch("save_file".into()))
// }
//
// fn fn_open(name: &str, params: &[Field], f: Box<dyn RelFun>) -> Function {
//     let head =
//         tablam::function::FunctionDec::new(name, params, &Field::new_positional(DataType::File));
//     Function::new(head, f)
// }

pub fn functions() -> Vec<Function> {
    vec![
        // fn_open("open", &[Field::new_positional(DataType::Utf8)], open),
        // fn_open(
        //     "read_to_string",
        //     &[Field::new_positional(DataType::Utf8)],
        //     read_to_string,
        // ),
        // fn_open(
        //     "save",
        //     &[
        //         Field::new_positional(DataType::Any),
        //         Field::new_positional(DataType::Utf8),
        //     ],
        //     save_file,
        // ),
    ]
}
