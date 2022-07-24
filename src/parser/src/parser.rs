use crate::ast::{Ast, Ty};
use crate::checklist::{CheckList, Step, Task};
use crate::cst::{src_to_cst, Cst, CstNode};
use crate::errors;
use crate::files::Files;
use crate::token::{Syntax, Token};

use crate::errors::ErrorParser;
use corelib::errors::Span;
use corelib::tree_flat::prelude::{Node, Tree};
use std::fmt;

/// The points in the code where we can recover after a failed parse
const RECOVERY_SET: [Syntax; 2] = [Syntax::LetKw, Syntax::VarKw];

pub struct ParsedPrinter<'a> {
    parsed: &'a Parsed,
    src: &'a str,
}

#[derive(Debug)]
pub struct Parsed {
    ast: Tree<Ast>,
    errors: Vec<ErrorParser>,
}

struct Checker<'a> {
    check: CheckList,
    cst: Cst<'a>,
    ast: Tree<Ast>,
    cursor: usize,
    errors: Vec<ErrorParser>,
}

impl<'a> Checker<'a> {
    pub fn new(cst: Cst<'a>) -> Self {
        // Start at 1 to skip Root!
        let root = cst.ast.root();
        Self {
            check: CheckList::new(Task::Start, root.data.span()),
            ast: Tree::with_capacity(Ast::Root, cst.code.len()),
            cst,
            cursor: 1,
            errors: vec![],
        }
    }

    fn at_end(&self) -> bool {
        //NOTE: The last token is always EOF!
        !(self.cursor < self.ast.len())
    }

    fn new_task(&mut self, task: Task, t: &Token) {
        self.check = CheckList::new(task, t.into())
    }
    fn new_task_span(&mut self, task: Task, s: Span) {
        self.check = CheckList::new(task, s)
    }
    fn cst(&self) -> Option<Node<'_, CstNode>> {
        self.cst.ast.node(self.cursor.into())
    }

    fn push(&mut self, ast: Ast, pos: usize) {
        let mut node = self.ast.node_mut((pos - 1).into()).expect("Invalid AST id");
        node.push(ast);
    }

    fn next(&mut self) -> CstNode {
        self.cst().map(|x| x.data.clone()).unwrap_or(CstNode::Eof)
    }

    fn advance(&mut self) {
        self.cursor += 1;
    }

    fn parse_scalar(&mut self, t: &Token) -> Result<Ast, ErrorParser> {
        let txt = &self.cst.code[t.range];

        match t.kind {
            Syntax::Bool => {
                self.check.check(Step::Bool, t.into());
                match txt.parse::<bool>() {
                    Ok(x) => Ok(Ast::scalar(x.into(), t)),
                    Err(x) => Err(errors::parse(t, &x.to_string())),
                }
            }
            Syntax::Int64 => {
                self.check.check(Step::I64, t.into());
                match txt.parse::<i64>() {
                    Ok(x) => Ok(Ast::scalar(x.into(), t)),
                    Err(x) => Err(errors::parse(t, &x.to_string())),
                }
            }
            _ => unimplemented!(),
        }
    }

    fn push_or_err(&mut self, of: Result<Ast, ErrorParser>) {
        match of {
            Ok(ast) => {
                self.push(ast, self.cursor);
            }
            Err(err) => self.errors.push(err),
        }
        self.advance()
    }

    fn recover(&mut self) {}

    fn check_pending(&mut self) {
        //It has a pending task unfinished?
        if !self.check.is_done() {
            dbg!(&self.check);
            let err = errors::incomplete(&self.check);
            self.errors.push(err);
            self.recover();
        }
    }

    /// The main interface that run the parser with a [CheckList]
    /// and report the errors
    fn verify(&mut self) {
        let next = self.next();
        dbg!("Checking", &next);
        if next == CstNode::Eof {
            return;
        }
        if let CstNode::Err(err) = next {
            self.recover();
        }

        match &self.check.task {
            Task::Start => {
                if let CstNode::Atom(t) = next {
                    self.new_task(Task::Expr, &t);
                    self.verify();
                }
                if let CstNode::Op(t) = next {
                    self.new_task(Task::Expr, &t);
                    self.verify();
                }
            }
            Task::Expr => {
                if let CstNode::Atom(t) = &next {
                    let of = self.parse_scalar(t);
                    self.push_or_err(of)
                }
            }
            x => unimplemented!("{:?}", x),
        }
    }
}

pub struct Parser {
    pub(crate) files: Files,
}

impl Parser {
    pub fn new(files: Files) -> Self {
        Self { files }
    }

    pub fn from_src(source: &str) -> Self {
        let files = Files::from_src(source);
        Self::new(files)
    }

    pub fn parse(&self) -> Parsed {
        let root = self.files.get_root();
        let cst = src_to_cst(root.data.source());

        let mut check = Checker::new(cst);

        loop {
            check.verify();

            if check.at_end() {
                break;
            } else {
                //It has a pending task unfinished?
                check.check_pending();
                if !check.at_end() {
                    let next = check.next();
                    check.new_task_span(Task::Start, next.span());
                }
            }
        }

        //It has a pending task unfinished?
        check.check_pending();

        Parsed {
            ast: check.ast,
            errors: check.errors,
        }
    }
}

fn fmt_plain<T: fmt::Debug>(
    f: &mut fmt::Formatter<'_>,
    level: usize,
    val: &T,
    span: &Span,
) -> fmt::Result {
    write!(f, "{}{}: {:?}", " ".repeat(level + 1), span.range, val)
}

fn fmt_t<T: fmt::Debug>(
    f: &mut fmt::Formatter<'_>,
    level: usize,
    kind: Ty,
    val: &T,
    span: &Span,
) -> fmt::Result {
    write!(
        f,
        "{}{} @@ {}: {:?}",
        " ".repeat(level + 1),
        kind,
        span.range,
        val
    )
}

impl fmt::Display for ParsedPrinter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for node in self.parsed.ast.iter() {
            let level = node.level();
            let kind = node.data.ty();

            match node.data {
                Ast::Root => write!(f, "Root")?,
                Ast::Scalar { val, span } => fmt_t(f, level, kind, val, span)?,
                Ast::Pass(span) => fmt_plain(f, level, &"Pass", span)?,
                Ast::If(span) => fmt_plain(f, level, &"if", span)?,
                Ast::Eof => write!(f, "Eof")?,
            };

            writeln!(f)?;
        }

        if !self.parsed.errors.is_empty() {
            writeln!(f, "Errors")?;
            for err in &self.parsed.errors {
                writeln!(f, " {:?}", err)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
pub(crate) fn check(source: &str, expected_tree: expect_test::Expect) {
    let parse = Parser::from_src(source);
    let result = parse.parse();

    let printer = ParsedPrinter {
        parsed: &result,
        src: source,
    };
    println!("{}", &printer);
    expected_tree.assert_eq(&printer.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::expect;

    #[test]
    fn parse_nothing() {
        check(
            "",
            expect![[r##"
Root
"##]],
        );
    }

    #[test]
    fn parse_int() {
        check(
            " 123",
            expect![[r##"
Root
  T: I64 @@ 1..4: I64([123])
"##]],
        );
    }
}
