use indexmap::IndexMap;
use std::io::prelude::*;
use std::io::StdinLock;
use std::path::PathBuf;

pub(crate) struct Edit {}

impl Edit {
    pub(crate) fn parse(
        reader: StdinLock<'_>,
    ) -> Result<IndexMap<PathBuf, PathBuf>, std::io::Error> {
        let mut path_to_path = IndexMap::new();
        for line in reader.lines() {
            let path = PathBuf::from(line?);
            path_to_path.insert(path, path);
        }
        return Ok(path_to_path);
    }
}
