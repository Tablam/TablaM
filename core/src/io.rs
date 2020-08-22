use std::fs;
use std::io::prelude::*;
use std::io::{BufReader, SeekFrom};
use std::path::PathBuf;

use console::Term;
use derive_more::{Display, From};

use crate::for_impl::*;
use crate::prelude::*;

pub struct Console {
    _out: Term,
    _err: Term,
}

impl Console {
    pub fn new() -> Self {
        Console {
            _out: Term::buffered_stdout(),
            _err: Term::buffered_stderr(),
        }
    }

    pub fn write_line(&self, of: &str) -> Result<()> {
        self._out.write_line(of)?;
        Ok(())
    }

    pub fn write_str(&self, of: &str) -> Result<()> {
        self._out.write_str(of)?;
        Ok(())
    }
}

#[derive(Debug, Display, Derivative, From)]
#[display(fmt = "{:?}", path)]
#[derivative(Hash, PartialEq, PartialOrd, Ord, Eq)]
pub struct File {
    #[derivative(
        Hash = "ignore",
        PartialEq = "ignore",
        PartialOrd = "ignore",
        Ord = "ignore"
    )]
    f: fs::File,
    path: PathBuf,
    read: bool,
    write: bool,
    create: bool,
}

impl File {
    pub fn map_err<T>(of: std::io::Result<T>, path: &PathBuf) -> Result<T> {
        of.map_err(|e| Error::file_err(e, path.clone()))
    }

    pub fn new(path: PathBuf, read: bool, write: bool, create: bool) -> Result<Self> {
        let f = fs::OpenOptions::new()
            .read(read)
            .write(write)
            .create(create)
            .open(&path);

        let f = Self::map_err(f, &path)?;

        Ok(File {
            f,
            path,
            read,
            write,
            create,
        })
    }

    pub fn read_to_string(&mut self) -> Result<String> {
        let mut x = String::new();
        Self::map_err(self.f.read_to_string(&mut x), &self.path)?;
        Ok(x)
    }

    pub fn write_string(&mut self, content: &str) -> Result<()> {
        Self::map_err(self.f.write_all(content.as_bytes()), &self.path)?;
        Ok(())
    }

    pub fn seek_start(&mut self, pos: u64) -> Result<()> {
        Self::map_err(self.f.seek(SeekFrom::Start(pos)), &self.path)?;
        Ok(())
    }

    pub fn rows_iter(&self) -> impl Iterator<Item = Tuple> + '_ {
        let buffer = BufReader::new(&self.f);

        buffer
            .lines()
            .scan((), |_, x| x.ok())
            .map(|x| vec![x.into()])
    }
}

impl Rel for File {
    fn type_name(&self) -> &str {
        "File"
    }

    fn kind(&self) -> DataType {
        DataType::Vec(vec![DataType::UTF8].into())
    }

    fn schema(&self) -> Schema {
        schema_it(DataType::UTF8)
    }

    fn len(&self) -> usize {
        if let Ok(m) = self.f.metadata() {
            m.len() as usize
        } else {
            0
        }
    }

    fn cols(&self) -> usize {
        1
    }

    fn rows(&self) -> Option<usize> {
        None
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn rel_shape(&self) -> RelShape {
        RelShape::Vec
    }

    fn rel_hash(&self, mut hasher: &mut dyn Hasher) {
        self.hash(&mut hasher)
    }

    fn rel_eq(&self, other: &dyn Rel) -> bool {
        cmp_eq(self, other)
    }

    fn rel_cmp(&self, other: &dyn Rel) -> Ordering {
        cmp(self, other)
    }
}
