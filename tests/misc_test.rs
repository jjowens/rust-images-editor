#[cfg(test)]
mod misc_test {
    use assert_cmd::Command;
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

    #[test]
    fn custom_square_centered() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin(APP_NAME).unwrap();

        cmd.arg("misc-square-centered")
            .arg("--savefilepath").arg("test-output/misc/custom_centred.png")
            .arg("--imagewidth").arg("100")
            .arg("--imageheight").arg("100")
            .arg("--squarewidth").arg("10")
            .arg("--squareheight").arg("10");

        let _output = cmd.unwrap();

        // misc-square-centered --savefilepath test-output/misc/custom_centred.png --imagewidth 100 --imageheight 100 --squarewidth 10 --squareheight 10

        Ok(())
    }

}