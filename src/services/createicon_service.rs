use image::{ImageFormat};
use image::imageops::FilterType;

pub fn createicon_service(open_file_path: &str, save_dir_path: &str, savefilename: &str) -> Result<(), String> {
    println!("- Get {} image", open_file_path);
    println!("- Saving icons to {}", save_dir_path);

    let img = image::open(open_file_path).unwrap();

    let save_32x32_filepath = format!("{}/{}32x32.ico", save_dir_path, savefilename);
    let save_16x16_filepath = format!("{}/{}16x16.ico", save_dir_path, savefilename);

    img.resize_exact(32,32, FilterType::Gaussian).save_with_format(save_32x32_filepath, ImageFormat::Ico).unwrap();

    img.resize_exact(16,16, FilterType::Gaussian).save_with_format(save_16x16_filepath, ImageFormat::Ico).unwrap();

    img.

    Ok(())
}