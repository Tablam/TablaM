use crate::prelude::*;

fn print(of: &[Scalar]) -> Result<Scalar> {
    for x in of {
        print!("{}", x);
    }
    Ok(Scalar::None)
}

fn print_ln(of: &[Scalar]) -> Result<Scalar> {
    for x in of {
        println!("{}", x);
    }
    Ok(Scalar::None)
}

fn basic_fn_variadic(name: &str, kind: DataType, f: RelFun) -> Function {
    Function::new_single(
        name,
        Param::kind(DataType::Variadic(Box::new(kind))),
        DataType::ANY,
        Box::new(f),
    )
}

pub fn basic_functions() -> Vec<Function> {
    vec![
        basic_fn_variadic("print", DataType::ANY, print),
        basic_fn_variadic("println", DataType::ANY, print_ln),
    ]
}
