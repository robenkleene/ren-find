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

    let man = clap_mangen::Man::new(cmd);
    let mut man_path = std::path::PathBuf::from(var("OUT_DIR").unwrap());
    man_path.push("ren.1");
    let mut out = std::fs::File::create(man_path).unwrap();
    man.render(&mut out).unwrap();
}
