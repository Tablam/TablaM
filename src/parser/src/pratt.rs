//! # First parsing pass:
//!
//! Reorder the code with the proper precedence of operators using the pratt parsing technique.
//!
//! ### Note ###
//!
//! This step not validate the parsing is correct,
//! only prepare the code to be linearized to the next pass
//!
use crate::cst::CstNode;
use corelib::text_size::TextRange;
use corelib::tree_flat::prelude::{NodeId, NodeMut, Tree};
use std::fmt;
use std::iter::Peekable;

use crate::lexer::{Lexer, Scanner};
use crate::token::{token_test, Syntax, SyntaxKind, Token};

#[derive(Debug, Clone)]
pub(crate) enum S {
    Err(Token),
    Atom(Token),
    Cons(Token, Vec<S>),
    Eof(Token),
}

pub(crate) struct Pratt<'a> {
    pub(crate) ast: S,
    pub(crate) code: &'a str,
}

impl fmt::Display for Pratt<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.ast {
            S::Atom(t) => write!(f, "{}: {}", &self.code[t.range], t.kind),
            S::Cons(head, rest) => {
                write!(f, "({}", head.kind)?;
                for s in rest {
                    let p = Pratt {
                        ast: s.clone(),
                        code: self.code,
                    };
                    write!(f, " {}", p)?
                }
                write!(f, ")")
            }
            S::Err(t) => write!(f, "ERR({})", &self.code[t.range]),
            S::Eof(t) => {
                write!(f, "{}", t.kind)
            }
        }
    }
}

fn prefix_binding_power(op: Syntax) -> Option<((), u8)> {
    let res = match op {
        Syntax::Plus => ((), 9),
        _ => return None,
    };
    Some(res)
}

fn postfix_binding_power(op: Syntax) -> Option<(u8, ())> {
    let res = match op {
        Syntax::LSquare => (11, ()),
        _ => return None,
    };
    Some(res)
}

fn infix_binding_power(op: Syntax) -> Option<(u8, u8)> {
    let res = match op {
        Syntax::Equals => (2, 1),
        Syntax::Question => (4, 3),
        Syntax::Plus | Syntax::Minus => (5, 6),
        Syntax::Star | Syntax::Slash => (7, 8),
        Syntax::Point => (14, 13),
        _ => return None,
    };
    Some(res)
}

fn expr_bp(lexer: &mut Scanner, min_bp: u8) -> S {
    let t = lexer.next();

    let mut lhs = match t.kind {
        Syntax::Bool | Syntax::Int64 | Syntax::Decimal => S::Atom(t),
        Syntax::LParen => {
            let lhs = expr_bp(lexer, 0);
            assert_eq!(lexer.next().kind, Syntax::RParen);
            lhs
        }
        Syntax::Plus => {
            if let Some(((), r_bp)) = prefix_binding_power(t.kind) {
                let rhs = expr_bp(lexer, r_bp);
                S::Cons(t, vec![rhs])
            } else {
                S::Err(t)
            }
        }
        s => match s.is() {
            SyntaxKind::Eof => S::Eof(t),
            _ => S::Err(t),
        },
    };

    loop {
        let mut next = lexer.peek();

        if next.kind == Syntax::Eof {
            break;
        };

        let op = next.kind;

        if let Some((l_bp, ())) = postfix_binding_power(op) {
            if l_bp < min_bp {
                break;
            }
            lexer.next();

            lhs = if op.is() == SyntaxKind::Open {
                let rhs = expr_bp(lexer, 0);
                //assert_eq!(lexer.next(), Token::Op(']'));
                S::Cons(next, vec![lhs, rhs])
            } else {
                S::Cons(next, vec![lhs])
            };
            continue;
        }

        if let Some((l_bp, r_bp)) = infix_binding_power(op) {
            if l_bp < min_bp {
                break;
            }
            lexer.next();

            let rhs = expr_bp(lexer, r_bp);
            lhs = S::Cons(next, vec![lhs, rhs]);

            continue;
        }
        break;
    }

    lhs
}

pub(crate) fn expr(code: &str) -> Pratt<'_> {
    let lexer = Lexer::new(0.into(), code);
    let mut scanner = Scanner::from(lexer);
    let ast = expr_bp(&mut scanner, 0);
    Pratt { ast, code }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_nothing() {
        let s = expr("");
        assert_eq!(s.to_string(), "Eof");
    }

    #[test]
    fn parser() {
        let s = expr("1");
        assert_eq!(s.to_string(), "1: Int64");

        let s = expr("1.45");
        assert_eq!(s.to_string(), "1.45: Decimal");

        let s = expr("(((0)))");
        assert_eq!(s.to_string(), "0: Int64");
    }

    #[test]
    fn ops() {
        let s = expr("1 + 2 * 3");
        assert_eq!(s.to_string(), "(+ 1: Int64 (* 2: Int64 3: Int64))");
    }
}
