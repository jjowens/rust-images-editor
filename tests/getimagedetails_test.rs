#[cfg(test)]
mod getimagedetails_test {
    use assert_cmd::Command;
    const APP_NAME: &str = "rustimageseditor";

    #[test]
    fn get_dog1_details() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin(APP_NAME).unwrap();

        cmd.arg("image-details")
            .arg("--filepath").arg("test-images/dog1.png");

        let _output = cmd.unwrap();

        Ok(())
    }
}