#[cfg(test)]
mod grid_test {
    use assert_cmd::Command;
    const APP_NAME: &str = "rustimageseditor";

    #[test]
    fn should_write_to_file() {
        let mut cmd = Command::cargo_bin(APP_NAME).unwrap();

        cmd.arg("write-to-file")
            .arg("--filepath").arg("test-output/hello.txt")
            .arg("--content").arg("This is a test");

        let output = cmd.unwrap();
    }

}