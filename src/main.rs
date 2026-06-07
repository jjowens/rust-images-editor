use clap::{Parser, Subcommand};
use rustimageseditor::services::grid_service::grid_service;
use rustimageseditor::services::imagedetails_service::imagedetails_service;
use rustimageseditor::services::gradient_service::gradient_service;
use rustimageseditor::services::writetofile_service::writetofile_service;

#[derive(Parser)]
#[command(name = "myapp", author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Grid {
        /// Open image Filepath
        #[arg(long)]
        openfilepath: String,
        /// Save image to directory
        #[arg(long)]
        savedirectory: String,
        /// File name to prefix
        #[arg(long)]
        savefilename: String,
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
        Some(Commands::Grid { openfilepath, savedirectory, savefilename, rows, columns }) => {
            grid_service(&openfilepath, &savedirectory, &savefilename, &rows, &columns);
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
        None => {
            std::process::exit(1);
        }
    }
}
