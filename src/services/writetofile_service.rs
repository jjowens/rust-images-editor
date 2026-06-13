use std::fs::File;
use std::io::prelude::*;
pub fn writetofile_service(filepath: String, content: String)   -> Result<(), String> {
    let mut file = File::create(filepath);
    file.unwrap().write_all(content.as_bytes());
    Ok(())
}