#[cfg(test)]
mod createicon_test {
    use assert_cmd::Command;
    const APP_NAME: &str = "rustimageseditor";

    #[test]
    fn create_icon() {
        let mut cmd = Command::cargo_bin(APP_NAME).unwrap();

        cmd.arg("create-icon")
            .arg("--openfilepath").arg("test-images/dog1.png")
            .arg("--savedirectory").arg("test-output")
            .arg("--savefilename").arg("dog_icon_");

        let output = cmd.unwrap();
    }
}