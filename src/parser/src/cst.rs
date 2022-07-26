//! The CST store a full-fidelity view of the code (even if wrong)
use corelib::errors::Span;
use std::fmt;

use corelib::tree_flat::prelude::{NodeMut, Tree};

use crate::pratt::S;
use crate::pratt::{expr, Pratt};
use crate::token::{token_eof, token_test, Syntax, Token};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum CstNode {
    Root,
    Atom(Token),
    Op(Token),
    Err(Token),
    If(Token),
    Else(Token),
    Do(Token),
    End(Token),
    Eof,
}

impl CstNode {
    pub(crate) fn span(&self) -> Span {
        match self {
            CstNode::Root => (&token_test()).into(),
            CstNode::Atom(x) => x.into(),
            CstNode::Op(x) => x.into(),
            CstNode::Err(x) => x.into(),
            CstNode::If(x) => x.into(),
            CstNode::Else(x) => x.into(),
            CstNode::Do(x) => x.into(),
            CstNode::End(x) => x.into(),
            CstNode::Eof => (&token_eof()).into(),
        }
    }
}

pub(crate) struct Cst<'a> {
    pub(crate) ast: Tree<CstNode>,
    pub(crate) code: &'a str,
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
        for node in self.ast.iter() {
            let level = node.level();

            match node.data {
                CstNode::Root => write!(f, "Root")?,
                CstNode::Atom(t) => fmt_t(f, level, self.code, t)?,
                CstNode::Op(t) => fmt_op(f, level, self.code, t)?,
                CstNode::Err(t) => fmt_t(f, level, self.code, t)?,
                CstNode::Eof => write!(f, "{}EOF", " ".repeat(level + 1))?,
                CstNode::If(t) => fmt_t(f, level, self.code, t)?,
                CstNode::Else(t) => fmt_t(f, level, self.code, t)?,
                CstNode::Do(t) => fmt_t(f, level, self.code, t)?,
                CstNode::End(t) => fmt_t(f, level, self.code, t)?,
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
        S::Atom(t) => push(tree, CstNode::Atom(t)),
        S::Cons(op, rest) => {
            let node = match op.kind {
                Syntax::IfKw => CstNode::If(op),
                Syntax::ElseKw => CstNode::Else(op),
                Syntax::DoKw => CstNode::Do(op),
                Syntax::EndKw => CstNode::End(op),
                _ => CstNode::Op(op),
            };

            let op = &mut tree.push(node);
            for s in rest {
                to_cst(op, s);
            }
        }
        S::Err(t) => push(tree, CstNode::Err(t)),
        S::Eof(_) => push(tree, CstNode::Eof),
    };
}

pub(crate) fn parse(pratt: Pratt<'_>) -> Cst<'_> {
    let mut ast = Tree::new(CstNode::Root);

    let mut root = ast.root_mut();

    to_cst(&mut root, pratt.ast);

    Cst {
        ast,
        code: pratt.code,
    }
}

pub(crate) fn src_to_cst(code: &str) -> Cst<'_> {
    let s = expr(code);
    println!("{}", s);
    parse(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::expect;

    fn check(code: &str, expected_tree: expect_test::Expect) {
        let tree = src_to_cst(code);
        expected_tree.assert_eq(&tree.to_string());
    }

    #[test]
    fn parser() {
        let s = expr("1");
        assert_eq!(s.to_string(), "1: Integer");

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
   Integer @ 0..1 "1"
   BinOp Mul @ 6..7 "*"
    Integer @ 4..5 "2"
    Integer @ 8..9 "3"
"##]],
        );
    }

    #[test]
    fn ifs() {
        let s = check(
            "if true do false else true end",
            expect![[r##"
Root
  if @ 0..2 "if"
   Bool @ 3..7 "true"
   do @ 8..10 "do"
    Bool @ 11..16 "false"
    else @ 17..21 "else"
     Bool @ 22..26 "true"
     end @ 27..30 "end"
"##]],
        );
    }
}
