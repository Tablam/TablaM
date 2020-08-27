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
fn test_syntax_collections() {
    assert_lex(
        "let n := [8; 9; 10]",
        &[
            (Token::Let, "let", 0..3),
            (Token::Variable("n".to_string()), "n", 4..5),
            (Token::Assignment, ":=", 6..8),
            (Token::StartVector, "[", 9..10),
            (Token::Integer(8i64), "8", 10..11),
            (Token::RowSeparator, ";", 11..12),
            (Token::Integer(9i64), "9", 13..14),
            (Token::RowSeparator, ";", 14..15),
            (Token::Integer(10i64), "10", 16..18),
            (Token::EndVector, "]", 18..19),
        ],
    );

    assert_lex(
        "let num := [Int; 5; 6; 7]",
        &[
            (Token::Let, "let", 0..3),
            (Token::Variable("num".to_string()), "num", 4..7),
            (Token::Assignment, ":=", 8..10),
            (Token::StartVector, "[", 11..12),
            (Token::Type(String::from("Int")), "Int", 12..15),
            (Token::RowSeparator, ";", 15..16),
            (Token::Integer(5i64), "5", 17..18),
            (Token::RowSeparator, ";", 18..19),
            (Token::Integer(6i64), "6", 20..21),
            (Token::RowSeparator, ";", 21..22),
            (Token::Integer(7i64), "7", 23..24),
            (Token::EndVector, "]", 24..25),
        ],
    );

    assert_lex(
        "let numbers := [name:Int; 1; 2; 3; 4]",
        &[
            (Token::Let, "let", 0..3),
            (Token::Variable("numbers".to_string()), "numbers", 4..11),
            (Token::Assignment, ":=", 12..14),
            (Token::StartVector, "[", 15..16),
            (Token::Variable(String::from("name")), "name", 16..20),
            (Token::TypeDefiner, ":", 20..21),
            (Token::Type(String::from("Int")), "Int", 21..24),
            (Token::RowSeparator, ";", 24..25),
            (Token::Integer(1i64), "1", 26..27),
            (Token::RowSeparator, ";", 27..28),
            (Token::Integer(2i64), "2", 29..30),
            (Token::RowSeparator, ";", 30..31),
            (Token::Integer(3i64), "3", 32..33),
            (Token::RowSeparator, ";", 33..34),
            (Token::Integer(4i64), "4", 35..36),
            (Token::EndVector, "]", 36..37),
        ],
    );

    let result: Vec<_> = Token::lexer("let numbers := [name:Int; 1; 2; 3; 4]")
        .spanned()
        .collect();
    dbg!(result);
}

#[test]
fn test_strings() {
    assert_lex(r#""a""#, &[(Token::String("a".into()), r#""a""#, 0..3)]);
    assert_lex("'a'", &[(Token::String("a".into()), "'a'", 0..3)]);
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

    let input = "let t := a and b";
    let mut parser = Parser::new(input);
    let result = parser.parse();
    assert_eq!(
        result.expect("not getting expression").to_string(),
        String::from("let t := a and b")
    );

    let input = "let t := a and b or 1 <> 2";
    let mut parser = Parser::new(input);
    let result = parser.parse();
    assert_eq!(
        result.expect("not getting expression").to_string(),
        String::from("let t := a and b or 1 <> 2")
    );

    let input = "let empty := []";
    let mut parser = Parser::new(input);
    let result = parser.parse();
    assert_eq!(
        result.expect("not getting expression").to_string(),
        String::from("let empty := Vec[it:Any;]")
    );

    let input = "let n := [9; 8; 10]";
    let mut parser = Parser::new(input);
    let result = parser.parse();
    assert_eq!(
        result.expect("not getting expression").to_string(),
        String::from("let n := Vec[it:Int; 9; 8; 10]")
    );

    let input = "let complex := [real:Decimal, img:Int; 1d,3; 3d,4; 4d,5;]";
    let mut parser = Parser::new(input);
    let result = parser.parse();
    assert_eq!(
        result.expect("not getting expression").to_string(),
        String::from("let complex := Vec[real:Dec, img:Int; 1, 3; 3, 4; 4, 5]")
    );

    /*let input = "let n := [9; 8; 10]";
    let mut parser = Parser::new(input);
    let result = parser.parse();
    println!("{}", input);
    dbg!(result);
    dbg!(result.expect("not getting expression").to_string());
    assert_eq!(
        result.expect("not getting expression").to_string(),
        String::from("let complex := Vec[real:Dec, img:Int; 1, 3; 3, 4; 4, 5]")
    );*/
}
