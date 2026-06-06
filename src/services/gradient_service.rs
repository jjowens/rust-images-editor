pub fn gradient_service(save_file_path: &str, width: u32, height: u32, transparency: f32) {
    let mut imgbuf = image::ImageBuffer::new(width, height);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (transparency * x as f32) as u8;
        let g = (transparency * x as f32) as u8;
        let b = (transparency * y as f32) as u8;

        *pixel = image::Rgb([r, g, b]);
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save(save_file_path).unwrap();
}