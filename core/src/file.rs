use std::fs;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

use crate::for_impl::*;
use crate::prelude::*;

use crate::derivative::Derivative;
use derive_more::{Display, From};

#[derive(Debug, Display, Derivative, From)]
#[display(fmt = "File([{}], {:?})", schema, path)]
#[derivative(Hash, PartialEq, PartialOrd, Ord, Eq)]
pub struct File {
    #[derivative(
        Hash = "ignore",
        PartialEq = "ignore",
        PartialOrd = "ignore",
        Ord = "ignore"
    )]
    f: fs::File,
    schema: Schema,
    path: PathBuf,
    read: bool,
    write: bool,
    create: bool,
}

impl File {
    pub fn map_err<T>(of: std::io::Result<T>, path: PathBuf) -> ResultT<T> {
        of.map_err(|e| Error::file_err(e, path))
    }

    pub fn new(path: PathBuf, read: bool, write: bool, create: bool) -> ResultT<Self> {
        let f = fs::OpenOptions::new()
            .read(read)
            .write(write)
            .create(create)
            .open(&path);

        let f = Self::map_err(f, path.clone())?;
        let schema = if let Some(header) = BufReader::new(&f).lines().next() {
            match header {
                Ok(header) => {
                    let mut fields = Vec::new();
                    for f in header.split(',') {
                        fields.push(Field::new(f, DataType::Utf8));
                    }
                    Schema::new(fields, None)
                }
                Err(e) => return Err(Error::file_err(e, path)),
            }
        } else {
            Schema::new_single("line", DataType::Utf8)
        };
        let mut f = File {
            f,
            schema,
            path,
            read,
            write,
            create,
        };
        f.seek_start(0)?;
        Ok(f)
    }

    pub fn read_to_string(&mut self) -> ResultT<String> {
        let mut x = String::new();
        Self::map_err(self.f.read_to_string(&mut x), self.path.clone())?;
        Ok(x)
    }

    pub fn write_string(&mut self, content: &str) -> ResultT<()> {
        Self::map_err(self.f.write_all(content.as_bytes()), self.path.clone())?;
        Ok(())
    }

    pub fn seek_start(&mut self, pos: u64) -> ResultT<()> {
        Self::map_err(self.f.seek(SeekFrom::Start(pos)), self.path.clone())?;
        Ok(())
    }

    pub fn rows_iter(&self) -> impl Iterator<Item = Tuple> + '_ {
        let buffer = BufReader::new(&self.f);

        buffer
            .lines()
            .skip(1)
            .scan((), |_, x| x.ok())
            .map(|x| x.split(',').map(Scalar::from).collect())
    }
}

impl Rel for File {
    fn type_name(&self) -> &str {
        "File"
    }

    fn kind(&self) -> DataType {
        DataType::File
    }

    fn schema(&self) -> Schema {
        self.schema.clone()
    }

    fn len(&self) -> usize {
        if let Ok(m) = self.f.metadata() {
            m.len() as usize
        } else {
            0
        }
    }

    fn size(&self) -> ShapeLen {
        ShapeLen::Iter(1, None)
    }

    fn as_any(&self) -> &dyn Any {
        self
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

    fn iter(&self) -> Box<dyn Iterator<Item = &Scalar>> {
        todo!()
    }

    fn col(&self, _pos: usize) -> Col<'_> {
        todo!()
    }

    fn rows(&self) -> Box<dyn Iterator<Item = Row<'_>>> {
        todo!()
    }

    fn from_query(_of: QueryResult<'_>) -> Self
    where
        Self: Sized,
    {
        todo!()
    }

    fn from_joins(_of: QueryResultOwned<'_>) -> Self
    where
        Self: Sized,
    {
        todo!()
    }
}

impl Clone for File {
    fn clone(&self) -> Self {
        //This never fail, according to impl of file
        let f = self.f.try_clone().expect("Fail to clone");
        File {
            f,
            schema: self.schema.clone(),
            path: self.path.clone(),
            read: self.read,
            write: self.write,
            create: self.create,
        }
    }
}

pub fn read_file_to_string(f: &mut fs::File) -> ResultT<String> {
    let mut x = String::new();
    f.read_to_string(&mut x)?;
    Ok(x)
}
