#[cfg(test)]
mod grid_test {
    use std::fs;
    use assert_cmd::Command;
    const APP_NAME: &str = "rustimageseditor";

    fn recreate_folder(directory_path: &str) -> std::io::Result<()> {
        if !fs::exists(directory_path)? {
            fs::create_dir(directory_path)?;
        } else {
            fs::create_dir(directory_path)?;
            fs::remove_dir_all(directory_path)?;
        }

        Ok(())
    }

    fn create_grid_image(rows: &str, columns: &str, open_file_path: &str, save_directory_path: &str, save_file_name: &str) -> Result<(), String> {
        let output = recreate_folder(save_directory_path);
        let mut cmd = Command::cargo_bin(APP_NAME).unwrap();

        cmd.arg("grid")
            .arg("--rows").arg(rows)
            .arg("--columns").arg(columns)
            .arg("--openfilepath").arg(open_file_path)
            .arg("--savedirectory").arg(save_directory_path)
            .arg("--savefilename").arg(save_file_name);

        let output = cmd.unwrap();

        Ok(())
    }

    #[test]
    fn should_create_2x2() {
        let output = create_grid_image("2", "2", "test-images/dog1.png", "test-output/grid/2x2", "dog2x2");

    }

    #[test]
    fn create_10x1() {
        let output =create_grid_image("10", "1", "test-images/dog1.png", "test-output/grid/10x1", "dog10x1");
    }

    #[test]
    fn create_20x20() {
        let output = create_grid_image("20", "20", "test-images/dog1.png", "test-output/grid/20x20", "dog20x20");
    }
}