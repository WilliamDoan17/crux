use edit;
use std::process::exit;
use std::path::{Path, PathBuf};
use crate::workspace::{write_file, check_crux_workspace};

pub fn run(args: &[String]) {
    match args {
        [input] => {
            add_test_to_crux_workspace(input);
        }
        _ => {
            eprintln!("Usage: crux add-test <name|path>");
            std::process::exit(1);
        }
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
        test_path.is_file()
    }; 
    
    while is_test_number_exists(n) {
        n += 1;
    }

    n
}

fn add_input_file(path: &Path, test_number: i8) {
    // creates input file with user input content
    // 1. opens $EDITOR for typing test input
    //  - template: type your input...
    // 2. store what user writes in the editor as input
    //  - if leave blank (input.trim() == ""): log error and exit
    // 3. create & write to test_number.in
    
    let test_input_path = path.join(format!("tests/{test_number}.in"));
    
    let template: &str = "type your input...";

    let input = match edit::edit(template) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Couldn't save input: {e}");
            exit(1);
        }
    };

    if input.trim().is_empty() {
        eprintln!("Error: cannot leave input blank");
        exit(1);
    }

    write_file(&test_input_path, &input);
} 

fn add_output_file(path: &Path, test_number: i8) {
    // add output file to a test by open editor and save to test_number.out
    // 1. open $EDITOR for typing test output
    //  - template: type expected results...
    // 2. store what user type in the editor as input
    //  - if input is empty: 
    //      - warn user with:
    //          "Warning: no expected output saved for test {N}.
    //          crux run will log output without comparison."
    //      - exit  
    // 3. create & write to test_number.out

    let test_output_path = path.join(format!("expected_results/{test_number}.out"));
    
    let template: &str = "type expected results...";

    let input = match edit::edit(template) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Couldn't save expected output: {e}");
            exit(1);
        }
    };
    
    if input.trim().is_empty() {
        let warning_log = format!(r#"Warning: no expected output saved for test {test_number}.
crux run will log output without comparison."#);

        println!("{warning_log}");
        exit(0);
    }
    
    write_file(&test_output_path, &input);
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

    add_input_file(&path, test_number);
    
    add_output_file(&path, test_number);
}
