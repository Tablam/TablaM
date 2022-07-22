use crate::token::{BinaryOp, CmpOp, UnaryOp};
use corelib::types::DataType;
use std::cmp::min;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Status {
    Finished,
    Continue,
    Error(Step),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Keyword {
    Let,
    Var,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Step {
    Bool,
    I64,
    Expr,
    ExprIncomplete,
    Ident,
    Kw(Keyword),
    Assign,
    UnaryOp(UnaryOp),
    BinOP(BinaryOp),
    CmpOp(CmpOp),
}

impl Step {
    fn is_replaceable(&self) -> bool {
        matches!(self, Step::Expr) && self != &Step::ExprIncomplete
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Task {
    Start,
    Scalar(DataType),
    Expr,
    UnaryOp(UnaryOp),
    BinOp(BinaryOp),
    DefVar,
}

impl Task {
    fn steps(&self) -> Vec<Step> {
        match self {
            Task::Start => vec![],
            Task::Scalar(_) => vec![Step::Expr],
            Task::Expr => vec![Step::Expr],
            Task::DefVar => vec![
                Step::Kw(Keyword::Var),
                Step::Ident,
                Step::Assign,
                Step::Expr,
            ],
            Task::UnaryOp(op) => vec![Step::UnaryOp(*op), Step::Expr, Step::Expr],
            Task::BinOp(op) => vec![Step::BinOP(*op), Step::Expr, Step::Expr],
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct CheckList {
    pub task: Task,
    pub pos: usize,
    pub steps: Vec<Step>,
    found: Option<Step>,
}

impl CheckList {
    pub fn new(task: Task) -> Self {
        Self {
            steps: task.steps(),
            task,
            pos: 0,
            found: None,
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

    pub fn check(&mut self, step: Step) -> Status {
        if self.is_done() {
            return Status::Finished;
        }

        self.found = Some(step);
        let actual = self.steps[self.pos];
        let actual = if actual.is_replaceable() {
            self.steps[self.pos] = step;
            step
        } else {
            actual
        };

        if actual == step {
            self.pos += 1;

            if self.is_done() {
                self.found = None;
                Status::Finished
            } else {
                Status::Continue
            }
        } else {
            Status::Error(step)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_var() {
        // Check: var x := y
        let mut checklist = CheckList::new(Task::DefVar);
        assert!(!checklist.is_done());
        assert_eq!(checklist.check(Step::Kw(Keyword::Var)), Status::Continue);
        assert_eq!(checklist.check(Step::Ident), Status::Continue);
        assert_eq!(checklist.check(Step::Assign), Status::Continue);
        assert_eq!(checklist.check(Step::Expr), Status::Finished);
        assert!(checklist.is_done());
    }

    #[test]
    fn check_var_error() {
        // Check: var x y
        let mut checklist = CheckList::new(Task::DefVar);
        assert_eq!(checklist.check(Step::Kw(Keyword::Var)), Status::Continue);

        assert_eq!(checklist.check(Step::Expr), Status::Error(Step::Expr));

        assert_eq!(checklist.done(), &[Step::Kw(Keyword::Var)]);
        assert_eq!(
            checklist.pending(),
            &[Step::Ident, Step::Assign, Step::Expr]
        );
    }
}
