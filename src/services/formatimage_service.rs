use image::{ImageFormat};
use image::imageops::FilterType;

pub fn formatimage_service(open_file_path: &str, save_file_path: &str, image_type: &str) {
    println!("- Get {} image", open_file_path);
    println!("- Saving {} image as {}", save_file_path, image_type);

    let img = image::open(open_file_path).unwrap();

    let image_file_type = match image_type {
        "tiff" => ImageFormat::Tiff,
        "png" => ImageFormat::Png,
        "gif" => ImageFormat::Gif,
        "webp" => ImageFormat::WebP,
        "jpeg" => ImageFormat::Jpeg,
        "jpg" => ImageFormat::Jpeg,
        "bmp" => ImageFormat::Bmp,
        "tga" => ImageFormat::Tga,
        "ico" => ImageFormat::Ico,
        "hdr" => ImageFormat::Hdr,
        "avif" => ImageFormat::Avif,
        _ => ImageFormat::Jpeg,
    };

    if image_file_type == ImageFormat::Ico {
        img.resize_exact(32,32, FilterType::Gaussian).save_with_format(save_file_path, image_file_type).unwrap();
    } else {
        img.save_with_format(save_file_path, image_file_type).unwrap();
    }
}