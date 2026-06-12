use rand::random_range;

pub fn misc_custom_service(save_file_path: &str, width: u32, height: u32) {
    // let mut imgbuf = image::ImageBuffer::new(width, height);
    let mut imgbuf = image::ImageBuffer::<image::Rgb<u8>, _>::new(width, height);

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save(save_file_path).unwrap();
}