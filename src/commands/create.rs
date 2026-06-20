use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::Error;
use std::io::Write;
use std::process::exit;

pub fn run(args: &[String]) {
    match args {
        [input] => {
            create_crux_workspace(input);
        },
        _ => {
            eprintln!("Usage: crux create <name|path>");
            exit(1);
        }
    };
}

fn has_error(destination_path: &PathBuf) -> bool {
    // handle using blank file path to create crux_workspace
    let is_blank = destination_path.as_os_str().is_empty();
    if is_blank {
        eprintln!("Invalid path: blank file path");
        return true
    }

    // handle using file path to create crux_workspace
    if destination_path.exists() && !destination_path.is_dir() {
        eprintln!("Invalid path: cannot use file path to create crux workspace");
        return true
    }

    let parent_path: &Path = match destination_path.parent() {
        None => {
            // handle creating crux_workspace at root
            eprintln!("Cannot create crux workspace at root");
            return true
        },
        Some(n) => n
    };

    // handle parent_path doesn't exists
    let is_parent_exists: bool = parent_path.as_os_str().is_empty() || parent_path.is_dir();
    let workspace_name = destination_path.file_name().unwrap();
    if !is_parent_exists {
        eprintln!("Cannot create crux workspace {:?} at parent directory {:?}: No such parent directory", workspace_name, parent_path);
        return true
    }
    
    //handle input destination_path not empty
    let is_empty: bool = destination_path.read_dir().map_or(true, |mut entries| entries.next().is_none());
    if !is_empty {
        eprintln!("Destination path {:?} already exists and is not an empty directory", destination_path);
        return true
    }

    return false
}

fn create_dir(path: &Path) {  
    // creates directory at path
    // if encounters error, log and exit the process (stop the CLI immediately)

    if let Err(e) = fs::create_dir(&path) {
        eprintln!("Failed to create directory {:?}: {e}", path);
        exit(1);
    } 
}

fn create_file(path: &Path) { 
    // creates file at path
    // if encounters error, log and exit the process (stop the CLI immediately)

    if let Err(e) = File::create(path) {
        eprintln!("Failed to create file {:?}: {e}", path) 
    }
}

fn write_to_file(path: &Path, content: &str) -> Result<(), Error> {
    let mut file: File = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn create_crux_workspace(input: &str) {
    // creates a crux workspace at destination_path inputted
    // check for errors using has_error, if true then exit
    // else continues to init crux workspace by steps:
    // 1. main.cpp
    // 2. tests/
    // 3. expected_results/
    // 4. test_results/
    // 5. logs/

    let destination_path: PathBuf = PathBuf::from(input);
    if has_error(&destination_path) { exit(1) }
    
    create_dir(&destination_path);
    
    let main_path: PathBuf = destination_path.join("main.cpp");
    create_file(&main_path);
    
    let tests_path: PathBuf = destination_path.join("tests/");
    create_dir(&tests_path);

    let expected_results_path = destination_path.join("expected_results/");
    create_dir(&expected_results_path);

    let test_results_path = destination_path.join("test_results/");
    create_dir(&test_results_path);
    
    let logs_path = destination_path.join("logs/");
    create_dir(&logs_path);
} 
