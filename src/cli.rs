use clap::Parser;

fn parse_positive_usize(s: &str) -> std::result::Result<usize, String> {
    let n: usize = s.parse().map_err(|e: std::num::ParseIntError| e.to_string())?;
    if n == 0 {
        return Err("-n expects a positive integer".into());
    }
    Ok(n)
}

#[derive(Debug, Parser)]
#[command(version, next_line_help = true, after_help = "\
SPECIAL CHARACTERS:
  Use -- to separate options from arguments when the pattern starts
  with a hyphen (e.g., ren -- '--foo' '--bar').")]
pub(crate) struct Options {
    #[arg(short = 'w', long = "write")]
    /// Write the output to files directly (instead of outputting a patch)
    ///
    /// If this flag is not present, and a patch is output, then the default pager is `less`. The
    /// environment variable REN_PAGER can be used to override the pager.
    pub write: bool,

    #[arg(short = 'd', long = "delete")]
    /// Delete files and directories
    pub delete: bool,

    #[arg(short = 'D', long = "delete-all")]
    /// Delete files and directories, including directories that aren't empty
    pub delete_all: bool,

    #[arg(short = 's', long = "string-mode")]
    /// Treat expressions as non-regex strings
    pub literal_mode: bool,

    #[arg(short = 'n', value_parser = parse_positive_usize)]
    /// Limit the number of replacements per line
    pub replacements: Option<usize>,

    #[arg(long = "color")]
    /// Enable color (the default if the output is a TTY)
    pub color: bool,

    #[arg(long = "no-color")]
    /// Disable color
    pub no_color: bool,

    #[arg(short = 'f', long = "flags", verbatim_doc_comment)]
    /// Regex flags. May be combined (like `-f mc`)
    ///
    /// c - case-sensitive
    /// e - disable multi-line matching
    /// i - case-insensitive
    /// m - multi-line matching
    /// s - make `.` match newlines
    /// w - match full words only
    pub flags: Option<String>,

    /// The regexp or string (if -s) to search for.
    pub find: Option<String>,

    /// What to replace each match with. Unless in string mode, you may
    /// use captured values like $1, $2, etc.
    pub replace_with: Option<String>,
}

