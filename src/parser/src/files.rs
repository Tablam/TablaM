use corelib::prelude::FileId;
use std::path::PathBuf;

use corelib::tree_flat::prelude::{Node, NodeMut, Tree};

use crate::token::Token;

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
}

#[derive(Debug)]
pub struct Files {
    files: Tree<File>,
}

impl Files {
    pub fn new(root: File) -> Self {
        Files {
            files: Tree::new(root),
        }
    }

    pub fn from_src(source: &str) -> Self {
        let f = File::new(source);
        Self::new(f)
    }

    fn _add(&mut self, p: Option<PathBuf>, source: &str) -> FileId {
        let mut root = self.files.root_mut();
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

    pub fn get_root_mut(&mut self) -> NodeMut<'_, File> {
        self.files.root_mut()
    }

    pub fn get_root(&self) -> Node<'_, File> {
        self.files.root()
    }

    pub fn source(&self, token: &Token) -> &str {
        let f = self.get(token.file_id).unwrap();
        &f.source[token.range]
    }
}
