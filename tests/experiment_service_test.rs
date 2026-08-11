#[cfg(test)]
mod experiment_service_test {
    use assert_cmd::Command;
    const APP_NAME: &str = "rustimageseditor";

    #[test]
    fn experiment_1_test() {
        let mut cmd = Command::cargo_bin(APP_NAME).unwrap();

        cmd.arg("experiment")
            .arg("--openfilepath").arg("test-images/experiment/dog1.png")
            .arg("--savefilepath").arg("test-output/experiment/dog_updated.png");

        let output = cmd.unwrap();
    }

    #[test]
    fn experiment_2_test() {
        let mut cmd = Command::cargo_bin(APP_NAME).unwrap();

        cmd.arg("experiment")
            .arg("--openfilepath").arg("test-images/dog1.png")
            .arg("--savefilepath").arg("test-output/experiment/dog_updated.png");

        let output = cmd.unwrap();
    }
}