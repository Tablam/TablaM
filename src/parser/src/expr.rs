use crate::ast::{Ast, ExprBool};
use crate::checklist::{Kw, Step, Task};
use crate::cst::CstNode;
use crate::errors;
use crate::errors::{not_a_expr, ErrorParser};
use crate::parser::Checker;
use crate::token::{Syntax, Token};
use corelib::prelude::Scalar;
use corelib::tree_flat::node::NodeId;

pub(crate) fn root(p: &mut Checker) {
    let parent = p.cst.ast.root().id;

    loop {
        check(p, parent);

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
    dbg!("Checking", &next);
    if let CstNode::Eof(_) = next {
        p.check.advance();
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
            if p.check.task == Task::Start {
                //if p.peek()
                let _ = p.check.check(&next, Step::Expr, next.span(&p.cst.tokens));
                return Err(parent);
            } else {
                return check(p, parent);
            }
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

fn _parse_scalar<T>(code: &str, t: &Token) -> Result<T, ErrorParser>
where
    T: Into<Scalar> + std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    match code.parse::<T>() {
        Ok(x) => Ok(x),
        Err(x) => Err(errors::parse(t, &x.to_string())),
    }
}

fn parse_bool(code: &str, t: &Token) -> Result<(Ast, Step), ErrorParser> {
    let x = _parse_scalar::<bool>(code, t)?;
    Ok((Ast::scalar(x.into(), t), Step::Bool))
}

fn parse_i64(code: &str, t: &Token) -> Result<(Ast, Step), ErrorParser> {
    let x = _parse_scalar::<i64>(code, t)?;
    Ok((Ast::scalar(x.into(), t), Step::I64))
}

pub(crate) fn parse_scalar(
    p: &mut Checker,
    parent: NodeId,
    node: &CstNode,
) -> Result<Ast, ErrorParser> {
    let t = p.token(node.token_id());
    let span = t.into();
    let code = p.code(t);

    let (ast, step) = match t.kind {
        Syntax::Bool => parse_bool(code, t)?,
        Syntax::Integer => parse_i64(code, t)?,
        _ => unimplemented!(),
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
            let x = _parse_scalar::<bool>(code, t)?;
            return Ok(ExprBool::bool(x, t));
        }
    }

    Err(errors::bool_expr(t, code))
}

pub(crate) fn expr(p: &mut Checker, parent: NodeId, next: CstNode) -> Result<Ast, ErrorParser> {
    match next {
        CstNode::Atom(t) => parse_scalar(p, parent, &next),
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
    p.check.check(node, Step::Kw(Kw::If), (&t).into())?;
    let parent = p.push(Ast::If { span: (&t).into() }, parent);

    let next = p.advance_and_next();
    let bool_expr = parse_bool_expr(p, &next)?;
    p.check.check(&next, Step::Expr, (&t).into())?;
    let parent = p.push(Ast::BoolExpr(bool_expr), parent);

    let next = p.advance_and_next();
    p.check.check(&next, Step::Kw(Kw::Do), (&t).into())?;

    let next = p.advance_and_next();
    let ast = expr(p, parent, next)?;
    //p.check.check(&next, Step::Expr, (&t).into())?;

    let next = p.advance_and_next();
    p.check.check(&next, Step::Kw(Kw::Else), (&t).into())?;

    let next = p.advance_and_next();
    let ast = expr(p, parent, next)?;
    //p.check.check(&next, Step::Expr, (&t).into())?;

    let next = p.advance_and_next();
    p.check.check(&next, Step::Kw(Kw::End), (&t).into())?;

    Ok(Ast::Eof)
}
