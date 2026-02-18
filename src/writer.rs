use crate::EditKind;
use diffy_fork_filenames::{create_patch, PatchFormatter};
use indexmap::IndexMap;
use std::{fs, path::{Path, PathBuf}};

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub(crate) struct Writer {
    paths: Vec<PathBuf>,
    src_to_dst: Option<IndexMap<PathBuf, PathBuf>>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    File(#[from] std::io::Error),
    #[error(transparent)]
    String(#[from] std::str::Utf8Error),
    #[error("failed to move file: {0}")]
    TempfilePersist(#[from] tempfile::PersistError),
    #[error("missing source to destination mapping for replace operation")]
    MissingMapping,
    #[error("some operations failed")]
    PartialFailure,
}

impl Writer {
    pub(crate) fn new(paths: Vec<PathBuf>, src_to_dst: Option<IndexMap<PathBuf, PathBuf>>) -> Self {
        Self { paths, src_to_dst }
    }

    fn src_to_dst(&self) -> Result<&IndexMap<PathBuf, PathBuf>> {
        self.src_to_dst.as_ref().ok_or(Error::MissingMapping)
    }

    fn should_skip(src: &Path, dst: &Path) -> bool {
        src == dst || !Self::check(src, dst)
    }

    pub(crate) fn patch_preview(&self, color: bool, delete_kind: EditKind) -> Result<String, crate::writer::Error> {
        let mut modified_paths: Vec<String> = Vec::new();
        let mut print_diff = false;
        let mut modified = String::new();
        let original: String = self
            .paths
            .iter()
            .map(|p| p.to_string_lossy())
            .collect::<Vec<_>>()
            .join("\n") + "\n";
        if let EditKind::Replace = delete_kind {
            let src_to_dst = self.src_to_dst()?;
            for path in &self.paths {
                let dst = &src_to_dst[path];
                if Self::should_skip(path, dst) {
                    let path_string = path.to_string_lossy();
                    modified_paths.push(path_string.to_string());
                    continue;
                }
                print_diff = true;
                modified_paths.push(dst.to_string_lossy().to_string());
            }
            if !print_diff {
                return Ok(String::new());
            }
            modified = modified_paths.join("\n") + "\n";
        }
        let patch = create_patch(&original, &modified);
        // The new line added at the end of diff output appears to come from the `PatchFormatter`,
        // i.e., in the way it interprets the `Hunks` owned by the `Patch`
        // You can't remove the line endings from `original` and `modified` because that will
        // produce the `No new line at end of file` messages in the diffs
        let f = match color {
            true => PatchFormatter::new().with_color(),
            false => PatchFormatter::new(),
        };
        let result = f.fmt_patch(&patch).to_string();
        Ok(result)
    }

    pub(crate) fn write_file(&self, delete_kind: EditKind) -> Result<()> {
        let mut had_error = false;
        for path in &self.paths {
            match delete_kind {
                EditKind::Delete => {
                    if let Err(err) = Self::delete_path(path, false) {
                        eprintln!("Error: failed to remove '{}': {}", path.display(), err);
                        had_error = true;
                    }
                }
                EditKind::DeleteAll => {
                    if let Err(err) = Self::delete_path(path, true) {
                        eprintln!("Error: failed to remove '{}': {}", path.display(), err);
                        had_error = true;
                    }
                }
                EditKind::Replace => {
                    let src_to_dst = self.src_to_dst()?;
                    let dst = &src_to_dst[path];
                    if Self::should_skip(path, dst) {
                        continue;
                    }
                    if let Err(err) = fs::rename(path, dst) {
                        if err.raw_os_error() == Some(libc::EXDEV) {
                            eprintln!(
                                "Error: cannot move '{}' to '{}': source and destination are on different filesystems",
                                path.display(),
                                dst.display(),
                            );
                        } else {
                            eprintln!(
                                "Error: failed to move '{}' to '{}', underlying error: {}",
                                path.display(),
                                dst.display(),
                                err
                            );
                        }
                        had_error = true;
                    }
                }
            };
        }
        if had_error {
            return Err(Error::PartialFailure);
        }
        Ok(())
    }

    fn delete_path(path: &Path, recursive: bool) -> std::io::Result<()> {
        let meta = fs::symlink_metadata(path)?;
        if meta.is_symlink() {
            fs::remove_file(path)
        } else if meta.is_dir() {
            if recursive {
                fs::remove_dir_all(path)
            } else {
                fs::remove_dir(path)
            }
        } else {
            fs::remove_file(path)
        }
    }

    fn check(src: &Path, dst: &Path) -> bool {
        if fs::symlink_metadata(src).is_err() {
            eprintln!("Skipping {} because it doesn't exist", src.display());
            return false;
        }
        if fs::symlink_metadata(dst).is_ok() {
            eprintln!(
                "Skipping {} because {} already exists",
                src.display(),
                dst.display()
            );
            return false;
        }
        true
    }
}
