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
    #[error("failed to move file: {0}")]
    TempfilePersist(#[from] tempfile::PersistError),
}

impl<'a> Writer<'a> {
    pub(crate) fn new(paths: Vec<PathBuf>, replacer: &'a Replacer) -> Self {
        Self { paths, replacer }
    }

    pub(crate) fn patch_preview(&self, color: bool) -> Result<String, crate::writer::Error> {
        let modified_lines = Vec::new();
        for path in self.paths {
            let replaced = self.replacer.replace(path.to_string_lossy().as_bytes());
            let result = std::str::from_utf8(&replaced)?;
            modified_lines.push(result);
        }

        let modified = modified_lines.join("\n");
        let original = self.paths.to_string_lossy().join("\n");
        let patch = create_patch(&original, &modified);
        let f = match color {
            true => PatchFormatter::new().with_color(),
            false => PatchFormatter::new(),
        };
        return Ok(f.fmt_patch(&patch).to_string());
    }

    pub(crate) fn write_file(&self) -> Result<()> {
        for path in self.paths {
            let replaced = self.replacer.replace(path.to_string_lossy().as_bytes());
            let result = std::str::from_utf8(&replaced);
            fs::rename(path, result)?;
        }
        Ok(())
    }
}
