use image::{DynamicImage, GenericImage, GenericImageView, Luma, SubImage};

pub fn experiment_service(open_file_path: String, save_file_path: String) -> Result<(), String> {
    println!("Experiment with: {}", open_file_path);

    let mut img = image::open(open_file_path).unwrap();

    let _ = img.save(save_file_path);

    Ok(())
}

pub fn experiment_subimage_service(open_file_path: String, save_file_path: String)  -> Result<(), String> {
    println!("Experiment with: {}", open_file_path);

    let mut img = image::open(open_file_path).unwrap();

    //let sub_mage = img.sub_image(0,0,10,10);
    let sub_image = img.sub_image(0,0,100,100).to_image();
    //let sub_view = img.view(0, 0, 100, 100).to_image();

    let _ = sub_image.save(save_file_path).unwrap();

    Ok(())
}

pub fn experiment_warhol_service(open_file_path: String, save_file_path: String, file_name: String)  -> Result<(), String> {
    println!("Andy Warhol experiment with: {}", open_file_path);

    let mut canvas = image::RgbImage::new(1000, 1000);

    let mut img = image::open(open_file_path).unwrap();

    let img1 = update_image(img.clone(), 5);
    canvas.copy_from(&img1.to_rgb8(), 0, 0);

    let img2 =  update_image(img.clone(), 100);
    canvas.copy_from(&img2.to_rgb8(), 425, 0);

    let img3 =  update_image(img.clone(), 230);
    canvas.copy_from(&img3.to_rgb8(), 0, 290);

    let img4 =  update_image(img.clone(), 300);
    canvas.copy_from(&img4.to_rgb8(), 425, 290);

    let _ = canvas.save(save_file_path).unwrap();

    Ok(())
}

fn update_image(mut img: DynamicImage, rotate_val: i32) -> DynamicImage {
    img = img.huerotate(rotate_val);
    img = img.adjust_contrast(5.0);
    img = img.unsharpen(12.0, 0);

    img
}