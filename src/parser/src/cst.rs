//! The CST store a full-fidelity view of the code (even if wrong)
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Formatter;

use corelib::chrono::format::Parsed;
use tree_flat::prelude::{NodeMut, Tree};

use crate::pratt;
use crate::pratt::S;
use crate::pratt::{expr, Pratt};
use crate::token::{Syntax, Token};

#[derive(Debug, Clone)]
pub(crate) enum CstNode {
    Root,
    Trivia(Token),
    Atom(Token),
    Op(Token),
    Err(Token),
    Eof,
}

pub(crate) struct Cst<'a> {
    ast: Tree<CstNode>,
    code: &'a str,
}

fn fmt_t(f: &mut fmt::Formatter<'_>, level: usize, code: &str, t: &Token) -> fmt::Result {
    write!(
        f,
        "{}{} @ {:?} \"{}\"",
        " ".repeat(level + 1),
        t.kind,
        t.range,
        &code[t.range]
    )
}

fn fmt_op(f: &mut fmt::Formatter<'_>, level: usize, code: &str, t: &Token) -> fmt::Result {
    assert!(t.kind.is_op());
    let extra = if let Some(op) = t.kind.to_bin_op() {
        format!("BinOp {:?}", op)
    } else if let Some(op) = t.kind.to_unary_op() {
        format!("UnaryOp {:?}", op)
    } else {
        unreachable!()
    };

    write!(
        f,
        "{}{} @ {:?} \"{}\"",
        " ".repeat(level + 1),
        extra,
        t.range,
        &code[t.range]
    )
}

impl fmt::Display for Cst<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let last = self.ast.len() - 1;
        for node in self.ast.iter() {
            let level = node.level();

            match node.data {
                CstNode::Root => write!(f, "Root")?,
                CstNode::Trivia(t) => fmt_t(f, level, self.code, t)?,
                CstNode::Atom(t) => fmt_t(f, level, self.code, t)?,
                CstNode::Op(t) => fmt_op(f, level, self.code, t)?,
                CstNode::Err(t) => fmt_t(f, level, self.code, t)?,
                CstNode::Eof => write!(f, "EOF")?,
            };

            writeln!(f)?;
        }
        Ok(())
    }
}

fn push(tree: &mut NodeMut<CstNode>, t: CstNode) {
    tree.push(t);
}

fn to_cst(tree: &mut NodeMut<CstNode>, ast: S) {
    match ast {
        S::Trivia(t) => push(tree, CstNode::Trivia(t)),
        S::Atom(t) => push(tree, CstNode::Atom(t)),
        S::Cons(op, rest) => {
            let op = &mut tree.push(CstNode::Op(op));
            for s in rest {
                to_cst(op, s);
            }
        }
        S::Err(t) => push(tree, CstNode::Err(t)),
    };
}

pub(crate) fn parse(pratt: Pratt<'_>) -> Cst<'_> {
    let mut ast = Tree::with_capacity(CstNode::Root, 6);

    let mut root = ast.root_mut();

    to_cst(&mut root, pratt.ast);

    Cst {
        ast,
        code: pratt.code,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::expect;

    fn check(code: &str, expected_tree: expect_test::Expect) {
        let s = expr(code);
        println!("{}", s);
        let tree = parse(s);
        expected_tree.assert_eq(&tree.to_string());
    }

    #[test]
    fn parser() {
        let s = expr("1");
        assert_eq!(s.to_string(), "1: Int64");

        let s = expr("1.45");
        assert_eq!(s.to_string(), "1.45: Decimal");
    }

    #[test]
    fn linear() {
        check(
            "1 + 2 * 3",
            expect![[r##"
Root
  BinOp Add @ 2..3 "+"
   Int64 @ 0..1 "1"
   BinOp Mul @ 6..7 "*"
    Int64 @ 4..5 "2"
    Int64 @ 8..9 "3"
"##]],
        );
    }
}
