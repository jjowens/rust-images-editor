use image::codecs::avif::ColorSpace;

pub fn colourspace_service(filepath: &str) -> Result<(), String> {
    println!("Get {} image details", filepath);

    let mut img = image::open(filepath).unwrap();

    println!("colorspace {:?}", img.color_space());
    println!("full range {:?}", img.color_space().full_range);
    println!("matrix {:?}", img.color_space().matrix);
    println!("primaries {:?}", img.color_space().primaries);

    img.save(filepath).unwrap();

    Ok(())
}