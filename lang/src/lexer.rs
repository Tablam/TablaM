use logos::{Lexer, Logos, Skip, Span};

use std::iter::Peekable;
use tablam::derive_more::Display;
use tablam::prelude::*;

#[derive(Debug, Clone, PartialEq, Display)]
#[display(fmt = "Line {} ({:?}:{:?})", line, range_column, line_range_column)]
pub struct TokenData {
    pub line: usize,
    pub range_column: Span,
    pub line_range_column: Span,
}

#[derive(Debug, Clone, Copy)]
pub struct ExtrasLexer {
    current_line: usize,
    current_initial_column: usize,
}

impl Default for ExtrasLexer {
    fn default() -> Self {
        ExtrasLexer {
            current_line: 1,
            current_initial_column: 0,
        }
    }
}

fn increase_current_line(lexer: &mut Lexer<Token>) -> Skip {
    lexer.extras.current_line += 1;
    lexer.extras.current_initial_column = lexer.span().end;
    Skip
}

fn parse_token_data_without_suffix<T>(lexer: &mut Lexer<Token>, suffix_len: usize) -> Option<T>
where
    T: std::fmt::Debug + std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let value = lexer.slice();
    let parsed_value: T = value[..value.len() - suffix_len].parse().unwrap();

    Some(parsed_value)
}

fn parse_token_data<T>(lexer: &mut Lexer<Token>) -> Option<T>
where
    T: std::fmt::Debug + std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let parsed_value: T = lexer.slice().parse().unwrap();

    Some(parsed_value)
}

pub(crate) fn extract_token_data(lexer: &mut Lexer<Token>) -> TokenData {
    let columns = lexer.span();
    let start_column = columns.start - lexer.extras.current_initial_column;
    let end_column = (columns.end - columns.start) + start_column;
    TokenData {
        line: lexer.extras.current_line,
        line_range_column: Span {
            start: start_column,
            end: end_column,
        },
        range_column: columns,
    }
}

#[derive(Logos, Debug, Clone, PartialEq, Display)]
#[logos(extras = ExtrasLexer)]
#[display(fmt = "{}")]
pub enum Token {
    //Definition keywords
    #[token("let")]
    Let,
    #[token("var")]
    Var,

    //Identifiers
    #[regex(r"[[:upper:]]+(?:_[[[:upper:]][[:digit:]]]+)*", |lex| parse_token_data::<String>(lex))]
    Constant(String),
    #[regex(r"[[:upper:]](?:[[[:lower:]][[:digit:]]])+(?:_[[:upper:]][[[:lower:]][[:digit:]]]+)*", |lex| parse_token_data::<String>(lex))]
    Type(String),
    #[regex(r"[[:lower:]][[[:lower:]][[:digit:]]]+(?:_[[[:lower:]][[:digit:]]]+)*", |lex| parse_token_data::<String>(lex))]
    Variable(String),
    #[token(":=")]
    Assignment,
    #[token(":")]
    TypeDefiner,

    //Boolean
    #[token("true")]
    True,
    #[token("false")]
    False,

    //Bool operators
    #[token("=")]
    Equal,
    #[token("<>")]
    NotEqual,
    #[token(">")]
    Greater,
    #[token("<")]
    Less,
    #[token(">=")]
    GreaterEqual,
    #[token("<=")]
    LessEqual,
    #[token("not")]
    Not,
    #[token("and")]
    And,
    #[token("or")]
    Or,

    //Grouping
    #[token("(")]
    LeftParentheses,
    #[token(")")]
    RightParentheses,

    //Numbers
    #[regex(r"\d+", |lex| parse_token_data::<i64>(lex))]
    Integer(i64),
    #[regex(r"\d+\.*\d*f", |lex| parse_token_data_without_suffix::<R64>(lex, 1))]
    Float(R64),
    #[regex(r"\d+\.*\d*d", |lex| parse_token_data_without_suffix::<Decimal>(lex, 1))]
    Decimal(Decimal),

    //Arithmetic operators
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Multiplication,
    #[token("/")]
    Division,
    #[token("+=")]
    PlusEqual,
    #[token("-=")]
    MinusEqual,
    #[token("*=")]
    MultiplicationEqual,
    #[token("/=")]
    DivisionEqual,

    //Definitions
    #[token("do")]
    Start,
    #[token("end")]
    End,

    //Control flow
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("elif")]
    Elif,
    #[token("for")]
    For,
    #[token("in")]
    In,
    #[token("while")]
    While,
    #[token("continue")]
    Continue,
    #[token("break")]
    Break,

    //Functions
    #[token("func")]
    Function,
    #[token(",")]
    Separator,
    #[token("=>")]
    ReturnDefiner,
    #[token("return")]
    Return,

    /*
       //Strings
       #[regex(r#""[\w\d\s[^\s"{}]]+""#)]
       String,
       #[regex(r#""""[\w\d\s[^\s"{}]]+""""#)]
       Multiline,
    */
    #[token("\n", increase_current_line)]
    #[regex(r"[ \t\f]+", logos::skip)]
    #[error]
    Error,
}

pub struct Scanner<'source> {
    tokens: Peekable<DataIter<'source>>,
}

impl<'source> Scanner<'source> {
    pub fn new(buffer: &'source str) -> Self {
        let lexer = Token::lexer(buffer);
        let tokens = DataIter { lexer }.peekable();
        Scanner { tokens }
    }

    pub fn peek_both(&mut self) -> Option<&(Token, TokenData)> {
        self.tokens.peek()
    }

    pub fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek().map(|(x, _)| x)
    }

    pub fn accept(&mut self) -> Option<(Token, TokenData)> {
        self.tokens.next()
    }
}

pub struct DataIter<'source> {
    lexer: Lexer<'source, Token>,
}

impl<'source> Iterator for DataIter<'source> {
    type Item = (Token, TokenData);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(token) = self.lexer.next() {
            let data = extract_token_data(&mut self.lexer);
            Some((token, data))
        } else {
            None
        }
    }
}
