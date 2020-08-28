use logos::{Lexer, Logos, Skip, Span};

use std::iter::Peekable;
use tablam::derive_more::Display;
use tablam::prelude::*;

#[derive(Debug, Clone, PartialEq, Display)]
#[display(fmt = "Line {} |{:?}|", line, line_range_column)]
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

#[derive(Debug, Clone, PartialEq, Display)]
#[display(fmt = "#{} as {}", from, to)]
pub struct Alias {
    pub from: String,
    pub to: String,
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
    let parsed_value: T = value[..value.len() - suffix_len].parse::<T>().unwrap();

    Some(parsed_value)
}

fn parse_token_data_without_prefix<T>(lexer: &mut Lexer<Token>, prefix_len: usize) -> Option<T>
where
    T: std::fmt::Debug + std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let value = lexer.slice();
    let parsed_value: T = value[prefix_len..value.len()].parse::<T>().unwrap();

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

fn parse_token_quotes(lexer: &mut Lexer<Token>) -> Option<String> {
    let parsed_value: String = lexer.slice().parse().unwrap();

    Some(parsed_value[1..parsed_value.len() - 1].to_string())
}

fn parse_aliased_column(lexer: &mut Lexer<Token>) -> Option<Alias> {
    let target: String = lexer.slice().parse().unwrap();
    let delimiter = " as ";
    let init = target.find(delimiter).expect("As not found");
    let column = target[1..init].to_string();
    let alias = target[init + delimiter.len()..].to_string();
    Some(Alias {
        from: column,
        to: alias,
    })
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
    #[display(fmt = "let")]
    #[token("let")]
    Let,
    #[display(fmt = "var")]
    #[token("var")]
    Var,

    //Strings, capture with both single and double quote
    #[display(fmt = "{}", _0)]
    #[regex(
        r#""([^"\\]|\\t|\\u|\\n|\\")*"|'([^'\\]|\\t|\\u|\\n|\\')*'"#,
        parse_token_quotes
    )]
    String(String),
    /*
       #[regex(r#""""[\w\d\s[^\s"{}]]+""""#)]
       Multiline,
    */
    //Identifiers
    #[display(fmt = "{}", _0)]
    #[regex(r"[[:upper:]]+[_[[:upper:]][[:digit:]]]*", |lex| parse_token_data::<String>(lex))]
    Constant(String),
    #[display(fmt = "{}", _0)]
    #[regex(r"[[:upper:]](?:[[[:lower:]][[:digit:]]]+[[:upper:]]*)+", |lex| parse_token_data::<String>(lex))]
    Type(String),
    #[display(fmt = "{}", _0)]
    #[regex(r"[[:lower:]][_[[:lower:]][[:digit:]]]*", |lex| parse_token_data::<String>(lex))]
    Variable(String),
    #[display(fmt = ":=")]
    #[token(":=")]
    Assignment,
    #[display(fmt = ":")]
    #[token(":")]
    TypeDefiner,

    //Boolean
    #[display(fmt = "true")]
    #[token("true")]
    True,
    #[display(fmt = "false")]
    #[token("false")]
    False,

    //Bool operators
    #[display(fmt = "=")]
    #[token("=")]
    Equal,
    #[display(fmt = "<>")]
    #[token("<>")]
    NotEqual,
    #[display(fmt = ">")]
    #[token(">")]
    Greater,
    #[display(fmt = "<")]
    #[token("<")]
    Less,
    #[display(fmt = ">=")]
    #[token(">=")]
    GreaterEqual,
    #[display(fmt = "<=")]
    #[token("<=")]
    LessEqual,
    #[display(fmt = "not")]
    #[token("not")]
    Not,
    #[display(fmt = "and")]
    #[token("and")]
    And,
    #[display(fmt = "or")]
    #[token("or")]
    Or,

    //Grouping
    #[display(fmt = "(")]
    #[token("(")]
    LeftParentheses,
    #[display(fmt = ")")]
    #[token(")")]
    RightParentheses,

    //Numbers
    #[display(fmt = "{}", _0)]
    #[regex(r"\d+\.*\d*f", |lex| parse_token_data_without_suffix::<R64>(lex, 1))]
    Float(R64),
    #[display(fmt = "{}", _0)]
    #[regex(r"\d+\.*\d*d", |lex| parse_token_data_without_suffix::<Decimal>(lex, 1))]
    Decimal(Decimal),
    #[display(fmt = "{}", _0)]
    #[regex(r"\d+", |lex| parse_token_data::<i64>(lex))]
    Integer(i64),

    //Arithmetic operators
    #[display(fmt = "+")]
    #[token("+")]
    Plus,
    #[display(fmt = "-")]
    #[token("-")]
    Minus,
    #[display(fmt = "*")]
    #[token("*")]
    Multiplication,
    #[display(fmt = "/")]
    #[token("/")]
    Division,
    #[display(fmt = "+=")]
    #[token("+=")]
    PlusEqual,
    #[display(fmt = "-=")]
    #[token("-=")]
    MinusEqual,
    #[display(fmt = "*=")]
    #[token("*=")]
    MultiplicationEqual,
    #[display(fmt = "/=")]
    #[token("/=")]
    DivisionEqual,

    //Collections
    #[display(fmt = "[")]
    #[token("[")]
    StartVector,
    #[display(fmt = "]")]
    #[token("]")]
    EndVector,
    #[display(fmt = ";")]
    #[token(";")]
    RowSeparator,
    #[display(fmt = "#{}", _0)]
    #[regex("#[[:lower:]][_[[:lower:]][[:digit:]]]*", |lex| parse_token_data_without_prefix::<String>(lex, 1))]
    Column(String),
    #[display(fmt = "#{}", _0)]
    #[regex("#[[:digit:]]+", |lex| parse_token_data_without_prefix::<usize>(lex, 1))]
    IndexedColumn(usize),
    #[display(fmt = "#{}", _0)]
    #[regex(
        "#[[:lower:]][_[[:lower:]][[:digit:]]]* as [[:lower:]][_[[:lower:]][[:digit:]]]*",
        parse_aliased_column
    )]
    AliasedColumn(Alias),

    //Relational operators
    #[display(fmt = "?select")]
    #[token("?select")]
    Select,
    #[display(fmt = "?select")]
    #[token("?deselect")]
    Deselect,
    #[display(fmt = "?where")]
    #[token("?where")]
    Where,
    #[display(fmt = "?limit")]
    #[token("?limit")]
    Limit,
    #[display(fmt = "?skip")]
    #[token("?skip")]
    Skip,
    #[display(fmt = "?distinct")]
    #[token("?distinct")]
    Distinct,

    //Definitions
    #[display(fmt = "do")]
    #[token("do")]
    Start,
    #[display(fmt = "end")]
    #[token("end")]
    End,

    //Control flow
    #[display(fmt = "if")]
    #[token("if")]
    If,
    #[display(fmt = "else")]
    #[token("else")]
    Else,
    #[display(fmt = "elif")]
    #[token("elif")]
    Elif,
    #[display(fmt = "for")]
    #[token("for")]
    For,
    #[display(fmt = "in")]
    #[token("in")]
    In,
    #[display(fmt = "while")]
    #[token("while")]
    While,
    #[display(fmt = "continue")]
    #[token("continue")]
    Continue,
    #[display(fmt = "break")]
    #[token("break")]
    Break,

    //Functions
    #[display(fmt = "func")]
    #[token("func")]
    Function,
    #[display(fmt = ",")]
    #[token(",")]
    Separator,
    #[display(fmt = "return")]
    #[token("return")]
    Return,

    #[token("\n", increase_current_line)]
    #[regex(r"[ \t\f]+", logos::skip)]
    #[error]
    Error,
}

impl Token {
    pub fn can_reduce_to_value(&self) -> bool {
        self == &Token::True
            || self == &Token::False
            || self == &Token::Minus
            || self == &Token::MinusEqual
            || self == &Token::Multiplication
            || self == &Token::MultiplicationEqual
            || self == &Token::Division
            || self == &Token::DivisionEqual
    }

    pub fn is_literal_or_value(&self) -> bool {
        match self {
            Token::String(_)
            | Token::Constant(_)
            | Token::Variable(_)
            | Token::True
            | Token::False
            | Token::Float(_)
            | Token::Decimal(_)
            | Token::Integer(_) => true,
            _ => false,
        }
    }

    pub fn is_binary_operator(&self) -> bool {
        self == &Token::Plus
            || self == &Token::PlusEqual
            || self == &Token::Minus
            || self == &Token::MinusEqual
            || self == &Token::Multiplication
            || self == &Token::MultiplicationEqual
            || self == &Token::Division
            || self == &Token::DivisionEqual
    }

    pub fn is_comparison_operator(&self) -> bool {
        self == &Token::Equal
            || self == &Token::NotEqual
            || self == &Token::Greater
            || self == &Token::GreaterEqual
            || self == &Token::Less
            || self == &Token::LessEqual
            || self == &Token::And
            || self == &Token::Or
    }
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

    pub fn peek_both(&mut self) -> Option<(Token, TokenData)> {
        self.tokens.peek().cloned()
    }

    pub fn peek(&mut self) -> Option<Token> {
        self.tokens.peek().cloned().map(|(x, _)| x)
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
