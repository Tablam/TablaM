use logos::source::Source;
use logos::Logos;
use std::fmt;
use std::ops::Range;

use tablam::rust_decimal::prelude::*;
use tablam_lang::lexer::{Scanner, Token, TokenData};
use tablam_lang::parser::Parser;

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

pub fn assert_lexer_data(source: &str, tokens: &[(Token, TokenData)]) {
    let mut lex = Scanner::new(source);

    for tuple in tokens {
        assert_eq!(lex.accept().as_ref().unwrap(), tuple);
    }

    assert_eq!(lex.accept(), None);
}

#[test]
fn test_syntax_v0() {
    assert_lex(
        "let int := 1",
        &[
            (Token::Let, "let", 0..3),
            (Token::Variable("int".to_string()), "int", 4..7),
            (Token::Assignment, ":=", 8..10),
            (Token::Integer(1i64), "1", 11..12),
        ],
    );
}

#[test]
fn test_syntax_v1() {
    assert_lex(
        "let int := 1 \nlet float := 1.1f \nlet money := 10d \nvar payment := 150.5d \npayment += money",
        &[
            (Token::Let, "let", 0..3),
            (Token::Variable("int".to_string()), "int", 4..7),
            (Token::Assignment, ":=", 8..10),
            (Token::Integer(1i64), "1", 11..12),
            (Token::Let, "let", 14..17, ),
            (Token::Variable("float".to_string()), "float", 18..23),
            (Token::Assignment, ":=", 24..26),
            (Token::Float(1.1.into()), "1.1f", 27..31),
            (Token::Let, "let", 33..36),
            (Token::Variable("money".to_string()), "money", 37..42),
            (Token::Assignment, ":=", 43..45),
            (Token::Decimal(Decimal::from_str("10").unwrap()), "10d", 46..49),
            (Token::Var, "var", 51..54),
            (Token::Variable("payment".to_string()), "payment", 55..62),
            (Token::Assignment, ":=", 63..65), 
            (Token::Decimal(Decimal::from_str("150.5").unwrap()), "150.5d", 66..72),
            (Token::Variable("payment".to_string()), "payment", 74..81),
            (Token::PlusEqual, "+=", 82..84),
            (Token::Variable("money".to_string()), "money", 85..90)
        ],
    );
}

#[test]
fn test_scanner() {
    assert_lexer_data(
        "let true_value := true",
        &[
            (
                Token::Let,
                TokenData {
                    line: 1,
                    range_column: 0..3,
                    line_range_column: 0..3,
                },
            ),
            (
                Token::Variable("true_value".to_string()),
                TokenData {
                    line: 1,
                    range_column: 4..14,
                    line_range_column: 4..14,
                },
            ),
            (
                Token::Assignment,
                TokenData {
                    line: 1,
                    range_column: 15..17,
                    line_range_column: 15..17,
                },
            ),
            (
                Token::True,
                TokenData {
                    line: 1,
                    range_column: 18..22,
                    line_range_column: 18..22,
                },
            ),
        ],
    )
}

#[test]
fn test_parser() {
    let input = "1";
    let mut parser = Parser::new(input);
    let result = parser.parse();
    assert_eq!(
        result.expect("not getting expression").to_string(),
        String::from("1")
    );

    let input = "1+2";
    let mut parser = Parser::new(input);
    let result = parser.parse();
    assert_eq!(
        result.expect("not getting expression").to_string(),
        String::from("1 + 2")
    );

    let input = "1+2-1";
    let mut parser = Parser::new(input);
    let result = parser.parse();
    assert_eq!(
        result.expect("not getting expression").to_string(),
        String::from("1 + 2 - 1")
    );

    let input = "let t := 1";
    let mut parser = Parser::new(input);
    let result = parser.parse();
    assert_eq!(
        result.expect("not getting expression").to_string(),
        String::from("let t := 1")
    );

    let input = "var y = 1d";
    let mut parser = Parser::new(input);
    let result = parser.parse();
    assert_eq!(
        result
            .expect_err("erroneous assignment operator.")
            .to_string(),
        String::from(
            "Syntax error => Unexpected token. It found: =, it was expected: :=. (Line 1 |6..7|)"
        )
    );

    let input = "let t := b";
    let mut parser = Parser::new(input);
    let result = parser.parse();
    assert_eq!(
        result.expect("not getting expression").to_string(),
        String::from("let t := b")
    );

    let input = "let t := b + 1";
    let mut parser = Parser::new(input);
    let result = parser.parse();
    assert_eq!(
        result.expect("not getting expression").to_string(),
        String::from("let t := b + 1")
    );
}
