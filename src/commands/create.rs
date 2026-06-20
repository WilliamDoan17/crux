use std::fs;
use std::path::{Path, PathBuf};

pub fn run(args: &[String]) {
    match args {
        [input] => {
            create_crux_workspace(input);
        },
        _ => {
            eprintln!("Usage: crux create <name|path>");
            std::process::exit(1);
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

fn create_crux_workspace(input: &String) {
    let destination_path: PathBuf = PathBuf::from(input);
    if has_error(&destination_path) { return }
    
    println!("Ready to create crux workspace {input}");
}
