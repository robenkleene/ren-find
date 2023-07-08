use std::path::PathBuf;
use std::io::StdinLock;
use std::io::prelude::*;

#[derive(Debug)]
#[derive(thiserror::Error)]
pub enum Error {
  #[error("No file, number, and text matches")]
  Match,
}

pub(crate) struct Edit { }

impl Edit {
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

