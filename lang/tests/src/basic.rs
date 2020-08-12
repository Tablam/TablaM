use logos::source::Source;
use logos::Logos;
use std::fmt;
use std::ops::Range;
use tablam_lang::scanner::{Scanner, Token, TokenData};

pub fn assert_lex<'a, Token>(
    source: &'a Token::Source,
    tokens: &[(Token, &'a <Token::Source as Source>::Slice, Range<usize>)],
) where
    Token: Logos<'a> + fmt::Debug + PartialEq,
{
    let mut lex = Token::lexer(source);

    for tuple in tokens {
        assert_eq!(
            &(lex.next().expect("Unexpected end"), lex.slice(), lex.span()),
            tuple
        );
    }

    assert_eq!(lex.next(), None);
}

#[test]
fn test_syntax_v0() {
    assert_lex(
        "let int := 1",
        &[
            (
                Token::Let(TokenData {
                    value: None,
                    line: 0,
                    range_column: 0..3,
                }),
                "let",
                0..3,
            ),
            (
                Token::Variable(TokenData {
                    value: Some("int".to_string()),
                    line: 0,
                    range_column: 4..7,
                }),
                "int",
                4..7,
            ),
            (
                Token::Assignment(TokenData {
                    value: None,
                    line: 0,
                    range_column: 8..10,
                }),
                ":=",
                8..10,
            ),
            (
                Token::Integer(TokenData {
                    value: Some(1i64),
                    line: 0,
                    range_column: 11..12,
                }),
                "1",
                11..12,
            ),
        ],
    );
}

#[test]
fn test_scanner() {
    let mut scanner = Scanner::new("let int := 1");
    while let Some(token) = scanner.peek() {
        dbg!(token);
        scanner.accept();
    }
}
