#[cfg(test)]
mod formatimage_test {
    use assert_cmd::Command;
    const APP_NAME: &str = "rustimageseditor";

    fn save_as_image_type(image_type: &str)  {
        let mut cmd = Command::cargo_bin(APP_NAME).unwrap();

        let open_file_path = "test-images/dog1.png";
        let save_file_dir = "test-output";
        let save_file_path = format!("{}/image_{}.{}", save_file_dir, image_type, image_type);

        cmd.arg("format-image")
            .arg("--openfilepath").arg(open_file_path)
            .arg("--savefilepath").arg(save_file_path)
            .arg("--imagetype").arg(image_type);

        let output = cmd.unwrap();
    }

    #[test]
    fn save_image() {
        let list_of_image_types = vec!["tiff", "bmp", "gif", "png", "webp", "jpeg", "jpg", "avif", "ico"];

        for image_type in list_of_image_types {
            let output = save_as_image_type(image_type);
        }
    }
}