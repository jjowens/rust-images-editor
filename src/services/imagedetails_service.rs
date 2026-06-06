use image::GenericImageView;
pub fn imagedetails_service(filepath: &str) {
    println!("Get {} image details", filepath);

    let img = image::open(filepath).unwrap();

    println!("dimensions {:?}", img.dimensions());

    println!("{:?}", img.color());
}