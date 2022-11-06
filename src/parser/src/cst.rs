//! The CST store a full-fidelity view of the code (even if wrong)
use corelib::errors::Span;
use std::fmt;
use std::rc::Rc;

use crate::lexer::Scanner;
use corelib::tree_flat::prelude::{Tree, TreeMut};

use crate::pratt::S;
use crate::pratt::{expr, Pratt};
use crate::token::{Syntax, Token, TokenId};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum CstNode {
    Root(TokenId),
    Atom(TokenId),
    Op(TokenId),
    If(TokenId),
    Else(TokenId),
    Do(TokenId),
    End(TokenId),
    Err(TokenId),
    Eof(TokenId),
}

impl CstNode {
    pub(crate) fn token_id(&self) -> TokenId {
        let x = match self {
            CstNode::Root(x) => x,
            CstNode::Atom(x) => x,
            CstNode::Op(x) => x,
            CstNode::Err(x) => x,
            CstNode::If(x) => x,
            CstNode::Else(x) => x,
            CstNode::Do(x) => x,
            CstNode::End(x) => x,
            CstNode::Eof(x) => x,
        };
        *x
    }

    pub(crate) fn span(&self, tokens: &Scanner) -> Span {
        tokens.get(self.token_id()).into()
    }
}

pub(crate) struct Cst<'a> {
    pub(crate) ast: Tree<CstNode>,
    pub(crate) code: &'a str,
    pub(crate) tokens: Rc<Scanner>,
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
                CstNode::Root(_) => write!(f, "Root")?,
                CstNode::Atom(t) => {
                    let t = self.tokens.get(*t);
                    fmt_t(f, level, self.code, t)?
                }
                CstNode::Op(t) => {
                    let t = self.tokens.get(*t);
                    fmt_op(f, level, self.code, t)?
                }
                CstNode::Err(t) => {
                    let t = self.tokens.get(*t);
                    fmt_t(f, level, self.code, t)?
                }
                CstNode::Eof(_) => write!(f, "{}EOF", " ".repeat(level + 1))?,
                CstNode::If(t) => {
                    let t = self.tokens.get(*t);
                    fmt_t(f, level, self.code, t)?
                }
                CstNode::Else(t) => {
                    let t = self.tokens.get(*t);
                    fmt_t(f, level, self.code, t)?
                }
                CstNode::Do(t) => {
                    let t = self.tokens.get(*t);
                    fmt_t(f, level, self.code, t)?
                }
                CstNode::End(t) => {
                    let t = self.tokens.get(*t);
                    fmt_t(f, level, self.code, t)?
                }
            };

            writeln!(f)?;
        }
        Ok(())
    }
}

fn push(tree: &mut TreeMut<CstNode>, t: CstNode) {
    tree.push(t);
}

fn to_cst(tree: &mut TreeMut<CstNode>, tokens: &Scanner, ast: S) {
    match ast {
        S::Atom(t) => push(tree, CstNode::Atom(t)),
        S::Cons(op, rest) => {
            let op = tokens.get(op);
            let node = match op.kind {
                Syntax::IfKw => CstNode::If(op.id),
                Syntax::ElseKw => CstNode::Else(op.id),
                Syntax::DoKw => CstNode::Do(op.id),
                Syntax::EndKw => CstNode::End(op.id),
                _ => CstNode::Op(op.id),
            };

            let op = &mut tree.push(node);
            for s in rest {
                to_cst(op, tokens, s);
            }
        }
        S::Err(t) => push(tree, CstNode::Err(t)),
        S::Eof(t) => push(tree, CstNode::Eof(t)),
    };
}

pub(crate) fn parse(pratt: Pratt<'_>) -> Cst<'_> {
    let mut ast = Tree::new(CstNode::Root(pratt.tokens.root.id));

    let mut root = ast.tree_root_mut();
    let tokens = pratt.tokens.clone();

    to_cst(&mut root, &tokens, pratt.ast);

    Cst {
        ast,
        code: pratt.code,
        tokens,
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
        check(
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
