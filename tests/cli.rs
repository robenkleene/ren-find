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
        let input = fs::read_to_string("tests/data/line-endings/grep.txt").expect("Error reading input");
        let ending_path = Path::new("tests/data/line-endings/ending.txt");
        let noending_path = Path::new("tests/data/line-endings/noending.txt");
        let ending_file_name = ending_path.file_name().expect("Error getting filename");
        let noending_file_name = noending_path.file_name().expect("Error getting filename");
        let tmp_dir = tempfile::tempdir()?;
        let tmp_dir_path = tmp_dir.path();
        let ending_path_dst = tmp_dir_path.join(ending_file_name);
        let noending_path_dst = tmp_dir_path.join(noending_file_name);
        fs::copy(ending_path, &ending_path_dst).expect("Error copying file");
        fs::copy(noending_path, &noending_path_dst).expect("Error copying file");
        mov()
            .current_dir(tmp_dir_path)
            .write_stdin(input)
            .args(&["foo", "bar", "-w"])
            .assert()
            .success();
        fn has_eol(path: &PathBuf) -> std::io::Result<bool> {
            let mut file = File::open(&path)?;
            let mut buffer = [0; 1];

            match file.seek(std::io::SeekFrom::End(-1)) {
                Ok(_) => (),
                // Empty file
                Err(_) => return Ok(false),
            }

            file.read(&mut buffer[..])?;
            if buffer == [b'\n'] {
                return Ok(true);
            };
            Ok(false)
        }
        let ending_result = has_eol(&ending_path.to_path_buf())?;
        let noending_result = has_eol(&noending_path.to_path_buf())?;
        let ending_dst_result = has_eol(&ending_path_dst.to_path_buf())?;
        let noending_dst_result = has_eol(&noending_path_dst.to_path_buf())?;
        assert!(ending_result);
        assert!(!noending_result);
        assert_eq!(ending_result, ending_dst_result);
        assert_eq!(noending_result, noending_dst_result);
        Ok(())
    }
}
