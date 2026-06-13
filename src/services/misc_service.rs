use crate::services::helper::{create_centered_square, get_centred_point};

pub fn misc_custom_service(save_file_path: &str, width: u32, height: u32) -> Result<(), String> {
    // let mut imgbuf = image::ImageBuffer::new(width, height);

    let mut imgbuf = image::ImageBuffer::<image::Rgb<u8>, _>::new(width, height);

    // for x_axis in 0..=10 {
    //     for y_axis in 0..11 {
    //         imgbuf.put_pixel(x_axis, y_axis, image::Rgb([255, 255, 255]));
    //     }
    // }
    //
    // for x_axis in 50..=60 {
    //     for y_axis in 50..=60 {
    //         imgbuf.put_pixel(x_axis, y_axis, image::Rgb([255, 0, 0]));
    //     }
    // }

    let vecs = vec![[32,40, 32, 40]];

    for item in vecs.iter() {
        let x= item[0];
        let width = item[1];
        let y= item[2];
        let height = item[3];

        let max_x = x + width;
        let max_y = y + height;

        // COLUMNS
        //imgbuf.put_pixel(item[0], item[2], image::Rgb([255, 0, 0]));
        for current_x in x..=max_x {
            imgbuf.put_pixel(current_x, y, image::Rgb([255, 0, 0]));

            for current_y in y..=max_y {
                imgbuf.put_pixel(current_x, current_y, image::Rgb([255, 0, 0]));
            }
        }
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save(save_file_path).unwrap();

    Ok(())
}

pub fn misc_custom_square_service(save_file_path: &str, width: u32, height: u32) -> Result<(), String> {

    let mut imgbuf = image::ImageBuffer::<image::Rgb<u8>, _>::new(width, height);

    let vecs = vec![[32,40, 32, 40]];

    for item in vecs.iter() {
        let x= item[0];
        let width = item[1];
        let y= item[2];
        let height = item[3];

        let max_x = x + width;
        let max_y = y + height;

        // COLUMNS
        //imgbuf.put_pixel(item[0], item[2], image::Rgb([255, 0, 0]));
        for current_x in x..=max_x {
            imgbuf.put_pixel(current_x, y, image::Rgb([255, 0, 0]));

            for current_y in y..=max_y {
                imgbuf.put_pixel(current_x, current_y, image::Rgb([255, 0, 0]));
            }
        }
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save(save_file_path).unwrap();

    Ok(())
}

pub fn misc_square_centered(save_file_path: &str, image_width: u32, image_height: u32,  square_width: u32, square_height: u32) -> Result<(), String> {

    let mut imgbuf = image::ImageBuffer::<image::Rgb<u8>, _>::new(image_width, image_height);

    let background_vecs = create_centered_square(image_width, image_height, 60, 60);

    for item in background_vecs.iter() {
        imgbuf.put_pixel(item.get_x(), item.get_y(), image::Rgb([255, 255, 0]));
    }

    let vecs = create_centered_square(image_width, image_height, square_width, square_height);

    for item in vecs.iter() {
        imgbuf.put_pixel(item.get_x(), item.get_y(), image::Rgb([255, 0, 0]));
    }

    // HIGHLIGHT CENTRE
    // let centre_point = get_centred_point(image_width, image_height);
    // imgbuf.put_pixel(centre_point.get_x(), centre_point.get_y(), image::Rgb([255, 255, 0]));
    //println!("centred: {},{}", centre_point.get_x(), centre_point.get_y());

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save(save_file_path).unwrap();

    Ok(())
}