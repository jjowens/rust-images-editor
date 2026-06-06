use clap::{Parser, Subcommand};

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
    }
}

fn main() {
    println!("Hello, world!");
}
