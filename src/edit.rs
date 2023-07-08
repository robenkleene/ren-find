use std::path::PathBuf;
use std::io::StdinLock;
use std::io::prelude::*;

#[derive(Debug)]
pub(crate) struct Edit {
    pub(crate) file: PathBuf,
    pub(crate) text: String,
    pub(crate) number: u32
}

#[derive(Debug)]
#[derive(thiserror::Error)]
pub enum Error {
  #[error("No file, number, and text matches")]
  Match,
}

impl Edit {
    pub(crate) fn new(file: PathBuf, text: String, number: u32) -> Edit {
        Edit { file, text, number }
    }

    pub(crate) fn parse (
        reader: StdinLock<'_>
    ) -> Result<Vec<PathBuf>, std::io::Error> {
        let mut paths = Vec::new();
        for line in reader.lines() {
            let path = PathBuf::from(line?);
            paths.push(path);
        }
        return Ok(paths);
    }

}

