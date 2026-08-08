#[cfg(test)]
mod colourspace_details_test {
    use assert_cmd::Command;
    const APP_NAME: &str = "rustimageseditor";

    #[test]
    fn get_dog1_colourspace_details() {
        let mut cmd = Command::cargo_bin(APP_NAME).unwrap();

        cmd.arg("colour-space-details")
            .arg("--filepath").arg("test-images/dog1.png");

        let output = cmd.unwrap();
    }

    #[test]
    fn get_dog2_colourspace_details() {
        let mut cmd = Command::cargo_bin(APP_NAME).unwrap();

        cmd.arg("colour-space-details")
            .arg("--filepath").arg("test-images/dog2.png");

        let output = cmd.unwrap();
    }
}