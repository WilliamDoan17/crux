use std::fs;
use std::fs::File; 
use std::path::Path;
use std::io::Write;
use std::process::exit;

pub fn create_dir(path: &Path) {  
    // creates directory at path
    // if encounters error, log and exit the process (stop the CLI immediately)

    if let Err(e) = fs::create_dir(&path) {
        eprintln!("Failed to create directory {:?}: {e}", path);
        exit(1);
    } 
}

pub fn create_file(path: &Path) { 
    // creates file at path
    // if encounters error, log and exit the process (stop the CLI immediately)

    if let Err(e) = File::create(path) {
        eprintln!("Failed to create file {:?}: {e}", path);
        exit(1);
    }
}

pub fn write_file(path: &Path, content: &str) { 
    // access and write file at path
    // if file already exists, truncate the file and write to it
    // if has not exist yet, just create and write to the file 
    // if encounters error, log and exit the process (stop the CLI immediately)

    let mut file: File = match File::create(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to create or access file {:?}: {e}", path);
            exit(1);
        }
    };

    if let Err(e) = file.write_all(content.as_bytes()) {
        eprintln!("Failed to write to file {:?}, {e}", path);
        exit(1);
    }
} 


