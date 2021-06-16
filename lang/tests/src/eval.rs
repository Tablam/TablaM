use tablam_lang::ast::Return;
use tablam_lang::eval::Program;

fn run(source: &str) -> Return {
    let p = Program::new();
    p.execute_str(source)
}

fn run_ok(source: &str, expected: &str) {
    let expr = run(source).unwrap();
    let expr = format!("{}", expr);
    dbg!(source);
    assert_eq!(expected, expr.trim());
}

//#[test]
fn test_simple_math() {
    run_ok("1", "1");
    run_ok("1 + 1", "2");
    run_ok("1.0f / 2.0f", "0.5");
    run_ok("1d / 3d", "0.3333333333333333333333333333");
}

//#[test]
fn test_call_funs() {
    run_ok("print(1)", "pass");
    run_ok("println(1)", "pass");
}

#[test]
fn test_ifs() {
    run_ok("if true do 1 else 2 end", "1");
    run_ok("if false do 1 else 2 end", "2");
    run_ok("let a:=true\nif a do 1 else 2 end", "1");
    run_ok("let a:=true\n let b:= if a do 1 else 2 end\nb", "1");
}

#[test]
fn test_whiles() {
    run_ok("let a:=true\nwhile a do a:=false\n end", "pass");
}
