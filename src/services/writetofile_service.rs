use std::fs::File;
use std::io::prelude::*;
pub fn writetofile_service(filepath: String, content: String)  -> std::io::Result<()> {
    let mut file = File::create(filepath)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}