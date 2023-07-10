use crate::replacer::Replacer;
use std::{fs, path::PathBuf};
use diffy_fork_filenames::{create_patch, PatchFormatter};

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub(crate) struct Writer<'a> {
    paths: Vec<PathBuf>,
    replacer: &'a Replacer,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid path")]
    InvalidPath(std::path::PathBuf),
    #[error(transparent)]
    File(#[from] std::io::Error),
    #[error(transparent)]
    String(#[from] std::str::Utf8Error),
    #[error("failed to move file: {0}")]
    TempfilePersist(#[from] tempfile::PersistError),
}

impl<'a> Writer<'a> {
    pub(crate) fn new(paths: Vec<PathBuf>, replacer: &'a Replacer) -> Self {
        Self { paths, replacer }
    }

    pub(crate) fn patch_preview(&self, color: bool) -> Result<String, crate::writer::Error> {
        let mut modified_lines: Vec<String> = Vec::new();
        for path in &self.paths {
            let path_string = path.to_string_lossy();
            let path_bytes = path_string.as_bytes();
            let replaced = self.replacer.replace(path_bytes);
            let result = match std::str::from_utf8(&replaced) {
              Ok(result) => result,
              Err(err) => return Err(Error::String(err))
            };
            modified_lines.push(result.to_string());
        }

        let modified = modified_lines.join("\n");
        let original: String  = self.paths.clone().into_iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect::<Vec<String>>()
            .join("\n");
        let patch = create_patch(&original, &modified);
        let f = match color {
            true => PatchFormatter::new().with_color(),
            false => PatchFormatter::new(),
        };
        return Ok(f.fmt_patch(&patch).to_string());
    }

    pub(crate) fn write_file(&self) -> Result<()> {
        for path in &self.paths {
            let path_string = path.to_string_lossy();
            let path_bytes = path_string.as_bytes();
            let replaced = self.replacer.replace(path_bytes);
            let result = std::str::from_utf8(&replaced)?;
            fs::rename(path, result)?;
        }
        Ok(())
    }
}
