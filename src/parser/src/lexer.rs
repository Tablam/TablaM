use std::convert::TryFrom;
use std::ops::Range as StdRange;

use crate::token::{token_eof, Syntax, SyntaxKind, Token, TokenId};
use corelib::prelude::*;
use corelib::text_size::{TextRange, TextSize};
use logos::Logos;

pub struct Lexer<'a> {
    file_id: FileId,
    lexer: logos::Lexer<'a, Syntax>,
    cursor: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(file_id: FileId, input: &'a str) -> Self {
        Self {
            file_id,
            lexer: Syntax::lexer(input),
            cursor: 0,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        // Skip trivia
        let kind = loop {
            let kind = self.lexer.next()?;
            if kind.is() == SyntaxKind::Trivia {
                continue;
            } else {
                break kind;
            }
        };
        let span = self.lexer.span();

        let extra = self.lexer.extras;
        let line = (extra.current_line as u32).into();
        let col = ((span.end - extra.current_initial_column) as u32).into();

        let range = {
            let StdRange { start, end } = span;
            let start = TextSize::try_from(start).unwrap();
            let end = TextSize::try_from(end).unwrap();

            TextRange::new(start, end)
        };

        let token = Token {
            file_id: self.file_id,
            id: TokenId(self.cursor),
            kind,
            range,
            line,
            col,
        };
        self.cursor += 1;
        Some(token)
    }
}

pub struct Scanner {
    file_id: FileId,
    tokens: Vec<Token>,
}

impl Scanner {
    pub fn from(lexer: Lexer<'_>) -> Self {
        let file_id = lexer.file_id;
        let mut tokens: Vec<_> = lexer.collect();
        tokens.reverse();

        Self { file_id, tokens }
    }

    pub(crate) fn next(&mut self) -> Token {
        self.tokens.pop().unwrap_or(token_eof())
    }
    pub(crate) fn peek(&mut self) -> Token {
        self.tokens.last().copied().unwrap_or(token_eof())
    }
    pub(crate) fn len(&self) -> usize {
        self.tokens.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(input: &str, kind: Syntax) {
        let mut lexer = Lexer::new(0.into(), input);

        let token = lexer.next().unwrap();
        dbg!(&token);
        assert_eq!(token.kind, kind);
    }

    #[test]
    fn lex_bool() {
        check("true", Syntax::Bool);
        check("false", Syntax::Bool);
    }

    #[test]
    fn lex_numbers() {
        check("123456", Syntax::Integer);
        check("123_456", Syntax::Integer);

    }

    #[test]
    fn lex_decimals() {
        check("123456.123456", Syntax::Decimal);
        check("1234.1234d", Syntax::Decimal);
    }

    #[test]
    fn lex_floats() {
        check("123456.123456f", Syntax::Float);
    }

    #[test]
    fn lex_kw() {
        check("let", Syntax::LetKw);
        check("var", Syntax::VarKw);
        check("fun", Syntax::FnKw);
    }

    #[test]
    fn lex_identifier() {
        check("abcd", Syntax::Ident);
        check("ab123cde456", Syntax::Ident);
        check("ABCdef", Syntax::Ident);
        check("x", Syntax::Ident);
    }

    #[test]
    fn lex_ops() {
        check("+", Syntax::Plus);
        check("-", Syntax::Minus);
        check("*", Syntax::Star);
        check("/", Syntax::Slash);
        check(":=", Syntax::Assign);
    }

    #[test]
    fn lex_group() {
        check("(", Syntax::LParen);
        check(")", Syntax::RParen);
        check("{", Syntax::LBrace);
        check("}", Syntax::RBrace);
    }
}
