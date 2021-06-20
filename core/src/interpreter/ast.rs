use std::mem::discriminant;

use crate::for_impl::*;
use crate::interpreter::Identifier;
use crate::prelude::*;

use derive_more::{Display, From};

#[derive(Debug, Clone, From)]
pub struct Block(pub(crate) Vec<Expr>);

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format_list(&self.0, self.0.len(), "", "", f)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Display, From)]
pub enum Expr {
    //Values
    #[from]
    #[display(fmt = "{}", _0)]
    Value(Scalar),
    #[display(fmt = "{}", _0)]
    Variable(Identifier),

    //Variable definitions
    #[display(fmt = "var {:} := {}", _0, _1)]
    Mutable(Identifier, Box<Expr>),
    #[display(fmt = "let {:} := {}", _0, _1)]
    Immutable(Identifier, Box<Expr>),

    #[from]
    #[display(fmt = "{}", _0)]
    Function(Function),
    #[from]
    #[display(fmt = "{}", _0)]
    FunctionCall(FunctionCall),

    #[from]
    #[display(fmt = "{}", _0)]
    BinaryOp(BinaryOperation),

    #[from]
    #[display(fmt = "{}", _0)]
    ComparisonOp(ComparisonOperation),

    #[display(fmt = "{}", _0)]
    Block(Block),

    #[display(fmt = "if {} do\n\t{}\nelse\n\t{}\nend", _0, _1, _2)]
    If(Box<BoolOperation>, Box<Expr>, Box<Expr>),

    #[display(fmt = "while {} do\n\t{}\nend", _0, _1)]
    While(Box<BoolOperation>, Box<Expr>),

    #[display(fmt = "for {} do\n\t{}\nend", _0, _1)]
    ForIn(Box<RangeOperation>, Box<Expr>),

    #[from]
    #[display(fmt = "{}", _0)]
    ParameterDefinition(Field),

    #[from]
    #[display(fmt = "{}", _0)]
    Column(Column),

    #[display(fmt = "{}", _0)]
    QueryOperation(QueryOperation),

    // #[display(fmt = "{}", _0)]
    // BoolConditionQry(Token, Comparable, Comparable),
    #[display(fmt = "{}", _0)]
    Error(String),
    #[display(fmt = "pass")]
    Pass,
    #[display(fmt = "eof")]
    Eof,
}

impl Expr {
    pub fn is(variant: &Self, expected: &Self) -> bool {
        discriminant(variant) == discriminant(expected)
    }

    pub fn is_eof(&self) -> bool {
        matches!(self, Expr::Eof)
    }

    pub fn is_indexed_column(&self) -> bool {
        matches!(self, Expr::Column(Column::Pos(_)))
    }
    //
    // pub fn create_bool_condition_qry(operator: Token, left: Self, right: Self) -> Self {
    //     let lhs: Comparable = match left {
    //         Expression::Column(col) => match col {
    //             Column::Pos(position) => position.into(),
    //             _ => unreachable!("Not condition query implemented by name."),
    //         },
    //         Expression::Value(scalar) => scalar.into(),
    //         _ => unreachable!("Invalidate expression in condition."),
    //     };
    //
    //     let rhs: Comparable = match right {
    //         Expression::Column(col) => match col {
    //             Column::Pos(position) => position.into(),
    //             _ => unreachable!("Not condition query implemented by name."),
    //         },
    //         Expression::Value(scalar) => scalar.into(),
    //         _ => unreachable!("Invalidate expression in condition."),
    //     };
    //
    //     Expression::BoolConditionQry(operator, lhs, rhs)
    // }
}

#[derive(Debug, Clone, Display)]
#[display(fmt = "{} {}", collection, query)]
pub struct QueryOperation {
    pub collection: Box<Expr>,
    pub query: QueryOp,
}

impl QueryOperation {
    pub fn new(collection: Expr, query: QueryOp) -> Self {
        QueryOperation {
            collection: Box::new(collection),
            query,
        }
    }

    pub fn select(mut self, columns: Vec<Column>) -> Self {
        self.query = self.query.select(&columns);
        self
    }

    pub fn deselect(mut self, columns: Vec<Column>) -> Self {
        self.query = self.query.deselect(&columns);
        self
    }

    pub fn filter(mut self, operator: &CmOp, left: Comparable, right: Comparable) -> Self {
        self.query = match operator {
            CmOp::Eq => self.query.eq(left, right),
            CmOp::NotEq => self.query.not_eq(left, right),
            CmOp::Greater => self.query.greater(left, right),
            CmOp::GreaterEq => self.query.greater_eq(left, right),
            CmOp::Less => self.query.less(left, right),
            CmOp::LessEq => self.query.less_eq(left, right),
        };

        self
    }

    pub fn skip(mut self, offset: usize) -> Self {
        self.query = self.query.skip(offset);
        self
    }

    pub fn limit(mut self, offset: usize) -> Self {
        self.query = self.query.limit(offset);
        self
    }

    pub fn distinct(mut self) -> Self {
        self.query = self.query.distinct();
        self
    }
}

#[derive(Debug, Clone, Display)]
#[display(fmt = "{} {} {}", left, operator, right)]
pub struct BinaryOperation {
    pub operator: BinOp,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

impl BinaryOperation {
    pub fn new(operator: BinOp, left: Box<Expr>, right: Box<Expr>) -> Self {
        BinaryOperation {
            operator,
            left,
            right,
        }
    }
}

#[derive(Debug, Clone, Display)]
#[display(fmt = "{} {} {}", left, operator, right)]
pub struct ComparisonOperation {
    pub operator: LogicOp,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone, Display)]
pub enum BoolOperation {
    Bool(bool),
    Var(String),
    Cmp(ComparisonOperation),
}

#[derive(Debug, Clone, Display)]
pub enum RangeOperation {
    #[display(fmt = "{} in {}..{}", _0, _1, _2)]
    StartEnd(String, i64, i64),
}

#[derive(Debug, Clone, Display)]
#[display(fmt = "{} := {}", name, value)]
pub struct ParamCall {
    pub name: String,
    pub value: Expr,
}

impl ParamCall {
    pub fn new(name: &str, value: Expr) -> Self {
        ParamCall {
            name: name.to_string(),
            value,
        }
    }
}

#[derive(Debug, Clone, From)]
pub struct FunctionCall {
    pub name: String,
    pub params: Vec<ParamCall>,
}

impl FunctionCall {
    pub fn new(name: &str, params: &[ParamCall]) -> Self {
        FunctionCall {
            name: name.into(),
            params: params.to_vec(),
        }
    }
}

impl fmt::Display for FunctionCall {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)?;
        format_list(&self.params, self.params.len(), "(", ")", f)?;
        Ok(())
    }
}

impl ComparisonOperation {
    pub fn new(operator: LogicOp, left: Box<Expr>, right: Box<Expr>) -> Self {
        ComparisonOperation {
            operator,
            left,
            right,
        }
    }
}
