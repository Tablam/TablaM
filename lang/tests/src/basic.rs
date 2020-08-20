use logos::source::Source;
use logos::Logos;
use std::fmt;
use std::ops::Range;
use tablam::decorum::R64;
use tablam::rust_decimal::prelude::*;
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
                    line: 1,
                    range_column: 0..3,
                    line_range_column: 0..3,
                }),
                "let",
                0..3,
            ),
            (
                Token::Variable(TokenData {
                    value: Some("int".to_string()),
                    line: 1,
                    range_column: 4..7,
                    line_range_column: 4..7,
                }),
                "int",
                4..7,
            ),
            (
                Token::Assignment(TokenData {
                    value: None,
                    line: 1,
                    range_column: 8..10,
                    line_range_column: 8..10,
                }),
                ":=",
                8..10,
            ),
            (
                Token::Integer(TokenData {
                    value: Some(1i64),
                    line: 1,
                    range_column: 11..12,
                    line_range_column: 11..12,
                }),
                "1",
                11..12,
            ),
        ],
    );
}

#[test]
fn test_syntax_v1() {
    assert_lex(
        "let int := 1 \nlet float := 1.1f \nlet money := 10d \nvar payment := 150.5d \npayment += money",
        &[
            (
                Token::Let(TokenData {
                    value: None,
                    line: 1,
                    range_column: 0..3,
                    line_range_column: 0..3,
                }),
                "let",
                0..3,
            ),
            (
                Token::Variable(TokenData {
                    value: Some("int".to_string()),
                    line: 1,
                    range_column: 4..7,
                    line_range_column: 4..7,
                }),
                "int",
                4..7,
            ),
            (
                Token::Assignment(TokenData {
                    value: None,
                    line: 1,
                    range_column: 8..10,
                    line_range_column: 8..10,
                }),
                ":=",
                8..10,
            ),
            (
                Token::Integer(TokenData {
                    value: Some(1i64),
                    line: 1,
                    range_column: 11..12,
                    line_range_column: 11..12,
                }),
                "1",
                11..12,
            ),
            (
                Token::Let(TokenData {
                    value: None,
                    line: 2,
                    range_column: 14..17,
                    line_range_column: 0..3,
                }),
                "let",
                14..17,
            ),
            (
                Token::Variable(TokenData {
                    value: Some("float".to_string()),
                    line: 2,
                    range_column: 18..23,
                     line_range_column: 4..9,
                }),
                "float",
                18..23,
            ),
            (
                Token::Assignment(TokenData {
                    value: None,
                    line: 2,
                    range_column: 24..26,
                    line_range_column: 10..12,
                }),
                ":=",
                24..26,
            ),
            (
                Token::Float(TokenData {
                    value: Some(1.1.into()),
                    line: 2,
                    range_column: 27..31,
                    line_range_column: 13..17,
                }),
                "1.1f",
                27..31,
            ),
            (
                Token::Let(
                    TokenData {
                        value: None,
                        line: 3,
                        range_column: 33..36,
                        line_range_column: 0..3,
                    }),
                "let",
                33..36
            ),
            (
            Token::Variable(
                TokenData {
                    value: Some(
                        "money".to_string(),
                    ),
                    line: 3,
                    range_column: 37..42,
                    line_range_column: 4..9,
                }),
            "money",
            37..42
            ),
            (Token::Assignment(
                TokenData {
                    value: None,
                    line: 3,
                    range_column: 43..45,
                    line_range_column: 10..12,
                },
            ),
             ":=",
             43..45),
            (Token::Decimal(
                TokenData {
                    value: Some(
                        Decimal::from_str("10").unwrap(),
                    ),
                    line: 3,
                    range_column: 46..49,
                    line_range_column: 13..16,
                },
            ),
             "10d",
             46..49),
            (Token::Var(
                TokenData {
                    value: None,
                    line: 4,
                    range_column: 51..54,
                    line_range_column: 0..3,
                },
            ),
             "var",
             51..54),
            (Token::Variable(
                TokenData {
                    value: Some(
                        "payment".to_string(),
                    ),
                    line: 4,
                    range_column: 55..62,
                     line_range_column: 4..11,
                },
            ),
             "payment",
             55..62),
            (Token::Assignment(
                TokenData {
                    value: None,
                    line: 4,
                    range_column: 63..65,
                    line_range_column: 12..14,
                },
            ),
             ":=",
             63..65),
                (Token::Decimal(
                    TokenData {
                        value: Some(
                            Decimal::from_str("150.5").unwrap(),
                        ),
                        line: 4,
                        range_column: 66..72,
                        line_range_column: 15..21,
                    },
                ),
                 "150.5d",
                 66..72),
            (Token::Variable(
                TokenData {
                    value: Some(
                        "payment".to_string(),
                    ),
                    line: 5,
                    range_column:  74..81,
                    line_range_column: 0..7,
                },
            ),
             "payment",
             74..81),
            (Token::PlusEqual(
                TokenData {
                    value: None,
                    line: 5,
                    range_column:  82..84,
                    line_range_column: 8..10,
                },
            ),
             "+=",
             82..84),
            (Token::Variable(
                TokenData {
                    value: Some(
                        "money".to_string(),
                    ),
                    line: 5,
                    range_column:  85..90,
                    line_range_column: 11..16,
                },
            ),
             "money",
             85..90)
        ],
    );
}

#[test]
fn test_scanner() {
    let mut scanner = Scanner::new("let true_value := true \nlet false_value := false \n\nvar result := true_value = false_value \nresult := result <> true \nresult := not result = false \nlet flag := result or false_value \nlet complex_flag := (result and true_value) or flag");
    while let Some(token) = scanner.peek() {
        dbg!(token);
        scanner.accept();
    }
}
