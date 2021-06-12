// //#![allow(clippy::unnecessary_wraps)]
use crate::prelude::*;

use strum_macros::EnumString;

#[derive(Debug, Clone, Copy, Enum, EnumString)]
enum Functions {
    Print,
    PrintLn,
}

impl CallableStaticTraits for Functions {}

fn functions() -> CallableStatic<Functions> {
    CallableStatic::new(
        "std.prelude",
        "__internal",
        enum_map! {
            Functions::Print => fun_variadic("print", DataType::Any),
             Functions::PrintLn => fun_variadic("print_ln", DataType::Any),
        },
        Box::new(Caller {}),
    )
}

fn print(of: &[Scalar]) -> ResultT<Scalar> {
    for x in of {
        print!("{}", x);
    }
    Ok(Scalar::Unit)
}

fn print_ln(of: &[Scalar]) -> ResultT<Scalar> {
    for x in of {
        println!("{}", x);
    }
    Ok(Scalar::Unit)
}

//

//
// pub fn min(params: &[Scalar]) -> Result<Scalar> {
//     let value = &params[0];
//     if value.rows().unwrap_or(0) != 0 {
//         fold(Scalar::Top, params, fn_min)
//     } else {
//         Ok(Scalar::Unit)
//     }
// }
//
// pub fn max(params: &[Scalar]) -> Result<Scalar> {
//     let value = &params[0];
//     if value.rows().unwrap_or(0) != 0 {
//         fold(Scalar::Unit, params, fn_max)
//     } else {
//         Ok(Scalar::Unit)
//     }
// }
//
// fn basic_fn_variadic(name: &str, kind: DataType, f: RelFun) -> Function {
//     Function::new_single(
//         name,
//         Param::kind(DataType::Variadic(Box::new(kind))),
//         DataType::Any,
//         Box::new(f),
//     )
// }
//
// fn cmp_values(name: &str, param: DataType, f: RelFun) -> Function {
//     Function::new_single(name, Param::kind(param.clone()), param, Box::new(f))
// }
//
// pub fn functions() -> Vec<Function> {
//     vec![
//         basic_fn_variadic("print", DataType::Any, print),
//         basic_fn_variadic("println", DataType::Any, print_ln),
//         cmp_values("min", DataType::Any, min),
//         cmp_values("max", DataType::Any, max),
//     ]
// }

#[derive(Clone, Copy)]
struct Caller {}

impl StaticCall<Functions> for Caller {
    fn call(&self, f: Functions, params: &[Scalar]) -> ResultT<Scalar> {
        match f {
            Functions::Print => print(params),
            Functions::PrintLn => print_ln(params),
        }
    }
}
