use std::iter::Peekable;
use tablam::decorum::R64;
use tablam::rust_decimal::Decimal;

use logos::{Lexer, Logos, Skip};

pub struct ExtrasLexer {
    current_line: usize,
}

impl Default for ExtrasLexer {
    fn default() -> Self {
        ExtrasLexer { current_line: 1 }
    }
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

fn parse_token_data_without_suffix<T>(
    lexer: &mut Lexer<Token>,
    suffix_len: usize,
) -> Option<TokenData<T>>
where
    T: std::fmt::Debug + std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let value = lexer.slice();
    let parsed_value: T = value[..value.len() - suffix_len].parse().unwrap();
    let mut token_data: TokenData<T> = extract_token_data(lexer).unwrap();
    token_data.value = Some(parsed_value);

    Some(token_data)
}

fn parse_token_data_without_prefix<T>(
    lexer: &mut Lexer<Token>,
    suffix_len: usize,
) -> Option<TokenData<T>>
where
    T: std::fmt::Debug + std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let value = lexer.slice();
    let parsed_value: T = value[suffix_len - 1..value.len()].parse().unwrap();
    let mut token_data: TokenData<T> = extract_token_data(lexer).unwrap();
    token_data.value = Some(parsed_value);

    Some(token_data)
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
    #[token(":=", |lex| extract_token_data::<String>(lex))]
    Assignment(TokenData<String>),
    #[token(":", |lex| extract_token_data::<String>(lex))]
    TypeDefiner(TokenData<String>),

    //Boolean
    #[token("true", |lex| extract_token_data::<String>(lex))]
    True(TokenData<String>),
    #[token("false", |lex| extract_token_data::<String>(lex))]
    False(TokenData<String>),

    //Bool operators
    #[token("=", |lex| extract_token_data::<String>(lex))]
    Equal(TokenData<String>),
    #[token("<>", |lex| extract_token_data::<String>(lex))]
    NotEqual(TokenData<String>),
    #[token(">", |lex| extract_token_data::<String>(lex))]
    Greater(TokenData<String>),
    #[token("<", |lex| extract_token_data::<String>(lex))]
    Less(TokenData<String>),
    #[token(">=", |lex| extract_token_data::<String>(lex))]
    GreaterEqual(TokenData<String>),
    #[token("<=", |lex| extract_token_data::<String>(lex))]
    LessEqual(TokenData<String>),
    #[token("not", |lex| extract_token_data::<String>(lex))]
    Not(TokenData<String>),
    #[token("and", |lex| extract_token_data::<String>(lex))]
    And(TokenData<String>),
    #[token("or", |lex| extract_token_data::<String>(lex))]
    Or(TokenData<String>),

    //Grouping
    #[token("(", |lex| extract_token_data::<String>(lex))]
    LeftParentheses(TokenData<String>),
    #[token(")", |lex| extract_token_data::<String>(lex))]
    RightParentheses(TokenData<String>),

    //Numbers
    #[regex(r"\d+", |lex| parse_token_data::<i64>(lex))]
    Integer(TokenData<i64>),
    #[regex(r"\d+\.*\d*f", |lex| parse_token_data_without_suffix::<R64>(lex, 1))]
    Float(TokenData<R64>),
    #[regex(r"\d+\.*\d*d", |lex| parse_token_data_without_suffix::<Decimal>(lex, 1))]
    Decimal(TokenData<Decimal>),

    //Arithmetic operators
    #[token("+", |lex| extract_token_data::<String>(lex))]
    Plus(TokenData<String>),
    #[token("-", |lex| extract_token_data::<String>(lex))]
    Minus(TokenData<String>),
    #[token("*", |lex| extract_token_data::<String>(lex))]
    Multiplication(TokenData<String>),
    #[token("/", |lex| extract_token_data::<String>(lex))]
    Division(TokenData<String>),
    #[token("+=", |lex| extract_token_data::<String>(lex))]
    PlusEqual(TokenData<String>),
    #[token("-=", |lex| extract_token_data::<String>(lex))]
    MinusEqual(TokenData<String>),
    #[token("*=", |lex| extract_token_data::<String>(lex))]
    MultiplicationEqual(TokenData<String>),
    #[token("/=", |lex| extract_token_data::<String>(lex))]
    DivisionEqual(TokenData<String>),

    //Definitions
    #[token("do", |lex| extract_token_data::<String>(lex))]
    Start(TokenData<String>),
    #[token("end", |lex| extract_token_data::<String>(lex))]
    End(TokenData<String>),

    //Control flow
    #[token("if", |lex| extract_token_data::<String>(lex))]
    If(TokenData<String>),
    #[token("else", |lex| extract_token_data::<String>(lex))]
    Else(TokenData<String>),
    #[token("elif", |lex| extract_token_data::<String>(lex))]
    Elif(TokenData<String>),
    #[token("for", |lex| extract_token_data::<String>(lex))]
    For(TokenData<String>),
    #[token("in", |lex| extract_token_data::<String>(lex))]
    In(TokenData<String>),
    #[token("while", |lex| extract_token_data::<String>(lex))]
    While(TokenData<String>),
    #[token("continue", |lex| extract_token_data::<String>(lex))]
    Continue(TokenData<String>),
    #[token("break", |lex| extract_token_data::<String>(lex))]
    Break(TokenData<String>),

    //Functions
    #[token("func", |lex| extract_token_data::<String>(lex))]
    Function(TokenData<String>),
    #[token(",", |lex| extract_token_data::<String>(lex))]
    Separator(TokenData<String>),
    #[token("=>", |lex| extract_token_data::<String>(lex))]
    ReturnDefiner(TokenData<String>),
    #[token("return", |lex| extract_token_data::<String>(lex))]
    Return(TokenData<String>),

    /*
       //Strings
       #[regex(r#""[\w\d\s[^\s"{}]]+""#)]
       String,
       #[regex(r#""""[\w\d\s[^\s"{}]]+""""#)]
       Multiline,
    */
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
