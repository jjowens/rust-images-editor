use crate::structs::blind_row::BlindRow;
use crate::structs::image_pixel::ImagePixel;

/// Create a list of vectors to make a square
pub fn create_centered_square(image_width: u32, image_height: u32, square_width: u32, square_height: u32) -> Vec<ImagePixel> {
    let x_centered = image_width / 2;
    let y_centered = image_height / 2;

    let x_startpoint = x_centered - (square_width / 2);
    let y_startpoint = y_centered - (square_height / 2);
    let x_endpoint = x_centered + (square_width / 2);
    let y_endpoint = y_centered + (square_height / 2);

    // println!("x: {},y: {}", x_centered, y_centered);
    // println!("startpoints: {},{}", x_startpoint, y_startpoint);
    // println!("endpoints: {},{}", x_endpoint, y_endpoint);

    let mut vecs  = vec![];

    for current_x in x_startpoint..=x_endpoint {
        for current_y in y_startpoint..=y_endpoint {
            vecs.push(ImagePixel::new(current_x, current_y));
        }
    }

    vecs
}

pub fn get_centred_point(image_width: u32, image_height: u32) -> ImagePixel {
    let x_centered = image_width / 2;
    let y_centered = image_height / 2;

    ImagePixel::new(x_centered, y_centered)
}

pub fn get_blinds_rows(image_width: u32, image_height: u32) -> Vec<BlindRow> {
    let mut vecs  : Vec<BlindRow> = vec![];

    vecs.push(BlindRow::new(0,40, image_width, 10));
    vecs.push(BlindRow::new(0,60, image_width, 25));
    vecs.push(BlindRow::new(0,100, image_width, 40));
    vecs.push(BlindRow::new(0,150, image_width, 80));

    vecs
}