use clap::{Parser, Subcommand};
use rustimageseditor::services::formatimage_service::formatimage_service;
use rustimageseditor::services::grid_service::grid_service;
use rustimageseditor::services::imagedetails_service::imagedetails_service;
use rustimageseditor::services::writetofile_service::writetofile_service;
use rustimageseditor::services::createicon_service::createicon_service;
use rustimageseditor::services::gradient_service::{gradientrandom_service, gradient_service, gradientblock_service, gradientrgba_service};
use rustimageseditor::services::misc_service::misc_custom_service;

#[derive(Parser)]
#[command(name = "myapp", author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Create an icon file. It creates 16x16 and 32x32 icons based on your image
    CreateIcon {
        /// Set file path to open image
        #[arg(long)]
        openfilepath: String,
        /// Set directory path to save multiple images
        #[arg(long)]
        savedirectory: String,
        /// File name to prefix. No file extension required. Default file extension is ico
        #[arg(long)]
        savefilename: String,
    },
    /// Format your image as a different image.
    FormatImage {
        /// Set file path to open image
        #[arg(long)]
        openfilepath: String,
        /// Set file path to save image
        #[arg(long)]
        savefilepath: String,
        /// Set image type to save image as.  Available image types: bmp, jpg/jpeg, gif, png, tiff, tga, avif, webp
        #[arg(long)]
        imagetype: String
    },
    /// Basic Gradient
    Gradient {
        /// Set file path to save image
        #[arg(long)]
        savefilepath: String,
        /// Set image width
        #[arg(long, default_value_t = 100)]
        width: u32,
        /// Set image height
        #[arg(long, default_value_t = 100)]
        height: u32,
        /// Set transparency multipler for your reds, greens, and blues. Used as a part of a formula
        #[arg(long, default_value_t = 0.5)]
        transparency: f32
    },
    /// Create gradient with blocks of colours. Each quarter has a colour
    GradientBlock {
        /// Set file path to save image
        #[arg(long)]
        savefilepath: String,
        /// Set image width
        #[arg(long, default_value_t = 100)]
        width: u32,
        /// Set image height
        #[arg(long, default_value_t = 100)]
        height: u32
    },
    /// Create gradient with RGB and alpha
    GradientRgba {
        /// Set file path to save image
        #[arg(long)]
        savefilepath: String,
        /// Set image width
        #[arg(long, default_value_t = 100)]
        width: u32,
        /// Set image height
        #[arg(long, default_value_t = 100)]
        height: u32,
        /// Set Red value. Default axis is x
        #[arg(long, default_value_t = 0)]
        red: u32,
        /// Set Green value. Default axis is x
        #[arg(long, default_value_t = 0)]
        green: u32,
        /// Set Blue value. Default axis is y
        #[arg(long, default_value_t = 0)]
        blue: u32,
        /// Set Alpha value
        #[arg(long, default_value_t = 255)]
        alpha: u8,
        /// Use y axis for Red colour to update pixel.
        #[arg(long, default_value_t = false)]
        redy: std::primitive::bool,
        /// Use y axis for Green colour to update pixel.
        #[arg(long, default_value_t = false)]
        greeny:  std::primitive::bool,
        /// Use y axis for Blue colour to update pixel.
        #[arg(long, default_value_t = true)]
        bluey:  std::primitive::bool,
    },
    /// Create gradient with random values for RGB and alpha
    GradientRandom {
        /// Set file path to save image
        #[arg(long)]
        savefilepath: String,
        /// Set image width
        #[arg(long, default_value_t = 100)]
        width: u32,
        /// Set image height
        #[arg(long, default_value_t = 100)]
        height: u32
    },
    /// Cuts images into grid cells. Calculates cell size evenly by dividing width/height by columns/rows set
    Grid {
        /// Set file path to open image
        #[arg(long)]
        openfilepath: String,
        /// Set directory path to save multiple images
        #[arg(long)]
        savedirectory: String,
        /// Set file name to prefix multiple images
        #[arg(long)]
        savefilename: String,
        /// Number of rows to split image into
        #[arg(long, default_value_t = 0)]
        rows: u32,
        /// Number of columns to split image into
        #[arg(long, default_value_t = 0)]
        columns: u32
    },
    /// Get image details. Retrieves width, height, and colour type.
    ImageDetails {
        /// Set file path to open image
        #[arg(long)]
        filepath: String
    },
    /// Custom. WIP
    MiscCustom {
        /// Set file path to save image
        #[arg(long)]
        savefilepath: String,
        /// Set image width
        #[arg(long, default_value_t = 100)]
        width: u32,
        /// Set image height
        #[arg(long, default_value_t = 100)]
        height: u32
    },
    /// Write content to file. It does not append content to existing files.
    WriteToFile {
        /// Set file path to save image
        #[arg(long)]
        filepath: String,
        /// Set content to write to file.
        #[arg(long)]
        content: String,
    },
}

fn main() {
    println!("Images Editor");

    let args = Args::parse();

    match args.command {
        Some(Commands::Grid { openfilepath, savedirectory, savefilename, rows, columns }) => {
            grid_service(&openfilepath, &savedirectory, &savefilename, rows, columns);
        },
        Some(Commands::WriteToFile { filepath, content }) => {
            println!("Writing to file");
            let _ = writetofile_service(filepath, content);
        },
        Some(Commands::ImageDetails { filepath }) => {
            imagedetails_service(&filepath)
        },
        Some(Commands::Gradient { savefilepath, width, height, transparency  }) => {
            gradient_service(savefilepath.as_str(), width, height, transparency)
        },
        Some(Commands::FormatImage { openfilepath, savefilepath, imagetype }) => {
            formatimage_service(&openfilepath, &savefilepath, &imagetype);
        },
        Some(Commands::CreateIcon { openfilepath, savedirectory, savefilename }) => {
            createicon_service(&openfilepath, &savedirectory, &savefilename);
        },
        Some(Commands::GradientRgba { savefilepath, width, height, red, green, blue, alpha , redy   , greeny, bluey }) => {
            gradientrgba_service(savefilepath.as_str(), width, height, red, green, blue, alpha, redy, greeny, bluey);
        },
        Some(Commands::GradientRandom { savefilepath, width, height }) => {
            gradientrandom_service(savefilepath.as_str(), width, height);
        },
        Some(Commands::GradientBlock { savefilepath, width, height }) => {
            gradientblock_service(savefilepath.as_str(), width, height);
        },
        Some(Commands::MiscCustom { savefilepath, width, height }) => {
            misc_custom_service(savefilepath.as_str(), width, height);
        },
        None => {
            std::process::exit(1);
        }
    }
}
