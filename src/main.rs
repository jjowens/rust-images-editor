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
        #[arg(short, long, default_value_t = 0)]
        rows: u32,
        /// Number of columns to split image into
        #[arg(short, long, default_value_t = 0)]
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
    }
}

fn main() {
    println!("Hello, world!");

    let args = Args::parse();

    match args.command {
        Some(Commands::Grid { filepath, rows, columns }) => {
            println!("Filepath {}", filepath);
            println!("Rows: {}", rows);
            println!("Columns: {}", columns)
        },
        Some(Commands::WriteToFile { filepath, content }) => {
            println!("Writing to file");
            let _ = write_to_file(filepath, content);
        },
        Some(Commands::ImageDetails { filepath }) => {
            get_image_details(filepath)
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
