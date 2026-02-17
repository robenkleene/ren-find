use crate::{EditKind, edit::Edit, output::OutputType, writer::Writer, Replacer, Result};
use std::io::prelude::*;
use std::path::PathBuf;
use indexmap::IndexMap;

pub(crate) struct App {
    replacer: Option<Replacer>
}

impl App {
    pub(crate) fn new(replacer: Option<Replacer>) -> Self {
        Self { replacer }
    }

    pub(crate) fn run(&self, preview: bool, delete_kind: EditKind, color: bool, pager: Option<String>) -> Result<()> {
        {
            let stdin = std::io::stdin();
            let handle = stdin.lock();

            let mut output_type = OutputType::for_pager(pager, true)?;
            let write = output_type.handle()?;

            let mut sorted_paths = Vec::new();
            for line in handle.lines() {
                // Trim any trailing slashes by getting the `file_name()` and then adding it back
                let path = PathBuf::from(line?);
                let filename = match path.file_name() {
                  Some(filename) => filename,
                  None => continue
                };
                let filename_dir = match path.parent() {
                  Some(filename_dir) => filename_dir,
                  None => continue
                };
                let mut key = filename_dir.join(filename);
                // Add back the slash if the input had it
                if path.to_string_lossy().as_bytes().last() == Some(&b'/') {
                    key.push("");
                }
                sorted_paths.push(key);
            }
            sorted_paths.sort_by(|a, b| b.to_string_lossy().len().cmp(&a.to_string_lossy().len()));
            let mut src_to_dst: Option<IndexMap<PathBuf, PathBuf>> = None;
            if let Some(replacer) = &self.replacer {
                let edit = Edit::new(&replacer);
                src_to_dst = Some(edit.parse(&sorted_paths)?);
            }
            if preview {
                let writer = Writer::new(sorted_paths, src_to_dst);
                let text = writer.patch_preview(color, delete_kind)?;
                write!(write, "{}", text)?;
            } else {
                let writer = Writer::new(sorted_paths, src_to_dst);
                writer.write_file(delete_kind)?;
            }
            drop(output_type);
        }
        Ok(())
    }
}
