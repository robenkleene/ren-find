include!("src/cli.rs");

fn main() {
    use std::{env::var, fs};
    use clap::CommandFactory;
    use clap_complete::Shell;

    let mut cmd = Options::command();
    let out_dir = var("SHELL_COMPLETIONS_DIR").or(var("OUT_DIR")).unwrap();

    fs::create_dir_all(&out_dir).unwrap();

    for shell in [Shell::Bash, Shell::Fish, Shell::Zsh, Shell::PowerShell] {
        clap_complete::generate_to(shell, &mut cmd, "ren", &out_dir).unwrap();
    }

    create_man_page();
}

fn create_man_page() {
    use man::prelude::*;
    let page = Manual::new("ren")
        .about("Rename files from find results")
        .flag(
            Flag::new()
                .short("-w")
                .long("--write")
                .help("Write the output to files directly (instead of outputting a patch)."),
        )
        .flag(
            Flag::new()
                .short("-d")
                .long("--delete")
                .help("Delete files and directories."),
        )
        .flag(
            Flag::new()
                .short("-D")
                .long("--delete-all")
                .help("Delete files and directories, including directories that aren't empty."),
        )
        .flag(
            Flag::new()
                .short("-s")
                .long("--string-mode")
                .help("Treat expressions as non-regex strings."),
        )
        .flag(
            Flag::new()
                .short("-n")
                .help("Limit the number of replacements per line."),
        )
        .flag(
            Flag::new()
                .long("--color")
                .help("Enable color (the default if the output is a TTY)."),
        )
        .flag(
            Flag::new()
                .long("--no-color")
                .help("Disable color."),
        )
        .flag(Flag::new().short("-f").long("--flags").help(
            "Regex flags. May be combined (like `-f mc`).\n\n\
             c - case-sensitive\n\
             e - disable multi-line matching\n\
             i - case-insensitive\n\
             m - multi-line matching\n\
             s - make `.` match newlines\n\
             w - match full words only",
        ))
        .arg(Arg::new("find"))
        .arg(Arg::new("replace_with"))
        .example(
            Example::new()
                .text("Preview renaming files containing 'foo' to 'bar'")
                .command("find . -name '*foo*' | ren foo bar"),
        )
        .example(
            Example::new()
                .text("Rename the files")
                .command("find . -name '*foo*' | ren -w foo bar"),
        )
        .example(
            Example::new()
                .text("Preview deleting files")
                .command("find . -name '*.bak' | ren -d"),
        )
        .example(
            Example::new()
                .text("Use regex capture groups to rename files")
                .command(r#"find . -name '*.jpeg' | ren '(.*)\.jpeg' '$1.jpg'"#),
        )
        .example(
            Example::new()
                .text("Use string-literal mode to rename files with special characters")
                .command("find . -name '*[1]*' | ren -s '[1]' '(1)'"),
        )
        .custom(
            Section::new("special characters")
                .paragraph("Use -- to separate options from arguments when the pattern starts with a hyphen (e.g., ren -- '--foo' '--bar').")
        )
        .render();

    let mut man_path =
        std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    man_path.push("ren.1");
    std::fs::write(man_path, page).expect("Error writing man page");
}
