use crate::ast::{Ast, ExprBool};
use crate::checklist::{Kw, Step, Task};
use crate::cst::CstNode;
use crate::errors;
use crate::errors::{not_a_expr, ErrorParser};
use crate::parser::Checker;
use crate::token::{Syntax, Token};
use corelib::prelude::{Decimal, Scalar, F64};
use corelib::scalar::{BitVec, DateKind};
use corelib::tree_flat::node::NodeId;
use corelib::types;
use corelib::types::DataType;

pub(crate) fn root(p: &mut Checker) {
    let mut parent = p.cst.ast.root().id;

    loop {
        parent = match check(p, parent) {
            Ok(id) => id,
            Err(_id) => {
                //p.eat_to_recover(parent);
                parent
            }
        };

        if p.at_end() {
            break;
        } else {
            //It has a pending task unfinished?
            p.check_pending();
            if !p.at_end() {
                let next = p.next();
                p.new_task_span(Task::Start, next.span(&p.cst.tokens));
            }
        }
    }

    //It has a pending task unfinished?
    p.check_pending();
}

/// The main interface that run the parser with a [CheckList]
/// and report the errors
fn check(p: &mut Checker, parent: NodeId) -> Result<NodeId, NodeId> {
    let next = p.next();
    if let CstNode::Eof(_) = next {
        p.check.advance();
        p.advance_eof();
        return Ok(parent);
    }
    if let CstNode::Err(_) = next {
        p.recover();
    }

    match &p.check.task {
        Task::Start => {
            if let CstNode::Atom(t) = next {
                p.new_task(Task::Expr, t);
            }
            if let CstNode::Op(t) = next {
                p.new_task(Task::Expr, t);
            }
            if let CstNode::If(t) = next {
                p.new_task(Task::IfExpr, t);
            }
            //If the Task is still Start then we found a syntax error or unfinished parsing logic
            return if p.check.task == Task::Start {
                //if p.peek()
                let _ = p.check.check(&next, Step::Expr, next.span(&p.cst.tokens));
                Err(parent)
            } else {
                check(p, parent)
            };
        }
        Task::Expr => {
            if let CstNode::Atom(_) = &next {
                let of = parse_scalar(p, parent, &next);
                return p.push_or_err(of, parent);
            }
        }
        Task::IfExpr => {
            if let CstNode::If(_) = &next {
                let of = parse_if(p, parent, &next);
                return p.push_or_err(of, parent);
            }
        }
        x => unimplemented!("{:?}", x),
    }

    Err(parent)
}

fn _parse_scalar<T>(code: &str, kind: DataType, t: &Token) -> Result<T, ErrorParser>
where
    T: Into<Scalar> + std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    match code.parse::<T>() {
        Ok(x) => Ok(x),
        Err(x) => Err(errors::parse(t, kind, &x.to_string())),
    }
}

fn parse_bool(code: &str, t: &Token) -> Result<(Ast, Step), ErrorParser> {
    let x = _parse_scalar::<bool>(code, DataType::Bool, t)?;
    Ok((Ast::scalar(x.into(), t), Step::Bool))
}

fn clean_num(code: &str) -> String {
    code.replace('_', "")
}

fn clean_quotes(code: &str) -> &str {
    code.trim_start_matches(|x| x == '\'' || x == '"')
        .trim_end_matches(|x| x == '\'' || x == '"')
}

fn clean_prefix<'a>(code: &'a str, prefix: &str) -> &'a str {
    code.trim_start_matches(prefix)
}

fn clean_dates<'a>(code: &'a str, prefix: &str) -> &'a str {
    clean_quotes(clean_prefix(code, prefix))
}

fn clean_str(code: &str) -> String {
    format!("\"{}\"", clean_quotes(code))
}

fn clean_floats(code: &str) -> String {
    if code.ends_with('d') || code.ends_with('f') {
        clean_num(&code[..code.len() - 1])
    } else {
        clean_num(code)
    }
}

fn parse_bit(code: &str, t: &Token) -> Result<(Ast, Step), ErrorParser> {
    let mut bits = BitVec::with_capacity(code.len() - 1);

    for (pos, x) in code[..code.len() - 1].chars().enumerate() {
        match x {
            '0' => bits.push(false),
            '1' => bits.push(true),
            '_' => continue,
            x => {
                return Err(errors::parse(
                    t,
                    DataType::Bit,
                    &format!("Invalid bit char `{x}` at pos {pos}. Must be 1 or 0."),
                ))
            }
        }
    }

    Ok((Ast::scalar(Scalar::Bit(bits), t), Step::Bit))
}

fn parse_i64(code: &str, t: &Token) -> Result<(Ast, Step), ErrorParser> {
    let x = _parse_scalar::<i64>(&clean_num(code), DataType::I64, t)?;
    Ok((Ast::scalar(x.into(), t), Step::I64))
}

fn parse_d64(code: &str, t: &Token) -> Result<(Ast, Step), ErrorParser> {
    let x = _parse_scalar::<Decimal>(&clean_floats(code), DataType::Decimal, t)?;
    Ok((Ast::scalar(x.into(), t), Step::Dec))
}

fn parse_f64(code: &str, t: &Token) -> Result<(Ast, Step), ErrorParser> {
    let x = _parse_scalar::<F64>(&clean_floats(code), DataType::F64, t)?;
    Ok((Ast::scalar(x.into(), t), Step::Dec))
}

fn parse_str(code: &str, t: &Token) -> Result<(Ast, Step), ErrorParser> {
    let x = _parse_scalar::<String>(&clean_str(code), DataType::Utf8, t)?;
    Ok((Ast::scalar(x.into(), t), Step::Str))
}

fn parse_date(code: &str, t: &Token) -> Result<(Ast, Step), ErrorParser> {
    let x = clean_dates(code, "d");
    let d = types::parse_date_t(x).map_err(|x| ErrorParser::ScalarParse {
        kind: DataType::Date(DateKind::Date),
        span: t.into(),
        msg: x.to_string(),
    })?;
    Ok((Ast::scalar(d.into(), t), Step::Date))
}

fn parse_time(code: &str, t: &Token) -> Result<(Ast, Step), ErrorParser> {
    let x = clean_dates(code, "t");
    let d = types::parse_time_t(x).map_err(|x| ErrorParser::ScalarParse {
        kind: DataType::Date(DateKind::Time),
        span: t.into(),
        msg: x.to_string(),
    })?;
    Ok((Ast::scalar(d.into(), t), Step::Date))
}

fn parse_datetime(code: &str, t: &Token) -> Result<(Ast, Step), ErrorParser> {
    let x = clean_dates(code, "dt");
    let d = types::parse_date_time_t(x).map_err(|x| ErrorParser::ScalarParse {
        kind: DataType::Date(DateKind::DateTime),
        span: t.into(),
        msg: x.to_string(),
    })?;
    Ok((Ast::scalar(d.into(), t), Step::Date))
}

pub(crate) fn parse_scalar(
    p: &mut Checker,
    _parent: NodeId,
    node: &CstNode,
) -> Result<Ast, ErrorParser> {
    let t = p.token(node.token_id());
    let span = t.into();
    let code = p.code(t);

    let (ast, step) = match t.kind {
        Syntax::Bool => parse_bool(code, t)?,
        Syntax::Integer => parse_i64(code, t)?,
        Syntax::Decimal => parse_d64(code, t)?,
        Syntax::Float => parse_f64(code, t)?,
        Syntax::Bit => parse_bit(code, t)?,
        Syntax::String => parse_str(code, t)?,
        Syntax::Date => parse_date(code, t)?,
        Syntax::Time => parse_time(code, t)?,
        Syntax::DateTime => parse_datetime(code, t)?,
        x => unimplemented!("{:?}", x),
    };
    p.check.check(node, step, span)?;
    Ok(ast)
}

/// A valid boolean expression is one of:
/// true | false
/// A [CmpOp] between expressions
/// A function that return a boolean expression
pub(crate) fn parse_bool_expr(p: &mut Checker, node: &CstNode) -> Result<ExprBool, ErrorParser> {
    let t = p.token(node.token_id());
    let code = p.code(t);

    if let CstNode::Atom(_) = node {
        if t.kind == Syntax::Bool {
            let x = _parse_scalar::<bool>(code, DataType::Bool, t)?;
            return Ok(ExprBool::bool(x, t));
        }
    }

    Err(errors::bool_expr(t, code))
}

pub(crate) fn expr(p: &mut Checker, parent: NodeId, next: CstNode) -> Result<Ast, ErrorParser> {
    match next {
        CstNode::Atom(_) => parse_scalar(p, parent, &next),
        _ => {
            let t = p.token(next.token_id());
            let code = p.code(t);
            Err(not_a_expr(t, code))
        }
    }
}

pub(crate) fn parse_if(
    p: &mut Checker,
    parent: NodeId,
    node: &CstNode,
) -> Result<Ast, ErrorParser> {
    let t = *p.token(node.token_id());
    // Eat "if"
    assert_eq!(t.kind, Syntax::IfKw);
    let if_span = (&t).into();
    p.check.check(node, Step::Kw(Kw::If), if_span)?;

    let next = p.advance_and_next();
    let check = parse_bool_expr(p, &next)?;
    p.check.check(&next, Step::Expr, check.span())?;

    let next = p.advance_and_next();
    let do_span = next.span(&p.cst.tokens);
    p.check.check(&next, Step::Kw(Kw::Do), do_span)?;

    let next = p.advance_and_next();
    let if_true = expr(p, parent, next)?;

    let next = p.advance_and_next();
    let else_span = next.span(&p.cst.tokens);
    p.check.check(&next, Step::Kw(Kw::Else), else_span)?;

    let next = p.advance_and_next();
    let if_false = expr(p, parent, next)?;

    let next = p.advance_and_next();
    let end_span = next.span(&p.cst.tokens);
    p.check.check(&next, Step::Kw(Kw::End), end_span)?;

    Ok(Ast::IfBlock {
        if_span,
        do_span,
        else_span,
        end_span,
        check: Box::new(check),
        if_true: Box::new(if_true),
        if_false: Box::new(if_false),
    })
}
