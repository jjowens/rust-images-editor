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

    #[test]
    fn should_create_2x2() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin(APP_NAME).unwrap();

        cmd.arg("grid")
            .arg("--rows").arg("2")
            .arg("--columns").arg("2")
            .arg("--openfilepath").arg("test-images/dog1.png")
            .arg("--savedirectory").arg("test-output")
            .arg("--savefilename").arg("dog2x2");

        let _output = cmd.unwrap();

        Ok(())
    }

    #[test]
    fn create_10x1() -> Result<(), Box<dyn std::error::Error>> {
        let directory_output = "test-output/grid/10x1";
        let _ = recreate_folder(directory_output);

        let mut cmd = Command::cargo_bin(APP_NAME).unwrap();

        cmd.arg("grid")
            .arg("--rows").arg("10")
            .arg("--columns").arg("1")
            .arg("--openfilepath").arg("test-images/dog1.png")
            .arg("--savedirectory").arg(directory_output)
            .arg("--savefilename").arg("dog10x1");

        let _output = cmd.unwrap();

        Ok(())
    }

    #[test]
    fn create_20x20() -> Result<(), Box<dyn std::error::Error>> {
        let directory_output = "test-output/grid/20x20";
        let _ = recreate_folder(directory_output);

        let mut cmd = Command::cargo_bin(APP_NAME).unwrap();

        cmd.arg("grid")
            .arg("--rows").arg("20")
            .arg("--columns").arg("20")
            .arg("--openfilepath").arg("test-images/dog1.png")
            .arg("--savedirectory").arg(directory_output)
            .arg("--savefilename").arg("dog10x1");

        let _output = cmd.unwrap();

        Ok(())
    }
}