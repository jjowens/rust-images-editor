use image::{GenericImage, ImageFormat};
use image::flat::Error;

pub fn grid_service(open_file_path: &str, save_dir: &str, save_file_name: &str, rows: &u32, columns: &u32) {
    let mut img = image::open(open_file_path).unwrap();

    let image_width = img.width();
    let image_height = img.height();
    let grid_cell_width = img.width() / columns;
    let grid_cell_height = img.height() / rows;

    let mut x: u32 = 0;
    let mut y: u32 = 0;

    for row_number in 1..=20 {
        for column_number in 1..=1 {
            let new_file_name = format!("{}-{}-{}.png", save_file_name, row_number, column_number);
            let full_save_path = format!("{}/{}", save_dir, new_file_name);

            //img.sub_image(x,y, grid_cell_width,grid_cell_height).to_image().save_with_format(full_save_path, ImageFormat::Png).unwrap();
            let result = || -> Result<(), Error> {
                img.sub_image(x,y, grid_cell_width,grid_cell_height).to_image().save_with_format(full_save_path, ImageFormat::Png).unwrap();
                Ok(())
            };

            if result().is_err() {
                panic!("Error saving image");
            }

            x += grid_cell_width;
        }
        y += 10;
        x = 0;
    }
    //img.sub_image(0,90, grid_cell_width,grid_cell_height).to_image().save_with_format(save_file_path, ImageFormat::Png).unwrap()
}