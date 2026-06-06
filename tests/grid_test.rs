#[cfg(test)]
mod grid_test {
    use assert_cmd::Command;
    const APP_NAME: &str = "rustimageseditor";
    #[test]
    fn should_create_2x2() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin(APP_NAME).unwrap();

        cmd.arg("grid")
            .arg("--rows 2").arg("--columns 2")
            .arg("--openfilepath").arg("test-images/dog.png")
            .arg("--savefilepath").arg("test-output/dog_sliced.png");

        Ok(())
    }

    #[test]
    fn create_large_number_of_slices_one_row() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin(APP_NAME).unwrap();

        cmd.arg("grid")
            .arg("--rows").arg("10")
            .arg("--columns").arg("1")
            .arg("--openfilepath").arg("test-images/dog.png")
            .arg("--savefilepath").arg("test-output/dog_10x1.png");

        Ok(())
    }
}