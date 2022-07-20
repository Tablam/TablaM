use crate::files::Files;
use crate::token::Syntax;

/// The points in the code where we can recover after a failed parse
const RECOVERY_SET: [Syntax; 2] = [Syntax::LetKw, Syntax::VarKw];

pub struct Parser {
    pub(crate) files: Files,
}

impl Parser {
    pub fn new(files: Files) -> Self {
        let root = files.get_root();

        Self { files }
    }
}
