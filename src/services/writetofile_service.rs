use std::fs::File;
use std::io::prelude::*;
pub fn writetofile_service(filepath: String, content: String)   -> Result<(), String> {
    let file = File::create(filepath);
    let _ = file.unwrap().write_all(content.as_bytes());
    Ok(())
}