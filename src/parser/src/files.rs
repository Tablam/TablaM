use corelib::prelude::FileId;
use std::path::PathBuf;

use corelib::tree_flat::iter::TreeIter;
use corelib::tree_flat::prelude::Tree;

use crate::token::Token;

pub const REPL_FILE_NAME: &str = "repl";

/// The main container for a source "File"
#[derive(Debug)]
pub struct File {
    source: String,
    path: Option<PathBuf>,
}

impl File {
    pub fn new(source: &str) -> Self {
        File {
            source: source.to_string(),
            path: None,
        }
    }

    pub fn from_path(p: PathBuf, source: &str) -> Self {
        File {
            source: source.to_string(),
            path: Some(p),
        }
    }

    pub fn source(&self) -> &str {
        self.source.as_str()
    }

    pub fn name(&self) -> String {
        self.path
            .as_ref()
            .and_then(|x| x.to_str())
            .map(String::from)
            .unwrap_or_else(|| REPL_FILE_NAME.to_string())
    }

    pub fn append(&mut self, src: &str) {
        self.source += src;
    }
}

#[derive(Debug)]
pub struct FilesDb {
    files: Tree<File>,
}

impl FilesDb {
    pub fn new(root: File) -> Self {
        FilesDb {
            files: Tree::new(root),
        }
    }

    pub fn from_src(source: &str) -> Self {
        let f = File::new(source);
        Self::new(f)
    }

    fn _add(&mut self, p: Option<PathBuf>, source: &str) -> FileId {
        let mut root = self.files.tree_root_mut();
        let f = if let Some(f) = p {
            root.push(File::from_path(f, source))
        } else {
            root.push(File::new(source))
        };

        f.parent
    }

    pub fn add_direct(&mut self, source: &str) -> FileId {
        self._add(None, source)
    }

    pub fn add_file(&mut self, p: PathBuf, source: &str) -> FileId {
        self._add(Some(p), source)
    }

    pub fn get(&self, idx: FileId) -> Option<&File> {
        self.files.node(idx).map(|x| x.data)
    }

    pub fn get_root_mut(&mut self) -> &mut File {
        self.files.root_mut().data
    }

    pub fn get_root(&self) -> &File {
        self.files.root().data
    }

    pub fn files(&self) -> TreeIter<'_, File> {
        self.files.iter()
    }

    pub fn source(&self, token: &Token) -> &str {
        let f = self.get(token.file_id).unwrap();
        &f.source[token.range]
    }
}
