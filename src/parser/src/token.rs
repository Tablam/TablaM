use crate::files::FileId;
use corelib::derive_more::Display;

use logos::{Lexer, Logos};
use text_size::TextRange;

#[derive(Debug, Clone, Copy)]
pub struct ExtrasLexer {
    pub current_line: usize,
    pub current_initial_column: usize,
}

impl Default for ExtrasLexer {
    fn default() -> Self {
        ExtrasLexer {
            current_line: 1,
            current_initial_column: 0,
        }
    }
}

fn increase_current_line(lexer: &mut Lexer<Syntax>) {
    //When a line-feed happens, it reset the position of the "column"
    lexer.extras.current_line += 1;
    lexer.extras.current_initial_column = lexer.span().end;
}

/// Classify the kind of syntax for the parse, so it knows when to apply precedence...
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyntaxKind {
    Infix,
    Prefix,
    Postfix,
    Open,
    Close,
    Trivia,
    Kw,
    Expr,
    Err,
}

//TODO: https://github.com/YoloDev/yolodev-jsonnet/blob/master/crates/lex/src/lib.rs
// For ideas for the lexer
//TODO: Support count lines collapsing many CR?
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Display, Logos)]
#[logos(extras = ExtrasLexer)]
#[repr(u16)]
pub enum Syntax {
    //trivia
    #[token("\n", increase_current_line)]
    Cr,
    #[regex(r"[ \t\f]+")]
    Whitespace,
    #[regex("#.*")]
    Comment,

    //literals
    #[regex("true|false")]
    Bool,
    #[regex("[0-9]+")]
    Int64,

    //keywords
    #[display(fmt = "fun")]
    #[token("fun")]
    FnKw,

    #[display(fmt = "let")]
    #[token("let")]
    LetKw,

    #[display(fmt = "var")]
    #[token("var")]
    VarKw,

    //idents
    #[regex("[A-Za-z][A-Za-z0-9]*")]
    Ident,

    //OPS

    //Punctuation
    #[display(fmt = ".")]
    #[token(".")]
    Point,

    #[display(fmt = ",")]
    #[token(",")]
    Comma,

    #[display(fmt = ";")]
    #[token(";")]
    Semicolon,

    #[display(fmt = "?")]
    #[token("?")]
    Question,

    //Math
    Neg, // Minus get turned Neg only in parser

    #[display(fmt = "+")]
    #[token("+")]
    Plus,
    Positive, // Plus get turned Positive only in parser

    #[display(fmt = "-")]
    #[token("-")]
    Minus,

    #[display(fmt = "*")]
    #[token("*")]
    Star,

    #[display(fmt = "/")]
    #[token("/")]
    Slash,

    //Logic
    #[display(fmt = ":=")]
    #[token(":=")]
    Assign,

    #[display(fmt = "=")]
    #[token("=")]
    Equals,

    #[display(fmt = "!=")]
    #[token("!=")]
    NotEquals,

    #[display(fmt = "<")]
    #[token("<")]
    Less,
    #[display(fmt = "<=")]
    #[token("<=")]
    LessThan,

    #[display(fmt = ">")]
    #[token(">")]
    Greater,
    #[display(fmt = ">=")]
    #[token(">=")]
    GreaterThan,

    #[display(fmt = "and")]
    #[token("and")]
    AndKw,

    #[display(fmt = "or")]
    #[token("or")]
    OrKw,

    #[display(fmt = "not")]
    #[token("not")]
    NotKw,

    //Grouping
    #[display(fmt = "(")]
    #[token("(")]
    LParen,

    #[display(fmt = ")")]
    #[token(")")]
    RParen,

    #[display(fmt = "{{")]
    #[token("{")]
    LBrace,

    #[display(fmt = "}}")]
    #[token("}")]
    RBrace,

    #[display(fmt = "[")]
    #[token("[")]
    LSquare,

    #[display(fmt = "]")]
    #[token("]")]
    RSquare,

    #[error]
    Error,
}

impl Syntax {
    pub fn is(self) -> SyntaxKind {
        match self {
            Syntax::Cr | Syntax::Whitespace | Syntax::Comment => SyntaxKind::Trivia,
            Syntax::Bool | Syntax::Int64 | Syntax::Ident | Syntax::Assign | Syntax::LetKw => {
                SyntaxKind::Expr
            }
            Syntax::FnKw | Syntax::VarKw => SyntaxKind::Kw,
            Syntax::Point
            | Syntax::Question
            | Syntax::Plus
            | Syntax::Minus
            | Syntax::Star
            | Syntax::Slash
            | Syntax::Comma
            | Syntax::Semicolon
            | Syntax::Equals
            | Syntax::NotEquals
            | Syntax::Less
            | Syntax::LessThan
            | Syntax::Greater
            | Syntax::GreaterThan
            | Syntax::AndKw
            | Syntax::OrKw
            | Syntax::NotKw => SyntaxKind::Infix,
            Syntax::Neg | Syntax::Positive => SyntaxKind::Prefix,
            Syntax::LParen | Syntax::LBrace | Syntax::LSquare => SyntaxKind::Open,
            Syntax::RParen | Syntax::RBrace | Syntax::RSquare => SyntaxKind::Close,
            Syntax::Error => SyntaxKind::Err,
        }
    }
    pub fn is_head_tree(self) -> bool {
        self.is() != SyntaxKind::Expr
    }
    pub fn is_bin_op(self) -> bool {
        matches!(self, Self::Plus | Self::Minus | Self::Star | Self::Slash)
    }

    pub fn to_bin_op(self) -> BinaryOp {
        match self {
            Syntax::Plus => BinaryOp::Add,
            Syntax::Minus => BinaryOp::Sub,
            Syntax::Star => BinaryOp::Mul,
            Syntax::Slash => BinaryOp::Div,
            _ => unreachable!(),
        }
    }
    pub fn is_unary_op(self) -> bool {
        matches!(self, Self::Neg | Self::Positive)
    }
    pub fn to_unary_op(self) -> UnaryOp {
        match self {
            Syntax::Neg => UnaryOp::Neg,
            _ => unreachable!(),
        }
    }

    pub fn is_op(self) -> bool {
        self.is_unary_op() || self.is_bin_op() || matches!(self, Self::Point)
    }

    pub fn is_separator(self) -> bool {
        matches!(self, Self::Comma | Self::Semicolon)
    }
    pub fn to_separator(self) -> SepOp {
        match self {
            Syntax::Comma => SepOp::Comma,
            Syntax::Semicolon => SepOp::Semicolon,
            _ => unreachable!("{}", self),
        }
    }
    pub fn is_var_let(self) -> bool {
        matches!(self, Self::LetKw | Self::VarKw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CmpOp {
    Equals,
    NotEquals,
    Less,
    Greater,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SepOp {
    Comma,
    Semicolon,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Neg,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TokenId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Token {
    pub file_id: FileId,
    pub id: TokenId,
    pub kind: Syntax,
    pub range: TextRange,
    pub line: u32,
    pub col: u32,
}

impl Token {
    pub fn range_tokens(tokens: &[Token]) -> TextRange {
        let min = tokens.first().map(|x| x.range.start());
        let max = tokens.last().map(|x| x.range.start() + x.range.len());

        TextRange::new(min.unwrap_or_default(), max.unwrap_or_default())
    }
}

//#[cfg(test)]
pub(crate) fn token_test() -> Token {
    Token {
        file_id: FileId::from_index(0),
        id: TokenId(0),
        kind: Syntax::Error,
        range: Default::default(),
        line: 0,
        col: 0,
    }
}
