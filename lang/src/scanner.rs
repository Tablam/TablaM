use std::iter::Peekable;
use tablam::prelude::*;

use logos::{Lexer, Logos, Skip, Span};

#[derive(Default)]
pub struct ExtrasLexer {
    current_line: usize,
}

fn increase_current_line(lexer: &mut Lexer<Token>) -> Skip {
    lexer.extras.current_line += 1;
    Skip
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenData<T> {
    pub value: Option<T>,
    pub line: usize,
    pub range_column: logos::Span,
}

fn parse_token_data<T>(lexer: &mut Lexer<Token>) -> Option<TokenData<T>>
where
    T: std::fmt::Debug + std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let parsed_value: T = lexer.slice().parse().unwrap();
    let mut token_data: TokenData<T> = extract_token_data(lexer).unwrap();
    token_data.value = Some(parsed_value);

    Some(token_data)
}

fn extract_token_data<T>(lexer: &mut Lexer<Token>) -> Option<TokenData<T>> {
    Some(TokenData::<T> {
        line: lexer.extras.current_line,
        range_column: lexer.span(),
        value: None,
    })
}

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(extras = ExtrasLexer)]
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
    False,*/
    //Definition keywords
    #[token("let", |lex| extract_token_data::<String>(lex))]
    Let(TokenData<String>),
    #[token("var", |lex| extract_token_data::<String>(lex))]
    Var(TokenData<String>),

    //Identifiers
    #[regex(r"[[:upper:]]+(?:_[[[:upper:]][[:digit:]]]+)*", |lex| parse_token_data::<String>(lex))]
    Constant(TokenData<String>),
    #[regex(r"[[:upper:]](?:[[[:lower:]][[:digit:]]])+(?:_[[:upper:]][[[:lower:]][[:digit:]]]+)*", |lex| parse_token_data::<String>(lex))]
    Type(TokenData<String>),
    #[regex(r"[[:lower:]][[[:lower:]][[:digit:]]]+(?:_[[[:lower:]][[:digit:]]]+)*", |lex| parse_token_data::<String>(lex))]
    Variable(TokenData<String>),

    //Operators
    #[token(":=", |lex| extract_token_data::<String>(lex))]
    Assignment(TokenData<String>),
    /*
    #[token("=")]
    Equal,
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

pub struct Scanner<'source> {
    tokens: Peekable<Lexer<'source, Token>>,
}

impl<'source> Scanner<'source> {
    pub fn new(buffer: &'source str) -> Self {
        let lexer = Token::lexer(buffer).peekable();
        Scanner { tokens: lexer }
    }

    pub fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }

    pub fn accept(&mut self) -> Option<Token> {
        self.tokens.next()
    }
}
