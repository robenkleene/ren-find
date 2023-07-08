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
                        let writer = Writer::new(paths, &self.replacer);
                        let text = match writer.patch_preview(color) {
                            Ok(text) => text,
                            Err(_) => return Ok(()), // FIXME:
                        };
                        write!(write, "{}", text)?;
                    } else {
                        let writer = Writer::new(paths, &self.replacer);
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

    pub(crate) fn check_not_empty(mut file: File) -> Result<()> {
        let mut buf: [u8; 1] = Default::default();
        file.read_exact(&mut buf)?;
        Ok(())
    }
}
