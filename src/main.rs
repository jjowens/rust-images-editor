use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::prelude::*;

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
    }

}

fn main() {
    println!("Hello, world!");

    let args = Args::parse();

    match args.command {
        Some(Commands::Grid { filepath, rows, columns }) => {

        },
        Some(Commands::WriteToFile { filepath }) => {
            let _ = write_to_file(filepath, "hello".to_string());
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
