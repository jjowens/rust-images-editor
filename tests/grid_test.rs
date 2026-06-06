#[cfg(test)]
mod grid_test {
    use assert_cmd::Command;
    const APP_NAME: &str = "rustimageseditor";
    #[test]
    fn should_create_2x2() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin(APP_NAME).unwrap();

        cmd.arg("grid")
            .arg("--rows 3").arg("--columns 3")
            .arg("--filepath dog.png");

        Ok(())
    }
}