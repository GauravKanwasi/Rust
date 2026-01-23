use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

fn main() -> io::Result<()> {
    // Folder and file name
    let folder_name = "data";
    let file_name = "example.txt";

    // Build the full file path: data/example.txt
    let file_path = Path::new(folder_name).join(file_name);

    // Open the file
    let mut file = File::open(&file_path)?;

    // Read file contents into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Print the contents
    println!("Contents of {:?}:\n{}", file_path, contents);

    Ok(())
}
