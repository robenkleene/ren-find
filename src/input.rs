use crate::{edit::Edit, output::OutputType, writer::Writer, Replacer, Result};
use std::io::prelude::*;
use std::path::PathBuf;

pub(crate) struct App {
    replacer: Replacer,
}

impl App {
    pub(crate) fn new(replacer: Replacer) -> Self {
        Self { replacer }
    }

    pub(crate) fn run(&self, preview: bool, color: bool, pager: Option<String>) -> Result<()> {
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

            let mut paths = Vec::new();
            for line in handle.lines() {
                // Trim any trailing slashes by getting the `file_name()` and then adding it back
                let path = PathBuf::from(line?);
                let filename = path.file_name().unwrap();
                let filename_dir = path.parent().unwrap();
                let key = filename_dir.join(filename);
                paths.push(key);
            }
            let mut sorted_paths = paths.clone();
            sorted_paths.sort_by(|a, b| b.to_str().unwrap().len().cmp(&a.to_str().unwrap().len()));
            let edit = Edit::new(&self.replacer);
            match edit.parse(&sorted_paths) {
                Ok(src_to_dst) => {
                    if preview {
                        let writer = Writer::new(paths, src_to_dst);
                        let text = match writer.patch_preview(color) {
                            Ok(text) => text,
                            Err(_) => return Ok(()), // FIXME:
                        };
                        write!(write, "{}", text)?;
                    } else {
                        let writer = Writer::new(sorted_paths, src_to_dst);
                        if let Err(_) = writer.write_file() {
                            return Ok(()); // FIXME:
                        }
                    }
                }
                Err(_) => {
                    return Ok(()); // FIXME:
                }
            }
            drop(write);
        }
        Ok(())
    }
}
