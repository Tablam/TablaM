//! # First parsing pass:
//!
//! Reorder the code with the proper precedence of operators using the pratt parsing technique.
//!
//! ### Note ###
//!
//! This step not validate the parsing is correct,
//! only prepare the code to be linearized to the next pass
//!
use std::fmt;
use text_size::TextRange;

use crate::lexer::Lexer;
use crate::token::{token_test, Syntax, Token};

#[derive(Debug, Clone)]
enum S {
    Err(Token),
    Trivia(Token),
    Atom(Token),
    Cons(Syntax, Vec<S>),
}

struct Printer<'a> {
    ast: S,
    code: &'a str,
}

fn get_src<'a>(of: &'a str, t: &'a Token) -> &'a str {
    &of[t.range]
}

impl fmt::Display for Printer<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.ast {
            S::Trivia(t) => write!(f, "{}: {}", &self.code[t.range], t.kind),
            S::Atom(t) => write!(f, "{}: {}", &self.code[t.range], t.kind),
            S::Cons(head, rest) => {
                write!(f, "({}", head)?;
                for s in rest {
                    let p = Printer {
                        ast: s.clone(),
                        code: self.code,
                    };
                    write!(f, " {}", p)?
                }
                write!(f, ")")
            }
            S::Err(t) => write!(f, "ERR({})", &self.code[t.range]),
        }
    }
}

fn prefix_binding_power(op: Syntax) -> Option<((), u8)> {
    match op {
        Syntax::Plus => Some(((), 9)),
        _ => None,
    }
}

fn postfix_binding_power(op: Syntax) -> Option<(u8, ())> {
    match op {
        Syntax::LSquare => Some((11, ())),
        _ => None,
    }
}

fn expr_bp(lexer: &mut Lexer, min_bp: u8) -> S {
    let t = if let Some(t) = lexer.next() {
        t
    } else {
        return S::Atom(token_test());
    };

    let mut lhs = match t.kind {
        Syntax::Cr | Syntax::Whitespace | Syntax::Comment => S::Trivia(t),
        Syntax::Bool | Syntax::Int64 => S::Atom(t),
        Syntax::Plus => {
            if let Some(((), r_bp)) = prefix_binding_power(t.kind) {
                let rhs = expr_bp(lexer, r_bp);
                print!("{} ", t.kind);
                S::Cons(t.kind, vec![rhs])
            } else {
                S::Err(t)
            }
        }
        _ => S::Err(t),
    };

    lhs
}

fn expr(code: &str) -> Printer<'_> {
    let mut lexer = Lexer::new(0.into(), code);
    let ast = expr_bp(&mut lexer, 0);
    Printer { ast, code }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser() {
        let s = expr("1");
        assert_eq!(s.to_string(), "1: Int64");
    }
}
