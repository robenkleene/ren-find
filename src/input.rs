use crate::{edit::Edit, output::OutputType, patcher::Patcher, writer::Writer, Replacer, Result};
use std::fs::File;
use std::io::prelude::*;
use std::str;

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

            match Edit::parse(handle) {
                Ok(paths) => {
                    if preview {
                        for path in paths {
                            let patcher = Patcher::new(edits, &self.replacer);
                            let replaced = self.replacer.replace(path.to_string_lossy().as_bytes());
                            let result = str::from_utf8(&replaced);
                            // TODO: Verify that each file is valid
                            let writer = Writer::new(path.to_path_buf(), &patcher);
                            let text = match writer.patch_preview(color) {
                                Ok(text) => text,
                                Err(_) => continue, // FIXME:
                            };

                            write!(write, "{}", text)?;
                        }
                    } else {
                        for path in paths {
                            let patcher = Patcher::new(edits, &self.replacer);
                            if let Err(_) = Self::check_not_empty(File::open(&path)?) {
                                return Ok(()); // FIXME:
                            }
                            let writer = Writer::new(path, &patcher);
                            if let Err(_) = writer.write_file() {
                                return Ok(()); // FIXME:
                            }
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

    pub(crate) fn check_not_empty(mut file: File) -> Result<()> {
        let mut buf: [u8; 1] = Default::default();
        file.read_exact(&mut buf)?;
        Ok(())
    }
}
