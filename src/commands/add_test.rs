use edit;
use std::process::exit;
use std::path::{Path, PathBuf};
use crate::workspace::{create_file, write_file};

pub fn run(args: &[String]) {
    match args {
        [input] => {
            println!("Ready to add test to crux workspace {input}");
            add_test_to_crux_workspace(input);
        }
        _ => {
            eprintln!("Usage: crux add-test <name|path>");
            std::process::exit(1);
        }
    }
}

fn check_crux_workspace(path: &Path) {
    // check path to see if it is a crux workspace
    // cases:
    // 1. path not found as a directory
    // 2. path is not a crux folder
   
    // path not found as a directory
    if !path.is_dir() {
        eprintln!("Invalid path: no directory found");
        exit(1);
    }

    // path is not a crux folder 
    let marker_path = path.join(".crux");
    if !marker_path.exists() {
        eprintln!("Invalid path: not a crux workspace folder");
    }
}

fn get_test_number(path: &Path) -> i8 {
    // gets the test number for inputting test case
    // operation:
    // - implement a linear search on tests/ folder for the first N that N.in doesn't exist
    // - returns N

    let test_folder_path = path.join("tests/");

    let mut n: i8 = 1; 

    let is_test_number_exists = |n: i8| {
        let test_path = test_folder_path.join(format!("{n}.in"));
        return test_path.is_file()
    }; 
    
    while is_test_number_exists(n) {
        n += 1;
    }

    return n
}

fn open_editor_input(path: &Path, test_number: i8) {
    // opens $EDITOR for typing test input
    // template: type your input...
    // store what user writes in the editor
    // create test_number.in
    // write to test_number.in
    
    let test_path = path.join(format!("tests/{test_number}.in"));
   
    
    create_file(&test_path); 
}

fn add_test_to_crux_workspace(input: &str) {
    // add test to crux workspace 
    // 1. open and check input path if it is a crux workspace folder
    // 2. determine the test_number
    // 3. open input editor and save to tests/test_number.in
    // 4. open output editor and save to expected_results/test_number.out
    
    let path = PathBuf::from(input);

    check_crux_workspace(&path);

    let test_number = get_test_number(&path);

    open_editor_input(&path, test_number);
}
