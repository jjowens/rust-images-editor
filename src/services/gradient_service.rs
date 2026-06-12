use rand::random_range;

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

pub fn gradientrgba_service(save_file_path: &str, width: u32, height: u32, red: u32, green: u32, blue: u32, alpha: u8, red_y: bool, green_y:bool, blue_y: bool) {
    let mut imgbuf = image::ImageBuffer::new(width, height);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let mut r = (red * x) as u8;
        let mut g = (green * x) as u8;
        let mut b = (blue * x) as u8;

        if red_y == true {
            r = (red * y) as u8;
        }

        if green_y == true {
            g = (green * y) as u8;
        }

        if blue_y == true {
            b = (blue * y) as u8;
        }

        *pixel = image::Rgba([r, g, b, alpha]);
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save(save_file_path).unwrap();
}

pub fn gradientrandom_service(save_file_path: &str, width: u32, height: u32) {
    let mut imgbuf = image::ImageBuffer::new(width, height);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (random_range(1..256) * x) as u8;
        let g = (random_range(1..256) * x) as u8;
        let b = (random_range(1..256) * y) as u8;
        let a = (random_range(1..256) * y) as u8;

        *pixel = image::Rgba([r, g, b, a]);
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save(save_file_path).unwrap();
}

pub fn gradientblock_service(save_file_path: &str, width: u32, height: u32) {
    let mut imgbuf = image::ImageBuffer::new(width, height);

    let x_half = width / 2;
    let y_half = height/ 2;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let mut r = 0u8;
        let mut g = 0u8;
        let mut b = 0u8;
        let a = 255;

        if x <= x_half && y <= y_half {
            r = 255u8;
        }

        if x > x_half && y <= y_half {
            g = 255u8;
        }

        if x > x_half && y > y_half {
            b = 255u8;
        }

        if x < x_half && y > y_half {
            r = 255u8;
            b = 255u8;
        }

        *pixel = image::Rgba([r, g, b, a]);
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save(save_file_path).unwrap();
}