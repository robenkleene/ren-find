#[cfg(test)]
#[cfg(not(mov_cross_compile))] // Cross-compilation does not allow to spawn threads but `command.assert()` would do.

mod cli {
    use anyhow::Result;
    use assert_cmd::Command;
    use std::fs;
    use std::fs::File;
    use std::path::{Path, PathBuf};
    use std::io::Read;
    use std::io::Seek;

    fn mov() -> Command {
        Command::cargo_bin("mov").expect("Error invoking mov")
    }

    #[test]
    fn patch_preview_files_args() -> Result<()> {
        let input = fs::read_to_string("tests/data/simple/start.txt").expect("Error reading input");
        let result = fs::read_to_string("tests/data/simple/patch.patch").expect("Error reading input");
        mov()
            .current_dir("tests/data/simple")
            .write_stdin(input)
            .args(&["changes", "altered"])
            .assert()
            .success()
            .stdout(result);
        Ok(())
    }

    #[test]
    fn patch_preview_missing() -> Result<()> {
        let input = fs::read_to_string("tests/data/simple/start.txt").expect("Error reading input");
        mov()
            .current_dir("tests/data/missing")
            .write_stdin(input)
            .args(&["missing", "replaced"])
            .assert()
            .success()
            .stdout("");
        Ok(())
    }

    #[test]
    fn recursive_dirs() -> Result<()> {
        let input = fs::read_to_string("tests/data/dirs/find.txt").expect("Error reading input");
        let result = fs::read_to_string("tests/data/dirs/patch.patch").expect("Error reading input");
        mov()
            .current_dir("tests/data/dirs")
            .write_stdin(input)
            .args(&["changes", "altered"])
            .assert()
            .success()
            .stdout(result);
        Ok(())
    }

    #[test]
    fn test_moving() -> Result<()> {
        let input = fs::read_to_string("tests/data/move/find.txt").expect("Error reading input");
        let file_path_component = "tests/data/mov/change dir with spaces/change dir with spaces two/change file with spaces";
        let file_path = Path::new("tests/data/mov").join(file_path_component);
        let tmp_dir = tempfile::tempdir()?;
        let tmp_dir_path = tmp_dir.path();
        let file_path_dst = tmp_dir_path.join(file_path_component);
        fs::copy(file_path, &file_path_dst).expect("Error copying file");
        mov()
            .current_dir(tmp_dir_path)
            .write_stdin(input)
            .args(&["change", "altered", "-w"])
            .assert()
            .success();
        assert!(!Path::exists(&file_path_dst));
        let file_path_component_moved = "tests/data/mov/altered dir with spaces/altered dir with spaces two/altered file with spaces";
        let file_path_moved = tmp_dir_path.join(file_path_component_moved);
        assert!(Path::exists(&file_path_moved));
        Ok(())
    }
}
