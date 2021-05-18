use std::collections::HashMap;
use std::mem::discriminant;

use tablam::derive_more::{Display, From};
use tablam::prelude::{BinOp, Column, Comparable, Function, LogicOp, Param, QueryOp, Scalar};

use crate::lexer::{Token, TokenData};
use std::fmt;
use tablam::types::format_list;

pub type Identifier = String;

#[derive(Debug, Display, From)]
#[display(fmt = "Syntax error => {}")]
pub enum ErrorLang {
    #[from]
    #[display(fmt = "{}", _0)]
    CoreError(tablam::errors::Error),
    #[from]
    #[display(
        fmt = "Unexpected token. It found: {}, it was expected: {}. ({})",
        _0,
        _1,
        _2
    )]
    Unexpected(Token, Token, TokenData),
    #[display(
        fmt = "The field count not match, the header row declare {} columns, but at row {}, it have {} columns",
        _0,
        _1,
        _2
    )]
    FieldCountNotMatch(usize, usize, usize),
    #[display(fmt = "Unexpected item: {}", _0)]
    UnexpectedItem(Expression),
    #[display(fmt = "Syntax query error: {}", _0)]
    Query(String),

    #[from]
    #[display(fmt = "Unclosed group. It was expected: {}. ({})", _0, _1)]
    UnclosedGroup(Token, TokenData),
    #[display(fmt = "Variable '{}' not found in scope", _0)]
    VariableNotFound(String),
    #[display(fmt = "Function '{}' not found in scope", _0)]
    FunctionNotFound(String),
    #[display(
        fmt = "It was expected a boolean expression that return true OR false, but got: {}",
        _0
    )]
    ExpectedBoolOp(Token),
    #[display(fmt = "Unimplemented syntax. {:?}", _0)]
    Unimplemented(Token),
    #[display(fmt = "Unexpected EOF.")]
    Eof,
}

pub type Return = std::result::Result<Expression, ErrorLang>;
pub type ReturnT<T> = std::result::Result<T, ErrorLang>;

#[derive(Debug, Clone, From)]
pub struct Block(pub(crate) Vec<Expression>);

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format_list(&self.0, self.0.len(), "", "", f)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Display, From)]
pub enum Expression {
    //Values
    #[from]
    #[display(fmt = "{}", _0)]
    Value(Scalar),
    #[display(fmt = "{}", _0)]
    Variable(Identifier),

    //Variable definitions
    #[display(fmt = "var {:} := {}", _0, _1)]
    Mutable(Identifier, Box<Expression>),
    #[display(fmt = "let {:} := {}", _0, _1)]
    Immutable(Identifier, Box<Expression>),

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
    If(Box<BoolOperation>, Box<Expression>, Box<Expression>),

    #[display(fmt = "while {} do\n\t{}\nend", _0, _1)]
    While(Box<BoolOperation>, Box<Expression>),

    #[display(fmt = "for {} do\n\t{}\nend", _0, _1)]
    ForIn(Box<RangeOperation>, Box<Expression>),

    #[from]
    #[display(fmt = "{}", _0)]
    ParameterDefinition(Param),

    #[from]
    #[display(fmt = "{}", _0)]
    Column(Column),

    #[display(fmt = "{}", _0)]
    QueryOperation(QueryOperation),

    #[display(fmt = "{}", _0)]
    BoolConditionQry(Token, Comparable, Comparable),

    #[display(fmt = "{}", _0)]
    Error(String),
    #[display(fmt = "pass")]
    Pass,
    #[display(fmt = "eof")]
    Eof,
}

impl Expression {
    pub fn is(variant: &Self, expected: &Self) -> bool {
        discriminant(variant) == discriminant(expected)
    }

    pub fn is_eof(&self) -> bool {
        matches!(self, Expression::Eof)
    }

    pub fn is_indexed_column(&self) -> bool {
        matches!(self, Expression::Column(Column::Pos(_)))
    }

    pub fn create_bool_condition_qry(operator: Token, left: Self, right: Self) -> Self {
        let lhs: Comparable = match left {
            Expression::Column(col) => match col {
                Column::Pos(position) => position.into(),
                _ => unreachable!("Not condition query implemented by name."),
            },
            Expression::Value(scalar) => scalar.into(),
            _ => unreachable!("Invalidate expression in condition."),
        };

        let rhs: Comparable = match right {
            Expression::Column(col) => match col {
                Column::Pos(position) => position.into(),
                _ => unreachable!("Not condition query implemented by name."),
            },
            Expression::Value(scalar) => scalar.into(),
            _ => unreachable!("Invalidate expression in condition."),
        };

        Expression::BoolConditionQry(operator, lhs, rhs)
    }
}

#[derive(Debug, Clone, Display)]
#[display(fmt = "{} {}", collection, query)]
pub struct QueryOperation {
    pub collection: Box<Expression>,
    pub query: QueryOp,
}

impl QueryOperation {
    pub fn new(collection: Expression, query: QueryOp) -> Self {
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

    pub fn filter(mut self, operator: &Token, left: Comparable, right: Comparable) -> Self {
        self.query = match operator {
            Token::Equal => self.query.eq(left, right),
            Token::NotEqual => self.query.not_eq(left, right),
            Token::Greater => self.query.greater(left, right),
            Token::GreaterEqual => self.query.greater_eq(left, right),
            Token::Less => self.query.less(left, right),
            Token::LessEqual => self.query.less_eq(left, right),
            _ => unreachable!("Illegal operator in filter condition."),
        };

        self
    }

    pub fn skip(mut self, token: &Token) -> Self {
        self.query = match token {
            Token::Integer(offset) => self.query.skip(*offset as usize),
            _ => unreachable!("Illegal operator in filter condition."),
        };
        self
    }

    pub fn limit(mut self, token: &Token) -> Self {
        self.query = match token {
            Token::Integer(offset) => self.query.limit(*offset as usize),
            _ => unreachable!("Illegal operator in filter condition."),
        };
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
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

impl BinaryOperation {
    pub fn new(token: Token, left: Box<Expression>, right: Box<Expression>) -> Self {
        let operator = match token {
            Token::Plus => BinOp::Add,
            Token::Minus => BinOp::Minus,
            Token::Multiplication => BinOp::Mul,
            Token::Division => BinOp::Div,
            _ => unreachable!("Binary operator not implemented."),
        };

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
    pub left: Box<Expression>,
    pub right: Box<Expression>,
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
    pub value: Expression,
}

impl ParamCall {
    pub fn new(name: &str, value: Expression) -> Self {
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
    pub fn new(token: Token, left: Box<Expression>, right: Box<Expression>) -> Self {
        let operator = match token {
            Token::Equal => LogicOp::Equal,
            Token::NotEqual => LogicOp::NotEqual,
            Token::Greater => LogicOp::Greater,
            Token::GreaterEqual => LogicOp::GreaterEqual,
            Token::Less => LogicOp::Less,
            Token::LessEqual => LogicOp::LessEqual,
            Token::And => LogicOp::And,
            Token::Or => LogicOp::Or,
            _ => unreachable!("Binary operator not implemented."),
        };

        ComparisonOperation {
            operator,
            left,
            right,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Environment {
    vars: HashMap<Identifier, Expression>,
    functions: HashMap<Identifier, Expression>,
    parent: Option<Box<Environment>>,
}

impl Environment {
    pub fn new(parent: Option<Box<Environment>>) -> Self {
        Environment {
            vars: HashMap::new(),
            functions: HashMap::new(),
            parent,
        }
    }

    pub fn add_variable(&mut self, name: String, value: Expression) {
        self.vars.insert(name, value);
    }

    pub fn add_function(&mut self, name: String, def: Expression) {
        self.functions.insert(name, def);
    }

    pub fn find_variable(&self, name: &str) -> Result<&Expression, ErrorLang> {
        match self.vars.get(name) {
            Some(variable) => Ok(variable),
            None => match &self.parent {
                Some(env) => env.find_variable(name),
                None => Err(ErrorLang::VariableNotFound(name.to_string())),
            },
        }
    }

    pub fn find_function(&self, name: &str) -> Result<Function, ErrorLang> {
        match self.functions.get(name) {
            Some(function) => {
                if let Expression::Function(f) = function {
                    Ok(f.clone())
                } else {
                    Err(ErrorLang::FunctionNotFound(name.to_string()))
                }
            }
            None => match &self.parent {
                Some(env) => env.find_function(name),
                None => Err(ErrorLang::FunctionNotFound(name.to_string())),
            },
        }
    }

    pub fn create_child(self) -> Self {
        Environment::new(Some(Box::new(self)))
    }
}
