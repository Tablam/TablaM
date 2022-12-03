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
use std::rc::Rc;

use crate::lexer::{Lexer, Scanner};
use crate::token::{Syntax, SyntaxKind, Token, TokenId};

#[derive(Debug, Clone)]
pub(crate) enum S {
    Err(TokenId),
    Keyword(Syntax, TokenId),
    Atom(TokenId),
    Cons(TokenId, Vec<S>),
    Block(Vec<S>),
    Eof(TokenId),
}

pub(crate) struct Pratt<'a> {
    pub(crate) ast: S,
    pub(crate) code: &'a str,
    pub(crate) tokens: Rc<Scanner>,
}

impl fmt::Display for Pratt<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.ast {
            S::Atom(t) => {
                let t = self.tokens.get(*t);
                write!(f, "{}: {}", &self.code[t.range], t.kind)
            }
            S::Keyword(_s, t) => {
                let t = self.tokens.get(*t);
                write!(f, "{}: {}", &self.code[t.range], t.kind)
            }
            S::Block(rest) => {
                let last = rest.len() - 1;
                for (pos, s) in rest.iter().enumerate() {
                    let p = Pratt {
                        ast: s.clone(),
                        code: self.code,
                        tokens: self.tokens.clone(),
                    };
                    if pos < last {
                        write!(f, "{}\n", p)?
                    } else {
                        write!(f, "{}", p)?
                    }
                }
                Ok(())
            }
            S::Cons(head, rest) => {
                let head = self.tokens.get(*head);
                write!(f, "({}", head.kind)?;
                for s in rest {
                    let p = Pratt {
                        ast: s.clone(),
                        code: self.code,
                        tokens: self.tokens.clone(),
                    };
                    write!(f, " {}", p)?
                }
                write!(f, ")")
            }
            S::Err(t) => {
                let t = self.tokens.get(*t);
                write!(f, "ERR({})", &self.code[t.range])
            }
            S::Eof(t) => {
                let t = self.tokens.get(*t);
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

fn expr_lhs(lexer: &mut Scanner, t: Token) -> S {
    match t.kind {
        Syntax::LParen => {
            let lhs = expr_bp(lexer, 0);
            assert_eq!(lexer.next().kind, Syntax::RParen);
            lhs
        }
        Syntax::Plus => {
            if let Some(((), r_bp)) = prefix_binding_power(t.kind) {
                let rhs = expr_bp(lexer, r_bp);
                S::Cons(t.id, vec![rhs])
            } else {
                S::Err(t.id)
            }
        }
        Syntax::IfKw | Syntax::DoKw | Syntax::ElseKw | Syntax::EndKw => {
            // let rhs = expr_bp(lexer, 0);
            // S::Cons(t, vec![rhs])
            S::Keyword(t.kind, t.id)
        }
        s => match s.is() {
            SyntaxKind::Atom => S::Atom(t.id),
            SyntaxKind::Eof => S::Eof(t.id),
            SyntaxKind::Line => {
                let t = loop {
                    let t = lexer.next();
                    if t.kind != Syntax::Cr {
                        break t;
                    }
                };
                expr_lhs(lexer, t)
            }
            _ => S::Err(t.id),
        },
    }
}

fn expr_bp(lexer: &mut Scanner, min_bp: u8) -> S {
    let t = lexer.next();

    let mut lhs = expr_lhs(lexer, t);

    loop {
        let next = lexer.peek();

        if [SyntaxKind::Eof, SyntaxKind::Close].contains(&next.kind.is()) {
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
                S::Cons(next.id, vec![lhs, rhs])
            } else {
                S::Cons(next.id, vec![lhs])
            };
            continue;
        }

        if let Some((l_bp, r_bp)) = infix_binding_power(op) {
            if l_bp < min_bp {
                break;
            }
            lexer.next();

            let rhs = expr_bp(lexer, r_bp);
            lhs = S::Cons(next.id, vec![lhs, rhs]);

            continue;
        }
        lexer.next();
        if t.kind == Syntax::Cr || op == Syntax::Cr {
            let rhs = expr_lhs(lexer, next);
            lhs = S::Block(vec![lhs, rhs]);
        } else {
            lhs = expr_lhs(lexer, next);
            let rhs = expr_bp(lexer, 0);

            lhs = S::Cons(t.id, vec![lhs, rhs]);
        }
    }

    lhs
}

pub(crate) fn expr(code: &str) -> Pratt<'_> {
    let lexer = Lexer::new(0.into(), code);
    let mut scanner = Scanner::from(lexer);
    let ast = expr_bp(&mut scanner, 0);
    Pratt {
        ast,
        code,
        tokens: Rc::new(scanner),
    }
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
        assert_eq!(s.to_string(), "1: Integer");

        let s = expr("1.45");
        assert_eq!(s.to_string(), "1.45: Decimal");

        let s = expr("'hello'");
        assert_eq!(s.to_string(), "'hello': String");

        let s = expr("(((0)))");
        assert_eq!(s.to_string(), "0: Integer");

        let s = expr("d'2000-01-01'");
        assert_eq!(s.to_string(), "d'2000-01-01': Date");

        let s = expr("t'09:10:11'");
        assert_eq!(s.to_string(), "t'09:10:11': Time");

        let s = expr("dt'2000-01-01 09:10:11'");
        assert_eq!(s.to_string(), "dt'2000-01-01 09:10:11': DateTime");
    }

    #[test]
    fn ifs() {
        let s = expr("if true do false else true end");
        assert_eq!(
            s.to_string(),
            "(if true: Bool (do false: Bool (else true: Bool end: end)))"
        );

        let s = expr("if true do false\n1 else true\n2 end");
        assert_eq!(
            s.to_string(),
            "(if true: Bool (do false: Bool 1: Integer\nelse: else\ntrue: Bool\n2: Integer\nend: end))"
        );
    }

    #[test]
    fn ops() {
        let s = expr("1 + 2 * 3");
        assert_eq!(s.to_string(), "(+ 1: Integer (* 2: Integer 3: Integer))");
    }

    #[test]
    fn lit() {
        let s = expr("1\ntrue");
        assert_eq!(s.to_string(), "1: Integer\ntrue: Bool");
    }
}
