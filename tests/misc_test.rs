#[cfg(test)]
mod misc_test {
    use assert_cmd::Command;
    use chrono::Local;

    const APP_NAME: &str = "rustimageseditor";

    #[test]
    fn custom_image() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin(APP_NAME).unwrap();

        cmd.arg("misc-custom")
            .arg("--savefilepath").arg("test-output/misc/custom_100x100.png")
            .arg("--width").arg("100")
            .arg("--height").arg("100");

        let _output = cmd.unwrap();

        Ok(())
    }

}