use crate::replacer::Replacer;
use std::str::Utf8Error;
use indexmap::IndexMap;
use std::path::{Path, PathBuf};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Replace failed")]
    ReplaceError(Utf8Error),
    #[error("invalid path: {0}")]
    InvalidPath(PathBuf),
    #[error("replacement produces empty filename for '{0}'")]
    EmptyFilename(PathBuf),
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
        paths: &[PathBuf],
    ) -> Result<IndexMap<PathBuf, PathBuf>, Error> {
        let mut src_to_dst = IndexMap::new();
        for path in paths {
            let dst = self.replace_path(path)?;
            src_to_dst.insert(path.clone(), dst);
        }
        Ok(src_to_dst)
    }

    fn replace_path(&self, path: &Path) -> Result<PathBuf, Error> {
        // `path.file_name()` removes any trailing slash
        let filename = path.file_name()
            .ok_or_else(|| Error::InvalidPath(path.to_path_buf()))?;
        let filename_string = filename.to_string_lossy();
        let filename_bytes = filename_string.as_bytes();
        let filename_replaced = self.replacer.replace(filename_bytes);
        let filename_replaced_string = std::str::from_utf8(&filename_replaced)
            .map_err(|e| Error::ReplaceError(e))?;
        if filename_replaced_string.is_empty() {
            return Err(Error::EmptyFilename(path.to_path_buf()));
        }
        let filename_dir = path.parent()
            .ok_or_else(|| Error::InvalidPath(path.to_path_buf()))?;
        let mut dst_path = filename_dir.join(filename_replaced_string);
        // Add back the slash if the input had it
        if path.to_string_lossy().as_bytes().last() == Some(&b'/') {
            dst_path.push("");
        }
        Ok(PathBuf::from(dst_path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse<'a>(
        look_for: impl Into<String>,
        replace_with: impl Into<String>,
        paths: &[PathBuf],
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
        src: &Path,
        dst: &Path,
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
            (PathBuf::from("changes"), PathBuf::from("altered")),
            (PathBuf::from("changes/stays"), PathBuf::from("changes/stays")),
            (PathBuf::from("stays"), PathBuf::from("stays")),
        ]);
        let mut paths: Vec<PathBuf> = ["changes", "changes/stays", "stays"].iter().map(|a| PathBuf::from(a)).collect();
        // Input needs to be sorted
        paths.sort_by(|a, b| b.to_str().unwrap().len().cmp(&a.to_str().unwrap().len()));
        parse("changes", "altered", &paths, expected);
    }

    #[test]
    fn replace_path_slashes() {
        replace_path("changes", "altered", &PathBuf::from("stays/"), &PathBuf::from("stays"))
    }

    #[test]
    fn reject_empty_filename() {
        let replacer = Replacer::new(
            ".*".into(),
            "".into(),
            false,
            None,
            None,
        ).unwrap();
        let edit = Edit::new(&replacer);
        let result = edit.replace_path(Path::new("foo.txt"));
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, Error::EmptyFilename(_)));
    }
}
