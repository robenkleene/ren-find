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
        paths: &Vec<PathBuf>,
    ) -> Result<IndexMap<PathBuf, PathBuf>, Error> {
        let mut src_to_dst = IndexMap::new();
        for path in paths {
            let dst = match self.replace_path(&path) {
              Ok(result) => result,
              Err(err) => return Err(Error::ReplaceError(err)),
            };
            src_to_dst.insert(path.clone(), dst);
        }
        return Ok(src_to_dst);
    }

    fn replace_path(&self, path: &PathBuf) -> Result<PathBuf, Utf8Error> {
        // `path.file_name()` removes any trailing slash
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

#[cfg(test)]
mod tests {
    use super::*;

    fn parse<'a>(
        look_for: impl Into<String>,
        replace_with: impl Into<String>,
        paths: &Vec<PathBuf>,
        str_to_dst: IndexMap<PathBuf, PathBuf>,
    ) {
        let replacer = Replacer::new(
            look_for.into(),
            replace_with.into(),
            false,
            None,
            None,
        ).unwrap();
        let edit = Edit::new(&replacer);
        let parsed = edit.parse(paths).unwrap();
        assert_eq!(
            parsed,
            str_to_dst
        );
    }

    fn replace_path<'a>(
        look_for: impl Into<String>,
        replace_with: impl Into<String>,
        src: &PathBuf,
        dst: &PathBuf,
    ) {
        let replacer = Replacer::new(
            look_for.into(),
            replace_with.into(),
            false,
            None,
            None,
        ).unwrap();
        let edit = Edit::new(&replacer);
        let replaced = edit.replace_path(src).unwrap();
        assert_eq!(
            &replaced,
            dst
        );
    }

    #[test]
    fn dirs_replace() {
        let expected = IndexMap::from([
            (PathBuf::from("changes/"), PathBuf::from("altered/")),
            (PathBuf::from("changes/stays"), PathBuf::from("altered/stays")),
            (PathBuf::from("stays/"), PathBuf::from("stays/")),
        ]);
        let paths: Vec<PathBuf> = ["changes/", "changes/stays", "stays/"].iter().map(|a| PathBuf::from(a)).collect();
        parse("changes", "altered", &paths, expected);
    }

    #[test]
    fn replace_path_slashes() {
        replace_path("changes", "altered", &PathBuf::from("stays/"), &PathBuf::from("stays"))
    }
}
