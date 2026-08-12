use std::path::Path;
use image::{DynamicImage, GenericImage, GenericImageView};

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

pub fn experiment_huerotate_service(open_file_path: String, save_directory_path: String, file_name: String)  -> Result<(), String> {
    println!("Experiment with: {}", open_file_path);

    let mut img = image::open(open_file_path.clone()).unwrap();

    for degree in 1..360 {
        img = img.huerotate(degree);

        let new_save_file_path = [save_directory_path.as_str(), file_name.as_str(), "_", degree.to_string().as_str(), ".png"].concat();
        let _ = img.save(new_save_file_path).unwrap();
    }

    Ok(())
}

pub fn experiment_huerotate_separate_service(open_file_path: String, save_directory_path: String, file_name: String)  -> Result<(), String> {
    println!("Experiment with: {}", open_file_path);

    for degree in 1..360 {
        println!("Degrees: {}", degree);
        let mut img = image::open(open_file_path.clone()).unwrap();
        //img = img.huerotate(degree);
        img = update_image(img.clone(), degree);

        let new_save_file_path = [save_directory_path.as_str(), file_name.as_str(), "_", degree.to_string().as_str(), ".png"].concat();
        let _ = img.save(new_save_file_path).unwrap();
    }

    Ok(())
}

pub fn experiment_blinds_effect_service(open_file_path: String, save_file_path: String)  -> Result<(), String> {
    println!("Blinds experiment with: {}", open_file_path);

    let mut img = image::open(open_file_path).unwrap();

    let mut canvas = image::RgbaImage::new(img.width(), img.height());

    let total_height = img.height() - 100;

    //let subimage = img.sub_image(0,0,img.width(),img.height()).to_image();

    canvas.copy_from(&img.grayscale().to_rgba8(), 0, 0);

    let subimage = img.sub_image(0,25,img.width(),total_height).to_image();
    canvas.copy_from(&subimage, 0, 25);

    let _ = canvas.save(save_file_path).unwrap();

    subimage.save("./test-output/dumdum.png").unwrap();

    Ok(())
}

pub fn experiment_blinds_gradient_service(open_file_path: String, save_file_path: String)  -> Result<(), String> {
    println!("Blinds experiment with: {}", open_file_path);

    let mut img = image::open(open_file_path).unwrap();

    let mut canvas = image::RgbaImage::new(img.width(), img.height());

    canvas.copy_from(&img.grayscale().to_rgba8(), 0, 0);

    let mut subimage = img.sub_image(0,25, img.width(),10).to_image();

    canvas.copy_from(&subimage, 0, 25);

    subimage = img.sub_image(0,40, img.width(),10).to_image();
    canvas.copy_from(&subimage, 0, 40);

    subimage = img.sub_image(0,60, img.width(),25).to_image();
    canvas.copy_from(&subimage, 0, 60);

    subimage = img.sub_image(0,100, img.width(),40).to_image();
    canvas.copy_from(&subimage, 0, 100);

    subimage = img.sub_image(0,150, img.width(),80).to_image();
    canvas.copy_from(&subimage, 0, 150);

    canvas.save(save_file_path).unwrap();

    Ok(())
}
