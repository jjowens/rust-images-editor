#[cfg(test)]
mod splitimage_test {
    use std::fs;
    use assert_cmd::Command;
    const APP_NAME: &str = "rustimageseditor";

    #[test]
    fn create_grayscale_image() {
        let mut cmd = Command::cargo_bin(APP_NAME).unwrap();

        cmd.arg("split-image")
            .arg("--openfilepath").arg("test-images/dog1.png")
            .arg("--savefilename").arg("test-output/dog_grayscale.jpg");

        let output = cmd.unwrap();
    }

    #[test]
    fn create_rgb8_image() {
        let mut cmd = Command::cargo_bin(APP_NAME).unwrap();

        cmd.arg("split-image")
            .arg("--openfilepath").arg("test-images/dog1.png")
            .arg("--savefilename").arg("test-output/dog_rgb8.jpg");

        let output = cmd.unwrap();
    }


    fn hue_rotate_image(open_file_path: &str, save_file_name: &str, rotate: i32) -> Result<(), String> {
        let mut cmd = Command::cargo_bin(APP_NAME).unwrap();

        cmd.arg("hue-rotate")
            .arg("--openfilepath").arg(open_file_path)
            .arg("--savefilename").arg(save_file_name)
            .arg("--rotate").arg(rotate.to_string());

        let _ = cmd.unwrap();

        Ok(())
    }
    #[test]
    fn create_hue_rotate_single_image() {
        let _ = hue_rotate_image("test-images/dog1.png", "test-output/dog_rotate_180.png",180);
    }

    #[test]
    fn create_hue_rotate_images() {
        let mut cmd = Command::cargo_bin(APP_NAME).unwrap();

        let mut str_builder : String = "".to_owned();
        
        for degree in 0..=360 {
            let open_file_path = "test-images/dog1.png";
            let save_file_name = format!("dog_rotate_{}.jpg", degree.to_string());
            let save_file_path = format!("test-output/hue_rotate/{}",save_file_name);
            let _ = hue_rotate_image(open_file_path, &save_file_path, degree);

            let row = format!("<div><img src=\"{}\"/></div>\n", save_file_name);

            str_builder = format!("{}{}", str_builder, row);
        }

        fs::copy("test-images/index.html", "test-output/hue_rotate/index.html").unwrap();

    }

    fn contrast_image(open_file_path: &str, save_file_name: &str, contrast: f32) -> Result<(), String> {
        let mut cmd = Command::cargo_bin(APP_NAME).unwrap();

        cmd.arg("contrast")
            .arg("--openfilepath").arg(open_file_path)
            .arg("--savefilepath").arg(save_file_name)
            .arg("--contrast").arg(contrast.to_string());

        let _ = cmd.unwrap();

        Ok(())
    }

    #[test]
    fn create_contrast_image() {
        let mut cmd = Command::cargo_bin(APP_NAME).unwrap();

        let mut str_builder : String = "".to_owned();

        for contrast_value in 0..=400 {
            let open_file_path = "test-images/dog1.png";
            let save_file_name = format!("dog_contrast_{}.jpg", contrast_value.to_string());
            let save_file_path = format!("test-output/contrast/{}",save_file_name);
            let _ = contrast_image(open_file_path, &save_file_path, contrast_value as f32);
        }

        let _ = cmd.unwrap();
    }
}