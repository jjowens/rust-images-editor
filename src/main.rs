use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::prelude::*;
use image::GenericImageView;

#[derive(Parser)]
#[command(name = "myapp", author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Grid {
        /// Image Filepath
        #[arg(long)]
        filepath: String,
        /// Number of rows to split image into
        #[arg(long, default_value_t = 0)]
        rows: u32,
        /// Number of columns to split image into
        #[arg(long, default_value_t = 0)]
        columns: u32
    },
    WriteToFile {
        #[arg(long)]
        filepath: String,
        #[arg(long)]
        content: String,
    },
    ImageDetails {
        #[arg(long)]
        filepath: String
    },
    Gradient {
        #[arg(long)]
        savefilepath: String,
        /// Number of rows to split image into
        #[arg(long, default_value_t = 100)]
        width: u32,
        /// Number of columns to split image into
        #[arg(long, default_value_t = 100)]
        height: u32,
        #[arg(long, default_value_t = 0.5)]
        transparency: f32
    }
}

fn main() {
    println!("Images Editor");

    let args = Args::parse();

    match args.command {
        Some(Commands::Grid { filepath, rows, columns }) => {
            println!("Filepath {}", filepath);
            println!("{:?}", rows);
            println!("{:?}", columns)
        },
        Some(Commands::WriteToFile { filepath, content }) => {
            println!("Writing to file");
            let _ = write_to_file(filepath, content);
        },
        Some(Commands::ImageDetails { filepath }) => {
            get_image_details(filepath)
        },
        Some(Commands::Gradient { savefilepath, width, height, transparency  }) => {
            gradient_generate(savefilepath.as_str(), width, height, transparency)
        },
        None => {
            std::process::exit(1);
        }
    }
}

fn write_to_file(filepath: String, content: String)  -> std::io::Result<()> {
    let mut file = File::create(filepath)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn get_image_details(filepath: String) {
    println!("Get {} image details", filepath);

    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let img = image::open(filepath).unwrap();

    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's `ColorType`.
    println!("{:?}", img.color());
}

fn gradient_generate(save_file_path: &str, width: u32, height: u32, transparency: f32) {
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
