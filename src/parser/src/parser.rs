use crate::ast::{Ast, ExprBool, Ty};
use crate::checklist::{CheckList, Task};
use crate::cst::{src_to_cst, Cst, CstNode};
use crate::files::FilesDb;
use crate::token::{token_eof, Syntax, Token, TokenId};
use crate::{errors, expr};

use crate::errors::ErrorParser;
use corelib::errors::Span;
use corelib::tree_flat::node::NodeId;
use corelib::tree_flat::prelude::{Node, Tree};
use std::fmt;

/// The points in the code where we can recover after a failed parse
const RECOVERY_SET: [Syntax; 2] = [Syntax::LetKw, Syntax::VarKw];

pub struct ParsedPrinter<'a> {
    parsed: &'a Parsed,
}

#[derive(Debug)]
pub struct Parsed {
    pub ast: Tree<Ast>,
    pub errors: Vec<ErrorParser>,
}

impl Parsed {
    pub fn errors(&self) -> Option<&[ErrorParser]> {
        if self.errors.is_empty() {
            None
        } else {
            Some(&self.errors)
        }
    }
}

pub(crate) struct Checker<'a> {
    pub(crate) check: CheckList,
    pub(crate) cst: Cst<'a>,
    pub(crate) ast: Tree<Ast>,
    pub(crate) cursor: usize,
    pub(crate) errors: Vec<ErrorParser>,
}

impl<'a> Checker<'a> {
    pub fn new(cst: Cst<'a>) -> Self {
        // Start at 1 to skip Root!
        let root = cst.ast.root();
        let span = root.data.span(&cst.tokens);
        Self {
            check: CheckList::new(Task::Start, span),
            ast: Tree::with_capacity(Ast::Root(span), cst.code.len()),
            cst,
            cursor: 1,
            errors: vec![],
        }
    }

    pub(crate) fn at_end(&self) -> bool {
        //NOTE: The last token is always EOF!
        self.cursor >= self.cst.ast.len()
    }

    pub(crate) fn new_task(&mut self, task: Task, t: TokenId) {
        let t = self.token(t);
        self.check = CheckList::new(task, t.into())
    }
    pub(crate) fn new_task_span(&mut self, task: Task, s: Span) {
        self.check = CheckList::new(task, s)
    }
    fn cst(&self) -> Option<Node<'_, CstNode>> {
        self.cst.ast.node(self.cursor.into())
    }
    fn cst_peek(&self) -> Option<Node<'_, CstNode>> {
        self.cst.ast.node((self.cursor + 1).into())
    }

    pub(crate) fn push(&mut self, ast: Ast, parent: NodeId) -> NodeId {
        let mut node = self.ast.tree_node_mut(parent).expect("Invalid AST id");
        node.append(ast)
    }

    pub(crate) fn code(&self, t: &Token) -> &str {
        &self.cst.code[t.range]
    }

    pub(crate) fn token(&self, id: TokenId) -> &Token {
        self.cst.tokens.get(id)
    }

    pub(crate) fn next(&mut self) -> CstNode {
        self.cst()
            .map(|x| *x.data)
            .unwrap_or_else(|| CstNode::Eof(token_eof().id))
    }

    pub(crate) fn peek(&mut self) -> CstNode {
        self.cst_peek()
            .map(|x| *x.data)
            .unwrap_or_else(|| CstNode::Eof(token_eof().id))
    }

    pub(crate) fn advance(&mut self) {
        self.cursor += 1;
    }

    pub(crate) fn advance_eof(&mut self) {
        self.cursor = self.cst.ast.len();
    }

    pub(crate) fn advance_and_next(&mut self) -> CstNode {
        self.advance();
        self.next()
    }

    pub(crate) fn push_or_err(
        &mut self,
        of: Result<Ast, ErrorParser>,
        parent: NodeId,
    ) -> Result<NodeId, NodeId> {
        match of {
            Ok(ast) => {
                self.advance();
                Ok(self.push(ast, parent))
            }
            Err(err) => {
                self.advance();
                self.errors.push(err);
                Err(parent)
            }
        }
    }

    pub(crate) fn recover(&mut self) {}

    pub(crate) fn check_pending(&mut self) {
        //It has a pending task unfinished?
        if !self.check.is_done() {
            dbg!(&self.check);
            let current = self.next();
            let err = errors::incomplete(&self.check, current);
            self.errors.push(err);
            self.recover();
        }
    }
}

pub struct Parser {
    pub(crate) files: FilesDb,
}

impl Parser {
    pub fn new(files: FilesDb) -> Self {
        Self { files }
    }

    pub fn from_src(source: &str) -> Self {
        let files = FilesDb::from_src(source);
        Self::new(files)
    }

    pub fn parse(&self) -> Parsed {
        let root = self.files.get_root();
        let cst = src_to_cst(root.source());

        let mut check = Checker::new(cst);
        expr::root(&mut check);

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

pub(crate) fn fmt_t<T: fmt::Debug>(
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

fn fmt_bool_expr(
    node: &ExprBool,
    kind: Ty,
    level: usize,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    match node {
        ExprBool::Scalar { val, span } => fmt_t(f, level, kind, val, span),
    }
}

fn fmt_node(node: &Ast, level: usize, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let kind = node.ty();

    match node {
        Ast::Root(_) => write!(f, "Root")?,
        Ast::Scalar { val, span } => fmt_t(f, level, kind, val, span)?,
        Ast::Pass(span) => fmt_plain(f, level, &"Pass", span)?,
        Ast::Eof(_) => write!(f, "Eof")?,
        Ast::Cmp { op, span } => fmt_plain(f, level, &format!("{:?}", op), span)?,
        Ast::IfBlock {
            if_span,
            do_span,
            else_span,
            end_span,
            check,
            if_true,
            if_false,
        } => {
            fmt_plain(f, level, &"if", if_span)?;
            writeln!(f)?;
            fmt_bool_expr(check, kind, level + 1, f)?;
            writeln!(f)?;
            fmt_plain(f, level, &"do", do_span)?;
            writeln!(f)?;
            fmt_node(if_true, level + 1, f)?;
            writeln!(f)?;
            fmt_plain(f, level, &"else", else_span)?;
            writeln!(f)?;
            fmt_node(if_false, level + 1, f)?;
            writeln!(f)?;
            fmt_plain(f, level, &"end --if", end_span)?;
        }
    };
    Ok(())
}

impl fmt::Display for ParsedPrinter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for node in self.parsed.ast.iter() {
            fmt_node(node.data, node.level(), f)?;
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

    let printer = ParsedPrinter { parsed: &result };
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
    fn parse_num() {
        check(
            " 123",
            expect![[r##"
Root
  T: I64 @@ 1..4: I64([123])
"##]],
        );
    }

    #[test]
    fn parse_str() {
        check(
            "'hello\nworld'",
            expect![[r#"
                Root
                  T: Utf8 @@ 0..13: Utf8(["\"hello\nworld\""])
            "#]],
        );

        check(
            "\"hello\nworld\"",
            expect![[r#"
                Root
                  T: Utf8 @@ 0..13: Utf8(["\"hello\nworld\""])
            "#]],
        );
    }

    #[test]
    fn parse_date() {
        check(
            "d'2000-01-01'",
            expect![[r#"
                Root
                  T: Date(Date) @@ 0..13: Date([Date(2000-01-01)])
            "#]],
        );
        check(
            "d\"2000-01-01\"",
            expect![[r#"
                Root
                  T: Date(Date) @@ 0..13: Date([Date(2000-01-01)])
            "#]],
        );
        check(
            "t'22:10:57'",
            expect![[r#"
                Root
                  T: Date(Time) @@ 0..11: Date([Time(22:10:57)])
            "#]],
        );
        check(
            "t\"22:10:57\"",
            expect![[r#"
                Root
                  T: Date(Time) @@ 0..11: Date([Time(22:10:57)])
            "#]],
        );

        check(
            "dt'2000-01-01 22:10:57 +0900'",
            expect![[r#"
                Root
                  T: Date(DateTime) @@ 0..29: Date([DateTime(2000-01-01 22:10:57 +0900)])
            "#]],
        );

        check(
            "dt\"2000-01-01 22:10:57 +0900\"",
            expect![[r#"
                Root
                  T: Date(DateTime) @@ 0..29: Date([DateTime(2000-01-01 22:10:57 +0900)])
            "#]],
        );
    }

    #[test]
    fn parse_if() {
        check(
            "if true do 1 else 2 end",
            expect![[r##"
Root
  0..2: "if"
    @@ 3..7: Bool([true])
  8..10: "do"
   T: I64 @@ 11..12: I64([1])
  13..17: "else"
   T: I64 @@ 18..19: I64([2])
  20..23: "end --if"
"##]],
        );
    }

    #[test]
    fn parse_fragment_lit() {
        check(
            "1\ntrue",
            expect![[r##"
Root
  T: I64 @@ 0..1: I64([1])
  T: Bool @@ 2..6: Bool([true])
"##]],
        );
    }
}
