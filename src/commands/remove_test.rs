use std::path::{Path, PathBuf};
use std::process::exit;
use std::io;
use crate::workspace::{delete_file, check_crux_workspace};

pub fn run(args: &[String]) {   
    match args {
        [input, test_num] => {
            let test_number: i16 = match test_num.parse() {
                Ok(num) => num,
                Err(e) => {
                    eprintln!("Error receiving test case number: {e}");
                    exit(1);
                }
            };
            delete_test(input, test_number);
        },
        _ => {
            eprintln!("Usage: crux remove-test <name|path> <num>");
            std::process::exit(1);
        }
    };
}

fn delete_test(input: &str, test_number: i16) {
    // delete a test case by crux path and test_number
    // steps:
    // 1. check if input is a crux workspace folder
    // 2. find the test case number if it exists (find in tests/test_number.in)
    //  - if doesn't exist:
    //      - log "Couldn't find test {test_number}"
    //      - exit
    // 3. get user confirmation for deleting test
    //  - true -> continue 
    //  - false -> exit 
    // 4. delete test input 
    // 5. delete test output

    let path = PathBuf::from(input);
    
    check_crux_workspace(&path);

    check_test_exists(&path, test_number);

    if !get_user_confirmation(test_number) {
        exit(0)
    }

    delete_test_input(&path, test_number);

    delete_test_output(&path, test_number);
}

fn check_test_exists(path: &Path, test_number: i16) {
    // checks if test exists
    // only checks for input file: tests/test_number.in
    // if doesn't exist, exit the process
    // else: continue (return void)
    let test_path = path.join(format!("tests/{test_number}.in"));

    match test_path.exists() {
        false => {
            eprintln!("Couldn't find test number {test_number}");
            exit(1);
        }
        true => {}
    }
}

fn get_user_confirmation(test_number: i16) -> bool {
    // get user confirmation for deleting the file 
    // if receive: y/Y -> true, n/N -> false
    // on unknown response: 
    // log error: "Unknown user response: {response}"

    println!("Are you sure you want to delete test {test_number}? y/n");

    let mut input: String = String::new();  

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to get user confirmation");

    let input_str: &str = input.trim();

    match input_str {
        "y" | "Y" => true,
        "n" | "N" => false,
        unknown => {
            eprintln!("Unknown user response: {unknown}");
            exit(1)
        }
    }
}

fn delete_test_input(path: &Path, test_number: i16) {
    // delete test input for a test in crux workspace 
    // make a call to workspace::delete_file using test_path joined from path and test_number

    let test_input_path = path.join(format!("tests/{test_number}.in"));

    delete_file(&test_input_path);
} 

fn delete_test_output(path: &Path, test_number: i16) {
    // delete test output for a test in crux workspace
    // steps: 
    // 1. look for the .out file (expected_results/{test_number}.out) 
    // 2. if couldn't find it, skip and return
    // 3. if found: delete file using test_path joined from path and test_number

    let test_output_path = path.join(format!("expected_results/{test_number}.out"));

    match test_output_path.exists() {
        false => {},
        true => {
            delete_file(&test_output_path);
        }
    }
}

