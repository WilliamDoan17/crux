use std::path::{Path, PathBuf};
use std::process::exit;
use crate::workspace::{create_file, write_file, create_dir};

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

fn has_error(destination_path: &Path) -> bool {
    // handle using blank file path to create crux_workspace
    let is_blank = destination_path.as_os_str().is_empty();
    if is_blank {
        eprintln!("Invalid path: blank path");
        return true
    }

    // handle using file path to create crux_workspace
    if destination_path.is_file() {
        eprintln!("Invalid path: cannot use file path to create crux workspace");
        return true
    }

    let parent_path: &Path = match destination_path.parent() {
        None => {
            // handle creating crux_workspace at root
            eprintln!("Invalid path: using root or . or .. to create crux workspace");
            return true
        },
        Some(n) => n
    };

    // handle parent_path doesn't exists
    let is_parent_exists: bool = parent_path.as_os_str().is_empty() || parent_path.is_dir();

    // handle using . or .. as workspace name
    let workspace_name = match destination_path.file_name() {
        Some(name) => name,
        None => {
            eprintln!("Invalid path: using . or .. as crux workspace name");
            exit(1); 
        }
    };

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

    false
}

fn write_main_boilerplate(main_path: &Path) {
    // define the main_content for writing main boilerplate
    // then call to function write_file to write main boilerplate

    let main_content: &str = r#"#include <bits/stdc++.h>
    
int main() {

}
"#;

    write_file(main_path, main_content);
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
    // write main boilerplate

    let destination_path: PathBuf = PathBuf::from(input);
    if has_error(&destination_path) { exit(1) }
    
    create_dir(&destination_path);
    
    let main_path: PathBuf = destination_path.join("main.cpp");
    create_file(&main_path);

    let marker_path: PathBuf = destination_path.join(".crux");
    create_file(&marker_path);
    
    let tests_path: PathBuf = destination_path.join("tests/");
    create_dir(&tests_path);

    let expected_results_path = destination_path.join("expected_results/");
    create_dir(&expected_results_path);

    let test_results_path = destination_path.join("test_results/");
    create_dir(&test_results_path);
    
    let logs_path = destination_path.join("logs/");
    create_dir(&logs_path);

    write_main_boilerplate(&main_path);
} 
