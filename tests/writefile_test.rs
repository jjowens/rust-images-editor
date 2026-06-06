use assert_cmd::cargo::*; // Import cargo_bin_cmd! macro and methods
//use predicates::prelude::*; // Used for writing assertions
#[cfg(test)]
mod grid_test {
    use assert_cmd::{cargo_bin_cmd, Command};

    #[test]
    fn should_write_to_file() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("rustimageseditor").unwrap();

        cmd.arg("write-to-file")
            .arg("--filepath").arg("test-output/hello.txt")
            .arg("--content").arg("This is a test");

        let _output = cmd.unwrap();

        Ok(())
    }

}