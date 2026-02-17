mod cli;
mod error;
mod input;
mod output;
mod writer;
mod edit;
mod less;
pub(crate) mod replacer;

pub(crate) use self::input::App;
pub(crate) use error::Result;
use replacer::Replacer;
use std::env;
use std::io::IsTerminal;


#[derive(Debug)]
enum EditKind {
    Replace,
    Delete,
    DeleteAll,
}

fn main() -> Result<()> {
    use clap::Parser;
    let options = cli::Options::parse();

    let color = options.color || (!options.no_color && std::io::stdout().is_terminal());

    let pager = env::var("REN_PAGER").ok();

    let delete_kind = if options.delete_all {
        EditKind::DeleteAll
    } else if options.delete {
        EditKind::Delete
    } else {
        EditKind::Replace
    };

    match (options.find, options.replace_with) {
        (Some(find), Some(replace_with)) => {
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
        }
        (None, None) if options.delete || options.delete_all => {
            App::new(None)
            .run(!options.write, delete_kind, color, pager)?;
        }
        (Some(_), None) => {
            eprintln!("Error: missing replacement argument. Usage: ren <find> <replace_with>");
            std::process::exit(2);
        }
        _ => {
            eprintln!("Error: missing arguments. Usage: ren <find> <replace_with> or ren -d/-D");
            std::process::exit(2);
        }
    }
    Ok(())
}
