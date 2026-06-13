
#[cfg(test)]
mod createicon_test {
    use assert_cmd::Command;
    use crate::common_settings.get_app_name;

    const APP_NAME: &str = "rustimageseditor";

    #[test]
    fn create_icon() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin(APP_NAME).unwrap();

        cmd.arg("create-icon")
            .arg("--openfilepath").arg("test-images/dog1.png")
            .arg("--savedirectory").arg("test-output")
            .arg("--savefilename").arg("dog_icon_");

        let _output = cmd.unwrap();

        Ok(())
    }
}