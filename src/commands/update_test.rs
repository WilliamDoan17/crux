use std::path::{Path, PathBuf};
use std::process::exit;
use crate::workspace::{write_file, check_crux_workspace, read};
use std::thread::sleep;
use std::time::Duration;

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

            update_test(input, test_number);
        },
        _ => {
            eprintln!("Usage: crux update-test <name|path> <num>");
            exit(1);
        }
    }
}

fn update_test(input: &str, n: i16) {
    // update test case by number
    // check crux workspace
    // open editor and edit for input file
    // open editor and process the output file

    let path: PathBuf = PathBuf::from(input);
    check_crux_workspace(&path);

    check_test_exists(&path, n);

    update_input(&path, n);
    update_output(&path, n);
}

fn check_test_exists(path: &Path, n: i16) {
    // only checks for input file: tests/n.in
    // if doesn't exist, exit the process

    let input_path = path.join(format!("tests/{n}.in"));

    if !input_path.is_file() {
        eprintln!("Couldn't find test number {n}");
        exit(1);
    }
}

fn update_input(path: &Path, n: i16) {
    // update input for test case n
    // 1. read file tests/n.in and save to template
    // 2. open editor with the template of the old n.in 
    //  - upon error: 
    //      - log out error & exit the process
    // 3. save to n.in
    //      - if saved input = blank, don't change the file

    
    let input_path = path.join(format!("tests/{n}.in")); 

    println!("Opening editor for input ({:?})...", input_path);
    sleep(Duration::from_secs(1));    

    let template = read(&input_path);

    let input = match edit::edit(template) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("Error opening editor: {e}");
            exit(1)
        }
    };

    if input.trim().is_empty() {
        println!("Blank input. No changes made.");
        return
    }

    write_file(&input_path, &input);
}

fn update_output(path: &Path, n: i16) {
    // update output file at expected_results/n.out
     
    let output_path = path.join(format!("expected_results/{n}.out")); 

    println!("Open editor for expected output ({:?})...", output_path);
    sleep(Duration::from_secs(1));

    let template = match output_path.is_file() {
        true => read(&output_path),
        false => String::from("// type expected results..."),

    };

    let input = match edit::edit(template) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("Error opening editor: {e}");
            exit(1)
        }
    };
    
    if input.trim().is_empty() {
        if !output_path.is_file() {
            println!("Blank input. Output file not created");
        } else {        
            println!("Blank input. No changes made"); 
        }
        return
    }

    write_file(&output_path, &input);
}

