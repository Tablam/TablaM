use logos::source::Source;
use logos::Logos;
use std::fmt;
use std::ops::Range;

use tablam::rust_decimal::prelude::*;
use tablam_lang::lexer::{Scanner, Token, TokenData};

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
fn test_lexer_data() {
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
fn test_assignment() {
    assert_lex(
        "let int := 1",
        &[
            (Token::Let, "let", 0..3),
            (Token::Variable("int".to_string()), "int", 4..7),
            (Token::Assignment, ":=", 8..10),
            (Token::Integer(1i64), "1", 11..12),
        ],
    );

    let source = "int += 1";
    assert_lex(
        source,
        &[
            (Token::Variable("int".to_string()), "int", 0..3),
            (Token::PlusEqual, "+=", 4..6),
            (Token::Integer(1), "1", 7..8),
        ],
    );

    let source = "int += 2 * 1";
    assert_lex(
        source,
        &[
            (Token::Variable("int".to_string()), "int", 0..3),
            (Token::PlusEqual, "+=", 4..6),
            (Token::Integer(2), "2", 7..8),
            (Token::Multiplication, "*", 9..10),
            (Token::Integer(1), "1", 11..12),
        ],
    );

    let source = "int -= n";
    assert_lex(
        source,
        &[
            (Token::Variable("int".to_string()), "int", 0..3),
            (Token::MinusEqual, "-=", 4..6),
            (Token::Variable("n".to_string()), "n", 7..8),
        ],
    );

    let source = "int /= n";
    assert_lex(
        source,
        &[
            (Token::Variable("int".to_string()), "int", 0..3),
            (Token::DivisionEqual, "/=", 4..6),
            (Token::Variable("n".to_string()), "n", 7..8),
        ],
    );

    let source = "int *= n";
    assert_lex(
        source,
        &[
            (Token::Variable("int".to_string()), "int", 0..3),
            (Token::MultiplicationEqual, "*=", 4..6),
            (Token::Variable("n".to_string()), "n", 7..8),
        ],
    );
}

#[test]
fn test_commentaries() {
    assert_lex(
        "let sum := 1 + 1  -- sum = 2
    -- test one line
    var c := 1 + 2
    --- header ---
    collection ?where #name > 1
    --- multiline
    commentaries
    example
    ---",
        &[
            (Token::Let, "let", 0..3),
            (Token::Variable(String::from("sum")), "sum", 4..7),
            (Token::Assignment, ":=", 8..10),
            (Token::Integer(1i64), "1", 11..12),
            (Token::Plus, "+", 13..14),
            (Token::Integer(1i64), "1", 15..16),
            (Token::Var, "var", 54..57),
            (Token::Variable(String::from("c")), "c", 58..59),
            (Token::Assignment, ":=", 60..62),
            (Token::Integer(1i64), "1", 63..64),
            (Token::Plus, "+", 65..66),
            (Token::Integer(2i64), "2", 67..68),
            (
                Token::Variable(String::from("collection")),
                "collection",
                92..102,
            ),
            (Token::Where, "?where", 103..109),
            (Token::Column(String::from("name")), "#name", 110..115),
            (Token::Greater, ">", 116..117),
            (Token::Integer(1i64), "1", 118..119),
        ],
    );
}

#[test]
fn test_collections() {
    assert_lex(
        "let empty := []",
        &[
            (Token::Let, "let", 0..3),
            (Token::Variable("empty".to_string()), "empty", 4..9),
            (Token::Assignment, ":=", 10..12),
            (Token::StartVector, "[", 13..14),
            (Token::EndVector, "]", 14..15),
        ],
    );

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

    assert_lex(
        "let complex := [real:Int, img:Decimal; 1,1d; 2,2d; 3,3d]",
        &[
            (Token::Let, "let", 0..3),
            (Token::Variable("complex".to_string()), "complex", 4..11),
            (Token::Assignment, ":=", 12..14),
            (Token::StartVector, "[", 15..16),
            (Token::Variable(String::from("real")), "real", 16..20),
            (Token::TypeDefiner, ":", 20..21),
            (Token::Type(String::from("Int")), "Int", 21..24),
            (Token::Separator, ",", 24..25),
            (Token::Variable(String::from("img")), "img", 26..29),
            (Token::TypeDefiner, ":", 29..30),
            (Token::Type(String::from("Decimal")), "Decimal", 30..37),
            (Token::RowSeparator, ";", 37..38),
            (Token::Integer(1i64), "1", 39..40),
            (Token::Separator, ",", 40..41),
            (Token::Decimal(Decimal::from(1)), "1d", 41..43),
            (Token::RowSeparator, ";", 43..44),
            (Token::Integer(2i64), "2", 45..46),
            (Token::Separator, ",", 46..47),
            (Token::Decimal(Decimal::from(2)), "2d", 47..49),
            (Token::RowSeparator, ";", 49..50),
            (Token::Integer(3i64), "3", 51..52),
            (Token::Separator, ",", 52..53),
            (Token::Decimal(Decimal::from(3)), "3d", 53..55),
            (Token::EndVector, "]", 55..56),
        ],
    );
}

#[test]
fn test_strings() {
    assert_lex(r#""a""#, &[(Token::String("a".into()), r#""a""#, 0..3)]);
    assert_lex("'a'", &[(Token::String("a".into()), "'a'", 0..3)]);
}

#[test]
fn test_numbers() {
    assert_lex(
        "let int := 1 \nlet float := 1.1f \nlet money := 10d \nvar payment := 150.5 \npayment += money",
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
            (Token::Decimal(Decimal::from_str("150.5").unwrap()), "150.5", 66..71),
            (Token::Variable("payment".to_string()), "payment", 73..80),
            (Token::PlusEqual, "+=", 81..83),
            (Token::Variable("money".to_string()), "money", 84..89)
        ],
    );

    let source = "let b := 1b";
    assert_lex(
        source,
        &[
            (Token::Let, "let", 0..3),
            (Token::Variable("b".to_string()), "b", 4..5),
            (Token::Assignment, ":=", 6..8),
            (Token::Bit("1".to_string()), "1b", 9..11),
        ],
    );

    let source = "let b := 101001b";
    assert_lex(
        source,
        &[
            (Token::Let, "let", 0..3),
            (Token::Variable("b".to_string()), "b", 4..5),
            (Token::Assignment, ":=", 6..8),
            (Token::Bit("101001".to_string()), "101001b", 9..16),
        ],
    );

    // let result: Vec<_> = Token::lexer(source).spanned().collect();
    // dbg!(result);
}
