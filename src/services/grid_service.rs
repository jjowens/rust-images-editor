use image::{GenericImage, GenericImageView, ImageFormat};

pub fn grid_service(open_file_path: &str, save_file_path: &str, rows: &u32, columns: &u32) {
    let mut img = image::open(open_file_path).unwrap();

    let grid_cell_width = img.width() / columns;
    let grid_cell_height = img.height() / rows;

    img.sub_image(0,90, grid_cell_width,grid_cell_height).to_image().save_with_format(save_file_path, ImageFormat::Png).unwrap()
}