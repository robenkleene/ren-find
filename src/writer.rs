use crate::replacer::Replacer;
use std::{fs, fs::File, io::prelude::*, path::PathBuf};
use diffy_fork_filenames::PatchFormatter;

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub(crate) struct Writer<'a> {
    paths: Vec<PathBuf>,
    replacer: &'a Replacer<'a>,
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
        let original = paths.join("\n");
        let modified_lines = Vec::new();
        for path in paths {
            let replaced = self.replacer.replace(path);
            modified_lines.push(replaced);
        }
        let modified = modified_lines.join("\n");

        let patch = create_patch(&original, &modified);
        let f = match color {
            true => PatchFormatter::new().with_color(),
            false => PatchFormatter::new(),
        };
        return Ok(f.fmt_patch(&patch).to_string());
    }

    pub(crate) fn write_file(&self) -> Result<()> {
        use memmap::{Mmap, MmapMut};
        use std::ops::DerefMut;

        let source = File::open(self.path.clone())?;
        let meta = fs::metadata(self.path.clone())?;
        let mmap_source = unsafe { Mmap::map(&source)? };
        let lines = mmap_source.lines()
            .map(|l| l.expect("Error getting line"))
            .collect();
        let replaced = match self.patcher.patch(lines) {
            Ok(replaced) => replaced,
            Err(_) => panic!("Error patching lines"), // FIXME:
        };

        let target = tempfile::NamedTempFile::new_in(
            self.path.parent()
                .ok_or_else(|| Error::InvalidPath(self.path.to_path_buf()))?,
        )?;
        let file = target.as_file();
        file.set_len(replaced.len() as u64)?;
        file.set_permissions(meta.permissions())?;

        if !replaced.is_empty() {
            let mut mmap_target = unsafe { MmapMut::map_mut(&file)? };
            mmap_target.deref_mut().write_all(&replaced.as_bytes())?;
            mmap_target.flush_async()?;
        }

        drop(mmap_source);
        drop(source);

        target.persist(fs::canonicalize(self.path.clone())?)?;
        Ok(())
    }
}
