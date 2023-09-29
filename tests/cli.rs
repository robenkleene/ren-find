#[cfg(test)]
#[cfg(not(mov_cross_compile))] // Cross-compilation does not allow to spawn threads but `command.assert()` would do.

mod cli {
    use anyhow::Result;
    use assert_cmd::Command;
    use std::fs;
    use std::path::Path;

    fn ren() -> Command {
        Command::cargo_bin("ren").expect("Error invoking ren")
    }

    #[test]
    fn multiple_preview() -> Result<()> {
        let input = fs::read_to_string("tests/data/multiple/find.txt").expect("Error reading input");
        let result = fs::read_to_string("tests/data/multiple/patch.patch").expect("Error reading result");
        ren()
            .current_dir("tests/data/multiple")
            .write_stdin(input)
            .args(&["changes", "altered"])
            .assert()
            .success()
            .stdout(result);
        Ok(())
    }

    #[test]
    fn missing_preview() -> Result<()> {
        let input = fs::read_to_string("tests/data/missing/find.txt").expect("Error reading input");
        ren()
            .current_dir("tests/data/missing")
            .write_stdin(input)
            .args(&["missing", "replaced"])
            .assert()
            .success()
            .stdout("");
        Ok(())
    }

    #[test]
    fn simple_preview() -> Result<()> {
        let input = fs::read_to_string("tests/data/simple/find.txt").expect("Error reading input");
        let result = fs::read_to_string("tests/data/simple/patch.patch").expect("Error reading result");
        ren()
            .current_dir("tests/data/simple")
            .write_stdin(input)
            .args(&["changes", "altered"])
            .assert()
            .success()
            .stdout(result);
        Ok(())
    }

    #[test]
    fn nested_preview() -> Result<()> {
        let input = fs::read_to_string("tests/data/nested/find.txt").expect("Error reading input");
        let result = fs::read_to_string("tests/data/nested/patch.patch").expect("Error reading result");
        ren()
            .current_dir("tests/data/nested")
            .write_stdin(input)
            .args(&["changes", "altered"])
            .assert()
            .success()
            .stdout(result);
        Ok(())
    }

    #[test]
    fn dirs_preview() -> Result<()> {
        let input = fs::read_to_string("tests/data/dirs/find.txt").expect("Error reading input");
        let result = fs::read_to_string("tests/data/dirs/patch.patch").expect("Error reading result");
        ren()
            .current_dir("tests/data/dirs")
            .write_stdin(input)
            .args(&["changes", "altered"])
            .assert()
            .success()
            .stdout(result);
        Ok(())
    }

    #[test]
    fn simple_move() -> Result<()> {
        let input = fs::read_to_string("tests/data/simple/find.txt").expect("Error reading input");
        let file_path_component = "changes";
        let file_path = Path::new("tests/data/simple").join(file_path_component);
        let tmp_dir = tempfile::tempdir()?;
        let tmp_dir_path = tmp_dir.path();
        let file_path_dst = tmp_dir_path.join(file_path_component);
        let prefix = file_path_dst.parent().unwrap();
        std::fs::create_dir_all(prefix).unwrap();
        fs::copy(file_path, &file_path_dst).expect("Error copying file");
        ren()
            .current_dir(tmp_dir_path)
            .write_stdin(input)
            .args(&["changes", "altered", "-w"])
            .assert()
            .success();
        assert!(!Path::exists(&file_path_dst));
        let file_path_component_moved = "altered";
        let file_path_moved = tmp_dir_path.join(file_path_component_moved);
        assert!(Path::exists(&file_path_moved));
        Ok(())
    }

    #[test]
    fn nested_move() -> Result<()> {
        let input = fs::read_to_string("tests/data/nested/find.txt").expect("Error reading input");
        let file_path_component = "changes dir with spaces/stays dir with spaces two/changes file with spaces";
        let file_path = Path::new("tests/data/nested").join(file_path_component);
        let tmp_dir = tempfile::tempdir()?;
        let tmp_dir_path = tmp_dir.path();
        let file_path_dst = tmp_dir_path.join(file_path_component);
        let prefix = file_path_dst.parent().unwrap();
        std::fs::create_dir_all(prefix).unwrap();
        fs::copy(file_path, &file_path_dst).expect("Error copying file");
        ren()
            .current_dir(tmp_dir_path)
            .write_stdin(input)
            .args(&["changes", "altered", "-w"])
            .assert()
            .success();
        assert!(!Path::exists(&file_path_dst));
        let file_path_component_moved = "altered dir with spaces/stays dir with spaces two/altered file with spaces";
        let file_path_moved = tmp_dir_path.join(file_path_component_moved);
        assert!(Path::exists(&file_path_moved));
        Ok(())
    }

    #[test]
    fn simple_delete_preview() -> Result<()> {
        let input = fs::read_to_string("tests/data/simple/find.txt").expect("Error reading input");
        let result = fs::read_to_string("tests/data/simple/delete.patch").expect("Error reading result");
        ren()
            .current_dir("tests/data/simple")
            .write_stdin(input)
            .args(&["-d"])
            .assert()
            .success()
            .stdout(result);
        Ok(())
    }

    #[test]
    fn nested_delete_preview() -> Result<()> {
        let input = fs::read_to_string("tests/data/nested/find.txt").expect("Error reading input");
        let result = fs::read_to_string("tests/data/nested/delete.patch").expect("Error reading result");
        ren()
            .current_dir("tests/data/nested")
            .write_stdin(input)
            .args(&["-d"])
            .assert()
            .success()
            .stdout(result);
        Ok(())
    }

    #[test]
    fn simple_delete() -> Result<()> {
        let input = fs::read_to_string("tests/data/simple/find.txt").expect("Error reading input");
        let file_path_component = "changes";
        let file_path = Path::new("tests/data/simple").join(file_path_component);
        let tmp_dir = tempfile::tempdir()?;
        let tmp_dir_path = tmp_dir.path();
        let file_path_dst = tmp_dir_path.join(file_path_component);
        let prefix = file_path_dst.parent().unwrap();
        std::fs::create_dir_all(prefix).unwrap();
        fs::copy(file_path, &file_path_dst).expect("Error copying file");
        ren()
            .current_dir(tmp_dir_path)
            .write_stdin(input)
            .args(&["-d", "-w"])
            .assert()
            .success();
        assert!(!Path::exists(&file_path_dst));
        Ok(())
    }

    #[test]
    fn nested_delete() -> Result<()> {
        let input = fs::read_to_string("tests/data/nested/find.txt").expect("Error reading input");
        let tmp_dir = tempfile::tempdir()?;
        let tmp_dir_path = tmp_dir.path();

        // Path 1
        let file_path_component = "changes dir with spaces/stays dir with spaces two/changes file with spaces";
        let file_path = Path::new("tests/data/nested").join(file_path_component);
        let file_path_dst = tmp_dir_path.join(file_path_component);
        let prefix = file_path_dst.parent().unwrap();
        std::fs::create_dir_all(prefix).unwrap();
        fs::copy(file_path, &file_path_dst).expect("Error copying file");

        // Path 2
        let file_path_component2 = "changes dir with spaces 2/stays";
        let file_path2 = Path::new("tests/data/nested").join(file_path_component2);
        let file_path_dst2 = tmp_dir_path.join(file_path_component2);
        let prefix2 = file_path_dst2.parent().unwrap();
        std::fs::create_dir_all(prefix2).unwrap();
        fs::copy(file_path2, &file_path_dst2).expect("Error copying file");

        ren()
            .current_dir(tmp_dir_path)
            .write_stdin(input)
            .args(&["-D", "-w"])
            .assert()
            .success();
        assert!(!Path::exists(&file_path_dst));
        assert!(!Path::exists(&file_path_dst2));
        Ok(())
    }

    #[test]
    fn nested_delete_error() -> Result<()> {
        let input = fs::read_to_string("tests/data/nested/find.txt").expect("Error reading input");
        let tmp_dir = tempfile::tempdir()?;
        let tmp_dir_path = tmp_dir.path();

        // Path 1
        let file_path_component = "changes dir with spaces/stays dir with spaces two/changes file with spaces";
        let file_path = Path::new("tests/data/nested").join(file_path_component);
        let file_path_dst = tmp_dir_path.join(file_path_component);
        let prefix = file_path_dst.parent().unwrap();
        std::fs::create_dir_all(prefix).unwrap();
        fs::copy(file_path, &file_path_dst).expect("Error copying file");

        // Path 2
        let file_path_component2 = "changes dir with spaces 2/stays";
        let file_path2 = Path::new("tests/data/nested").join(file_path_component2);
        let file_path_dst2 = tmp_dir_path.join(file_path_component2);
        let prefix2 = file_path_dst2.parent().unwrap();
        std::fs::create_dir_all(prefix2).unwrap();
        fs::copy(file_path2, &file_path_dst2).expect("Error copying file");

        let command = ren()
            .current_dir(tmp_dir_path)
            .write_stdin(input)
            .args(&["-d", "-w"])
            .assert()
            .success();
        let output = command.get_output();
        assert!(output.stderr.len() > 0);
        assert!(!Path::exists(&file_path_dst));
        assert!(Path::exists(&file_path_dst2));
        Ok(())
    }
}
