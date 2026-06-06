use assert_cmd::cargo::*; // Import cargo_bin_cmd! macro and methods
#[cfg(test)]
mod gradient_test {
    use assert_cmd::Command;
    const APP_NAME: &str = "rustimageseditor";

    #[test]
    fn get_gradient_1() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin(APP_NAME).unwrap();

        cmd.arg("gradient")
            .arg("--savefilepath").arg("test-output/gradent_1.png")
            .arg("--width").arg("100")
            .arg("--height").arg("100")
            .arg("--transparency").arg("4");

        let _output = cmd.unwrap();

        Ok(())
    }
}