use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let folder_name = "folder.rs";

    if fs::metadata(folder_name).is_err() {
        fs::create_dir(folder_name)?;
        println!("Folder '{}' created successfully.", folder_name);
    } else {
        println!("Folder '{}' already exists.", folder_name);
    }

    Ok(())
}
