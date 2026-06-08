use image::{GenericImage, ImageFormat};
use image::flat::Error;

pub fn grid_service(open_file_path: &str, save_dir: &str, save_file_name: &str, rows: u32, columns: u32) {
    println!("- Creating grid image");
    let mut img = image::open(open_file_path).unwrap();

    let image_width = img.width();
    let image_height = img.height();
    let grid_cell_width = img.width() / columns;
    let grid_cell_height = img.height() / rows;

    let mut x: u32 = 0;
    let mut y: u32 = 0;

    println!("- Image dimension: width: {:?}, height: {:?}", image_width, image_height);
    println!("- Initial co-ordinates x: {:?}, y: {:?}", x, y);
    println!("- Grid cell dimensions width: {:?}, height: {:?}", grid_cell_width, grid_cell_height);
    for row_number in 1..=10 {
        for column_number in 1..=10 {
            let new_file_name = format!("{}-{}-{}.png", save_file_name, row_number, column_number);
            let full_save_path = format!("{}/{}", save_dir, new_file_name);

            let temp_cell_width = get_final_size(x,grid_cell_width, image_width);
            let temp_cell_height = get_final_size(y,grid_cell_height, image_height);

            println!("New co-ordinates x: {:?},y: {:?}", x, y);
            println!("New co-ordinates x: {:?},y: {:?}", x, y);
            println!("Temp dimensions width: {:?}, height: {:?}", temp_cell_width, temp_cell_height);

            //img.sub_image(x,y, grid_cell_width,grid_cell_height).to_image().save_with_format(full_save_path, ImageFormat::Png).unwrap();
            let result = || -> Result<(), Error> {
                img.sub_image(x,y, temp_cell_width, temp_cell_height).to_image().save_with_format(full_save_path, ImageFormat::Png).unwrap();
                Ok(())
            };

            if result().is_err() {
                println!("Error saving image");
                panic!("Error saving image");
            }

            x += grid_cell_width;

            if x >= image_width {
                break;
            }
        }
        y += grid_cell_height;
        x = 0;

        if y >= image_height {
            break;
        }
    }
    //img.sub_image(0,90, grid_cell_width,grid_cell_height).to_image().save_with_format(save_file_path, ImageFormat::Png).unwrap()
}

pub fn get_final_size(current_position: u32, cell_size: u32, full_size: u32) -> u32 {
    let mut final_size: u32 = cell_size;

    let total_crop_length = current_position + cell_size;

    if total_crop_length > full_size {
        final_size = total_crop_length - full_size;
    }

    final_size
}