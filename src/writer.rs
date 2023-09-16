use crate::EditKind;
use diffy_fork_filenames::{create_patch, PatchFormatter};
use indexmap::IndexMap;
use std::{fs, path::PathBuf};

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
}

impl Writer {
    pub(crate) fn new(paths: Vec<PathBuf>, src_to_dst: Option<IndexMap<PathBuf, PathBuf>>) -> Self {
        Self { paths, src_to_dst }
    }

    pub(crate) fn patch_preview(&self, color: bool, delete_kind: EditKind) -> Result<String, crate::writer::Error> {
        let mut modified_paths: Vec<String> = Vec::new();
        let mut print_diff = false;
        let mut modified = "".to_string();
        let original: String = self
            .paths
            .clone()
            .into_iter()
            .fold(String::new(), |s, l| s + &l.to_string_lossy() + "\n");
        if let EditKind::Replace = delete_kind {
            let src_to_dst = match &self.src_to_dst {
              Some(src_to_dst) => src_to_dst,
              None => panic!("Missing source to destination"),
            };
            for path in &self.paths {
                let dst = &src_to_dst[path];
                if path == dst || (path != dst && !Self::check(&path.to_path_buf(), &dst)) {
                    let path_string = path.to_string_lossy();
                    modified_paths.push(path_string.to_string());
                    continue;
                }
                print_diff = true;
                modified_paths.push(dst.to_string_lossy().to_string());
            }
            if !print_diff {
                return Ok("".to_string());
            }
            modified = modified_paths.into_iter().fold(String::new(), |s, l| s + &l + "\n");
        }
        let patch = create_patch(&original, &modified);
        let f = match color {
            true => PatchFormatter::new().with_color(),
            false => PatchFormatter::new(),
        };
        return Ok(f.fmt_patch(&patch).to_string());
    }

    pub(crate) fn write_file(&self, delete_kind: EditKind) -> Result<()> {
        for path in &self.paths {
            match delete_kind {
                EditKind::Delete => {
                    if path.is_dir() {
                        if let Err(err) = fs::remove_dir(path) {
                            eprintln!(
                                "Error: failed to remove directory '{}': {}",
                                path.display(),
                                err
                            );
                        }
                    } else {
                        if let Err(err) = fs::remove_file(path) {
                            eprintln!(
                                "Error: failed to remove file '{}': {}",
                                path.display(),
                                err
                            );
                        }
                    }
                }
                EditKind::DeleteAll => {
                    if path.is_dir() {
                        if let Err(err) = fs::remove_dir_all(path) {
                            eprintln!(
                                "Error: failed to remove directory recursively '{}': {}",
                                path.display(),
                                err
                            );
                        }
                    } else {
                        if let Err(err) = fs::remove_file(path) {
                            eprintln!(
                                "Error: failed to remove file '{}': {}",
                                path.display(),
                                err
                            );
                        }
                    }
                }
                EditKind::Replace => {
                    let src_to_dst = match &self.src_to_dst {
                      Some(src_to_dst) => src_to_dst,
                      None => panic!("Missing source to destination"),
                    };
                    let dst = &src_to_dst[path];
                    if path == dst || !Self::check(&path.to_path_buf(), &dst) {
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
            };
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
}
