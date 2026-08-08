use image::{ImageFormat};

pub fn split_image_service(open_file_path: &str, save_file_name: &str) -> Result<(), String> {
    println!("- Get {} image", open_file_path);
    println!("- Saving image to {}", save_file_name);

    let img = image::open(open_file_path).unwrap();

    //img.resize_exact(32,32, FilterType::Gaussian).save_with_format(save_file_name, ImageFormat::Ico).unwrap();
    //img.grayscale().save_with_format(save_file_name, ImageFormat::Jpeg).unwrap();
    //img.fliph().flipv().save_with_format(save_file_name, ImageFormat::Jpeg).unwrap();
    img.adjust_contrast(25.0)
        .huerotate(300)
        .save_with_format(save_file_name, ImageFormat::Jpeg).unwrap();

    img.huerotate(270)
        .adjust_contrast(45.0)
        .save_with_format("test-output/dog_hue_rotate_45_contrast.jpg", ImageFormat::Jpeg).unwrap();

    img.huerotate(270)
        .adjust_contrast(100.0)
        .save_with_format("test-output/dog_hue_rotate_100_contrast.jpg", ImageFormat::Jpeg).unwrap();

    img.huerotate(270)
        .adjust_contrast(200.0)
        .save_with_format("test-output/dog_hue_rotate_200_contrast.jpg", ImageFormat::Jpeg).unwrap();

    img.huerotate(270)
        .adjust_contrast(255.0)
        .save_with_format("test-output/dog_hue_rotate_255_contrast.jpg", ImageFormat::Jpeg).unwrap();

    img.huerotate(270)
        .adjust_contrast(300.0)
        .save_with_format("test-output/dog_hue_rotate_300_contrast.jpg", ImageFormat::Jpeg).unwrap();


    Ok(())
}

pub fn hue_rotate(open_file_path: &str, save_file_name: &str, rotate: i32) -> Result<(), String> {
    println!("- Hue rotate image {} to {} degrees", open_file_path, rotate);
    println!("- Saving image to {}", save_file_name);

    let img = image::open(open_file_path).unwrap();

    img.huerotate(rotate)
        .save_with_format(save_file_name, ImageFormat::Jpeg).unwrap();

    Ok(())
}

pub fn adjust_contrast(open_file_path: &str, save_file_name: &str, contrast: f32) -> Result<(), String> {
    println!("- Set contrast of {} to image: {}", contrast, open_file_path);
    println!("- Saving image to {}", save_file_name);

    let img = image::open(open_file_path).unwrap();

    img.adjust_contrast(contrast)
        .save_with_format(save_file_name, ImageFormat::Jpeg).unwrap();

    Ok(())
}

