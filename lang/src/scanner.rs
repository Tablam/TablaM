use tablam::scalar::{DateTime,Time,Date};
use tablam::decorum::R64;
use tablam::rust_decimal::Decimal;

use logos::{Logos, Lexer, Span, Skip};

#[derive(Default)]
struct ExtrasLexer {
    current_line: usize
}

fn increase_current_line(lexer: &mut Lexer<Token>) -> Skip{
    lexer.extras.current_line += 1;
    Skip
}

#[derive(Debug, Clone, PartialEq)]
struct TokenData<T>{
    value: T,
    line: usize,
    range_column: logos::Span,
    has_inner_expression: bool
}

fn parse_token_data<T>(lexer: &mut Lexer<Token>) -> Option<TokenData<T>>{
    let parsed_value:T = lexer.slice().parse().unwrap();
    let mut token_data:TokenData<T> = extract_token_data(lexer).unwrap();
    token_data.value = parsed_value;

    Some(token_data)
}

fn extract_token_data<T>(lexer: &mut Lexer<Token>) -> Option<TokenData<T>>{

    Some(TokenData::<T>{line: lexer.extras.current_line, range_column: lexer.span(),
        value:None, has_inner_expression:false })
}

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(extras = Scanner)]
pub enum Token {
    //Numbers
    #[regex(r"\d+", |lex| parse_token_data::<i64>(lex))]
    Integer(TokenData<i64>),
    /*#[regex(r"\d+\.*\d*f")]
    Float,
    #[regex(r"\d+\.*\d*d")]
    Decimal,

    //Strings
    #[regex(r#""[\w\d\s[^\s"{}]]+""#)]
    String,
    #[regex(r#""""[\w\d\s[^\s"{}]]+""""#)]
    Multiline,

    //Boolean,
    #[token("true")]
    True,
    #[token("false")]
    False,

    //Definition keywords
    #[token("let")]
    Let,
    #[token("var")]
    Var,

    //Identifiers
    #[regex(r"[[:upper:]]+(?:_[[[:upper:]][[:digit:]]]+)*")]
    Constant,
    #[regex(r"[[:upper:]](?:[[[:lower:]][[:digit:]]])+(?:_[[:upper:]][[[:lower:]][[:digit:]]]+)*")]
    Type,
    #[regex(r"[[:lower:]][[[:lower:]][[:digit:]]]+(?:_[[[:lower:]][[:digit:]]]+)*")]
    Variable,

    //Operators
    #[token("=")]
    Equal,
    #[token(":=")]
    Assignment,
    #[token(":")]
    Start,
    #[token("+")]
    Plus,
    #[token("+=")]
    PlusEqual,*/

    #[token("\n", increase_current_line)]
    #[regex(r" ", logos::skip)]
    #[error]
    Error,
}