use crate::cst::CstNode;
use crate::token::{BinaryOp, CmpOp, UnaryOp};
use corelib::prelude::Span;
use corelib::types::DataType;
use std::cmp::min;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Finished,
    Continue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kw {
    Let,
    Var,
    If,
    Do,
    Else,
    End,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Step {
    Bool,
    I64,
    Dec,
    Expr,
    ExprIncomplete,
    Ident,
    Kw(Kw),
    Assign,
    UnaryOp(UnaryOp),
    BinOP(BinaryOp),
    CmpOp(CmpOp),
}

impl Step {
    fn is_replaceable(&self) -> bool {
        matches!(self, Step::Expr)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Task {
    Start,
    Scalar(DataType),
    Expr,
    IfExpr,
    CmpExpr,
    UnaryOp(UnaryOp),
    BinOp(BinaryOp),
    DefVar,
}

impl Task {
    fn steps(&self) -> Vec<Step> {
        match self {
            Task::Start => vec![Step::ExprIncomplete],
            Task::Scalar(_) => vec![Step::Expr],
            Task::Expr => vec![Step::Expr],
            Task::DefVar => vec![Step::Kw(Kw::Var), Step::Ident, Step::Assign, Step::Expr],
            Task::UnaryOp(op) => vec![Step::UnaryOp(*op), Step::Expr, Step::Expr],
            Task::BinOp(op) => vec![Step::BinOP(*op), Step::Expr, Step::Expr],
            Task::IfExpr => vec![
                Step::Kw(Kw::If),
                Step::Expr,
                Step::Kw(Kw::Do),
                Step::Expr,
                Step::Kw(Kw::Else),
                Step::Expr,
                Step::Kw(Kw::End),
            ],
            Task::CmpExpr => {
                vec![Step::CmpOp(CmpOp::Equals), Step::Expr, Step::Expr]
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CheckError {
    pub(crate) span: Span,
    pub(crate) found: CstNode,
    pub(crate) expect: Option<Step>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckList {
    pub task: Task,
    pub pos: usize,
    pub steps: Vec<Step>,
    pub span: Vec<Span>,
    pub expect: Option<Step>,
}

impl CheckList {
    pub fn new(task: Task, span: Span) -> Self {
        Self {
            steps: task.steps(),
            task,
            pos: 0,
            expect: None,
            span: vec![span],
        }
    }

    pub fn is_done(&self) -> bool {
        self.pos >= self.steps.len()
    }

    pub fn done(&self) -> &[Step] {
        let total = self.steps.len();
        &self.steps[0..min(self.pos, total)]
    }

    pub fn pending(&self) -> &[Step] {
        let total = self.steps.len();
        &self.steps[min(self.pos, total)..total]
    }

    pub fn span(&self) -> Span {
        let mut first = *self.span.first().unwrap();
        for s in self.span.iter().skip(1) {
            first.range.0 += s.range.0.len();
        }

        first
    }

    pub(crate) fn advance(&mut self) {
        self.pos += 1;
    }

    pub(crate) fn check(
        &mut self,
        node: &CstNode,
        step: Step,
        span: Span,
    ) -> Result<Status, CheckError> {
        if self.is_done() {
            return Err(CheckError {
                span,
                found: *node,
                expect: Some(step),
            });
        }

        self.expect = Some(step);
        let actual = self.steps[self.pos];
        let actual = if actual.is_replaceable() {
            self.steps[self.pos] = step;
            if self.span.len() < self.pos {
                self.span[self.pos] = span;
            } else {
                self.span.push(span);
            }
            step
        } else {
            self.span.push(span);
            actual
        };

        if actual == step {
            self.advance();

            if self.is_done() {
                self.expect = None;
                Ok(Status::Finished)
            } else {
                Ok(Status::Continue)
            }
        } else {
            Err(CheckError {
                span,
                found: *node,
                expect: Some(step),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::{token_test, TokenId};

    #[test]
    fn check_var() {
        // Check: var x := y
        let span = (&token_test()).into();
        let node = CstNode::Root(TokenId(0));
        let mut checklist = CheckList::new(Task::DefVar, span);
        assert!(!checklist.is_done());
        assert_eq!(
            checklist.check(&node, Step::Kw(Kw::Var), span),
            Ok(Status::Continue)
        );
        assert_eq!(
            checklist.check(&node, Step::Ident, span),
            Ok(Status::Continue)
        );
        assert_eq!(
            checklist.check(&node, Step::Assign, span),
            Ok(Status::Continue)
        );
        assert_eq!(
            checklist.check(&node, Step::Expr, span),
            Ok(Status::Finished)
        );
        assert!(checklist.is_done());
    }

    #[test]
    fn check_var_error() {
        // Check: var x y
        let span = (&token_test()).into();
        let node = CstNode::Root(TokenId(0));

        let mut checklist = CheckList::new(Task::DefVar, span);
        assert_eq!(
            checklist.check(&node, Step::Kw(Kw::Var), span),
            Ok(Status::Continue)
        );

        assert_eq!(
            checklist.check(&node, Step::Expr, span),
            Err(CheckError {
                span,
                found: node,
                expect: Some(Step::Expr)
            })
        );

        assert_eq!(checklist.done(), &[Step::Kw(Kw::Var)]);
        assert_eq!(
            checklist.pending(),
            &[Step::Ident, Step::Assign, Step::Expr]
        );
    }
}
