use crate::code::Code;
use crate::env::Env;
use crate::errors::ErrorCode;
use corelib::tree_flat::prelude::Tree;
use parser::ast::Ast;
use parser::files::{File, FilesDb};
use parser::parser::{Parsed, Parser};
use std::io::Read;
use std::path::PathBuf;
use std::{fs, io};

#[derive(Debug)]
pub struct Program {
    code: Tree<Code>,
    env: Env,
    pub files: FilesDb,
}

impl Program {
    pub fn new() -> Self {
        Program {
            code: Tree::new(Code::Eof),
            env: Env::new(),
            files: FilesDb::from_src(""),
        }
    }

    pub fn from_file(file: File) -> Self {
        Program {
            code: Tree::new(Code::Eof),
            env: Env::new(),
            files: FilesDb::new(file),
        }
    }

    pub fn from_src(source: &str) -> Self {
        let mut p = Program {
            code: Tree::new(Code::Eof),
            env: Env::new(),
            files: FilesDb::from_src(source),
        };

        match p.compile_from_src(source) {
            Ok(()) => p,
            Err(error) => {
                let span = *error.span();
                p.code = Tree::new(Code::Halt { error, span });
                p
            }
        }
    }

    pub fn compile(&mut self, parsed: &Parsed) -> Result<(), ErrorCode> {
        // Only compile valid code!
        if let Some(err) = parsed.errors() {
            return Err(err.into());
        }

        let mut code = Tree::with_capacity(Code::Root, parsed.ast.len());
        let mut parent = code.root().id;
        // Moving forward this MUST be correct code!
        for node in parsed.ast.iter() {
            let c = match node.data {
                Ast::Root(_) => continue,
                Ast::Scalar { val, span } => Code::Scalar {
                    val: val.clone(),
                    span: *span,
                },
                Ast::IfBlock {
                    if_span,
                    do_span,
                    else_span,
                    end_span,
                    check,
                    if_true,
                    if_false,
                } => {
                    todo!()
                }
                Ast::Cmp { op, span } => {
                    todo!()
                }
                Ast::Pass(_) => continue,
                Ast::Eof(_) => Code::Eof,
            };

            let mut node = code.tree_node_mut(parent).expect("Invalid AST id");
            parent = node.append(c);
        }

        self.code = code;
        Ok(())
    }

    pub fn compile_from_src(&mut self, source: &str) -> Result<(), ErrorCode> {
        let parse = Parser::from_src(source);
        let result = parse.parse();
        self.compile(&result)
    }

    pub fn append_from_src(&mut self, source: &str) -> Result<(), ErrorCode> {
        let root = self.files.get_root_mut();
        root.append(source);
        root.append("\n");

        let parse = Parser::from_src(source);
        let result = parse.parse();
        self.compile(&result)
    }

    pub fn eval(&self) -> Code {
        let mut result = Code::Eof;
        for node in self.code.into_iter() {
            result = match node.data {
                Code::Root => Code::Root,
                Code::Scalar { .. } => node.data.clone(),
                Code::If { code, .. } => {
                    todo!()
                }
                Code::Halt { error, .. } => {
                    result = node.data.clone();
                    break;
                }
                Code::Eof => Code::Eof,
            };
        }

        result
    }
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}

pub fn read_file_to_string(f: &mut fs::File) -> Result<String, io::Error> {
    let mut x = String::new();
    f.read_to_string(&mut x)?;
    Ok(x)
}

pub fn create_file(path: PathBuf, source: &str) -> File {
    File::from_path(path, source)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::code::CodePrinter;
    use crate::diagnostic::{print_diagnostic, print_diagnostic_to_str};
    use expect_test::expect;

    pub(crate) fn check(source: &str, expected_tree: expect_test::Expect) {
        let parse = Program::from_src(source);

        let result = parse.eval();

        let printer = CodePrinter {
            parsed: &Tree::new(result),
        };
        println!("{}", &printer);
        expected_tree.assert_eq(&printer.to_string());
    }

    pub(crate) fn check_err(source: &str, expected_tree: expect_test::Expect) {
        let parse = Program::from_src(source);

        let result = parse.eval();

        let err = match result {
            Code::Halt { error, .. } => print_diagnostic_to_str(&parse.files, &error).unwrap(),
            x => panic!("Expected err, got: {x:?}"),
        };

        expected_tree.assert_eq(&err);
    }

    #[test]
    fn eval_nothing() {
        check(
            "",
            expect![[r##"
Root
"##]],
        );
    }

    #[test]
    fn eval_scalar() {
        check(
            "1",
            expect![[r##"
1
"##]],
        );

        check(
            "true",
            expect![[r##"
true
"##]],
        );

        check(
            "123_456",
            expect![[r##"
123456
"##]],
        );

        check(
            "1.1",
            expect![[r##"
1.1d
"##]],
        );

        check(
            "1.1f",
            expect![[r##"
1.1f
"##]],
        );

        check(
            "'hello\nworld'",
            expect![[r##"
"hello
world"
"##]],
        );
    }

    #[test]
    fn eval_bits() {
        check(
            "1b",
            expect![[r##"
Bits[1]
"##]],
        );

        check(
            "0b",
            expect![[r##"
Bits[0]
"##]],
        );

        check(
            "01_001b",
            expect![[r##"
Bits[0, 1, 0, 0, 1]
"##]],
        );
    }

    #[test]
    fn eval_diagnostic() {
        check_err(
            "1\n2b",
            expect![[r##"
[01] Error: Invalid bit char `2` at pos 0. Must be 1 or 0.
   ╭─[repl:1:1]
   │
 2 │ 2b
   · ─┬  
   ·  ╰── Invalid bit char `2` at pos 0. Must be 1 or 0.
   · 
   · Note: Parsing value of type: Bit
───╯
"##]],
        );
    }
}
