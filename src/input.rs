use crate::{edit::Edit, output::OutputType, writer::Writer, Replacer, Result};
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

    pub(crate) fn run(&self, preview: bool, delete: bool, color: bool, pager: Option<String>) -> Result<()> {
        {
            let stdin = std::io::stdin();
            let handle = stdin.lock();

            // FIXME: Instantiating `output_type` and `write` should only happen if `preview` is true
            let mut output_type = match OutputType::for_pager(pager, true) {
                Ok(output_type) => output_type,
                Err(_) => return Ok(()), // FIXME:
            };

            let write = match output_type.handle() {
                Ok(write) => write,
                Err(_) => return Ok(()), // FIXME:
            };

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
            sorted_paths.sort_by(|a, b| b.to_str().unwrap().len().cmp(&a.to_str().unwrap().len()));
            let mut src_to_dst: Option<IndexMap<PathBuf, PathBuf>> = None;
            if let Some(replacer) = &self.replacer {
                let edit = Edit::new(&replacer);
                src_to_dst = match edit.parse(&sorted_paths) {
                    Ok(src_to_dst) => Some(src_to_dst),
                    Err(_) => return Ok(()), // FIXME:
                };
            }
            if preview {
                let writer = Writer::new(sorted_paths, src_to_dst);
                let text = match writer.patch_preview(color, delete) {
                    Ok(text) => text,
                    Err(_) => return Ok(()), // FIXME:
                };
                write!(write, "{}", text)?;
            } else {
                let writer = Writer::new(sorted_paths, src_to_dst);
                if let Err(_) = writer.write_file(delete) {
                    return Ok(()); // FIXME:
                }
            }
            drop(write);
        }
        Ok(())
    }
}
