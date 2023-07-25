use crate::replacer::Replacer;
use std::str::Utf8Error;
use indexmap::IndexMap;
use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Replace failed")]
    ReplaceError(Utf8Error),
}

pub(crate) struct Edit<'a> {
    replacer: &'a Replacer,
}

impl<'a> Edit<'a> {
    pub(crate) fn new(replacer: &'a Replacer) -> Self {
        Self { replacer }
    }

    pub(crate) fn parse(
        self,
        paths: Vec<PathBuf>,
    ) -> Result<IndexMap<PathBuf, PathBuf>, Error> {
        let mut src_to_dst = IndexMap::new();
        for path in paths {
            let dst = match self.replace_path(&path) {
              Ok(result) => result,
              Err(err) => return Err(Error::ReplaceError(err)),
            };
            src_to_dst.insert(path, dst);
        }
        return Ok(src_to_dst);
    }

    fn replace_path(&self, path: &PathBuf) -> Result<PathBuf, Utf8Error> {
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
