use crate::ast::{Ast, Node};
use crate::checklist::{CheckList, Step, Task};
use crate::cst::{src_to_cst, Cst, CstNode};
use crate::files::Files;
use crate::token::{Syntax, Token};
use tree_flat::prelude::Tree;

/// The points in the code where we can recover after a failed parse
const RECOVERY_SET: [Syntax; 2] = [Syntax::LetKw, Syntax::VarKw];

struct Checker<'a> {
    check: CheckList,
    cst: &'a [CstNode],
    ast: Vec<Ast>,
    pos: usize,
    errors: Vec<usize>,
}

impl<'a> Checker<'a> {
    pub fn new(cst: &'a [CstNode]) -> Self {
        Self {
            check: CheckList::new(Task::Start),
            cst,
            ast: Vec::with_capacity(cst.len()),
            pos: 0,
            errors: vec![],
        }
    }

    fn at_end(&self) -> bool {
        //NOTE: The last token is always EOF!
        !(self.cursor < self.ust.len())
    }

    fn new_task(&mut self, task: Task) {
        //Check if we finish the token stream
        if !self.at_end() {
            self.check = CheckList::new(task);
        }
    }

    fn parse_trivia(&mut self, x: &Token) {}
    fn parse_scalar(&mut self, x: &Token) {}
    fn verify(&mut self) {
        let current = &self.cst[self.pos];

        while !self.at_end() {
            dbg!("RUN", p.cursor);

            match current {
                CstNode::Root => continue,
                CstNode::Trivia(x) => {
                    self.parse_trivia(x);
                }
                CstNode::Atom(x) => {
                    self.parse_scalar(x);
                }
                CstNode::Op(_) => {}
                CstNode::Err(err) => {}
                CstNode::Eof => break,
            }

            self.new_task(Task::Start);
        }
    }

    fn next(&mut self) {}
}

pub struct Parser {
    pub(crate) files: Files,
}

impl Parser {
    pub fn new(files: Files) -> Self {
        Self { files }
    }

    pub fn from_src(code: &str) -> Self {
        let files = Files::from_src(code);
        Self::new(files)
    }

    fn check(&self, cst: Cst) {
        let check = Checker::new(cst.ast.as_data());
    }

    pub fn parse(&self) {
        let root = self.files.get_root();
        let cst = src_to_cst(root.data.source());

        self.check(cst)
    }
}
