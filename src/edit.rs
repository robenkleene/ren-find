use crate::replacer::Replacer;
use indexmap::IndexMap;
use std::io::prelude::*;
use std::io::StdinLock;
use std::path::PathBuf;

pub(crate) struct Edit<'a> {
    replacer: &'a Replacer,
}

impl<'a> Edit<'a> {
    pub(crate) fn new(replacer: &'a Replacer) -> Self {
        Self { replacer }
    }

    pub(crate) fn parse(
        reader: StdinLock<'_>,
    ) -> Result<IndexMap<PathBuf, PathBuf>, std::io::Error> {
        let mut src_to_dst = IndexMap::new();
        for line in reader.lines() {
            let path = PathBuf::from(line?);
            let dst = self.replace_path(path)?;
            src_to_dst.insert(path, path);
        }
        return Ok(src_to_dst);
    }

    fn replace_path(&self, path: &PathBuf) -> Result<PathBuf, Error> {
        let filename = path.file_name().unwrap();
        let filename_string = filename.to_string_lossy();
        let filename_bytes = filename_string.as_bytes();
        let filename_replaced = self.replacer.replace(filename_bytes);
        let filename_replaced_string = std::str::from_utf8(&filename_replaced)?;
        let filename_dir = path.parent().unwrap();
        let dst_path = filename_dir.join(filename_replaced_string);
        Ok(PathBuf::from(dst_path))
    }
}
