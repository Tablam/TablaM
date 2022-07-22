use crate::token::TokenId;
use corelib::scalar::Scalar;

pub type Return = std::result::Result<Ast, ()>;

#[derive(Debug, Clone)]
pub struct Node<T> {
    pub span: TokenId,
    pub val: T,
}

#[derive(Debug, Clone)]
pub enum Ast {
    Scalar(Node<Scalar>),
    Pass(TokenId),
    Eof(TokenId),
}
