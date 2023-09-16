mod cli;
mod error;
mod input;
mod output;
mod writer;
mod edit;
// FIXME: Look at `pub(crate)` calls are all these necessary?
mod less;
pub(crate) mod replacer;
pub(crate) mod utils;

pub(crate) use self::input::App;
pub(crate) use error::Result;
use replacer::Replacer;
use std::env;
use std::process;

#[derive(Debug)]
enum EditKind {
    Replace,
    Delete,
    DeleteAll,
}

fn main() -> Result<()> {
    // Ignore ctrl-c (SIGINT) to avoid leaving an orphaned pager process.
    // See https://github.com/dandavison/delta/issues/681
    ctrlc::set_handler(|| {})
        .unwrap_or_else(|err| eprintln!("Failed to set ctrl-c handler: {}", err));

    use structopt::StructOpt;
    let options = cli::Options::from_args();

    let is_tty = atty::is(atty::Stream::Stdout);
    let color = if options.color {
        true
    } else if options.no_color {
        false
    } else if is_tty {
        true
    } else {
        false
    };

    let pager = env::var("REN_PAGER").ok();

    let delete_kind = || -> EditKind {
        if options.delete_all {
            return EditKind::DeleteAll;
        } else if options.delete {
            return EditKind::Delete;
        } else {
            return EditKind::Replace;
        }
    }();

    if let (Some(find), Some(replace_with)) = (options.find, options.replace_with) {
        App::new(
            Some(Replacer::new(
                find,
                replace_with,
                options.literal_mode,
                options.flags,
                options.replacements,
            )?)
        )
        .run(!options.write, delete_kind, color, pager)?;
    } else if options.delete || options.delete_all {
        App::new(None)
        .run(!options.write, delete_kind, color, pager)?;
    }
    process::exit(0);
}
