use crate::replacer::Replacer;
use diffy_fork_filenames::{create_patch, PatchFormatter};
use std::{fs, path::PathBuf};

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub(crate) struct Writer<'a> {
    paths: Vec<PathBuf>,
    replacer: &'a Replacer,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
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
        let mut print_diff = false;
        for path in &self.paths {
            let dst = self.replace_path(path)?;
            if *path == dst || (*path != dst && !Self::check(&path.to_path_buf(), &dst)) {
                let path_string = path.to_string_lossy();
                modified_lines.push(path_string.to_string());
                continue;
            }
            print_diff = true;
            modified_lines.push(dst.to_string_lossy().to_string());
        }
        if !print_diff {
            return Ok("".to_string());
        }
        let modified = modified_lines.join("\n");
        let original: String = self
            .paths
            .clone()
            .into_iter()
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
            let dst = self.replace_path(path)?;
            if *path == dst || !Self::check(&path.to_path_buf(), &dst) {
                continue;
            }
            if let Err(err) = fs::rename(path, &dst) {
                eprintln!(
                    "Error: failed to move '{}' to '{}', underlying error: {}",
                    path.display(),
                    &dst.display(),
                    err
                );
            }
        }
        Ok(())
    }

    fn check(src: &PathBuf, dst: &PathBuf) -> bool {
        if !src.is_file() && !src.is_dir() {
            eprintln!("Skipping {} because it doesn't exist", src.display());
            return false;
        }
        if dst.is_file() || dst.is_dir() {
            eprintln!(
                "Skipping {} because {} already exists",
                src.display(),
                dst.display()
            );
            return false;
        }
        return true;
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
