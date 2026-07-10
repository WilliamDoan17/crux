use std::process::exit;
use crate::workspace::{check_crux_workspace};
use std::path::{PathBuf, Path};
use std::process::Command;

pub fn run(args: &[String]) {
    match args {
        [input] => {
            run_crux(input);
        },
        _ => {
            eprintln!("Usage: crux run <name|path>");
            exit(1);
        }
    }
}

fn run_crux(input: &str) {
    // run crux problem solution
    // - check crux workspace to see any error
    // - compile the solution file
    // - run solution file with stdin and stdout redirection
    //  - for every file in tests/
        //  - store all test numbers in a Vec
        //  - if encounter a file that is not a test file (doesn't follow N.in):
        //      - give a warning: Warning: file_name is not a test file, skipped
        //  - for every test number in the Vect, run run_test
    // - compare and log results 
    
    let path: PathBuf = PathBuf::from(input);

    check_crux_workspace(&path);
    
    compile(&path);
}

fn compile(path: &Path) {
    // compile the main.cpp file to bin/solution

    let main_path = path.join("main.cpp");
    
    let executable_path = path.join("bin/solution"); 

    match Command::new("g++")
        .args(["-std=c++17", "-O2", "-Wall", "-o"])
        .arg(&executable_path)
        .arg(&main_path)
        .output()
    {
        Ok(out) if out.status.success() => { println!("Compile finished. Running tests...") },
        Ok(out) => {
            eprintln!("Compilation failed:\n{}", String::from_utf8_lossy(&out.stderr));
            exit(1);
        }
        Err(e) => {
            eprintln!("Failed to run g++: {e}");
            exit(1);
        }
    }
}

fn run_test(crux_path: &Path, test_path: &Path) {
    // run_test: run execution binary for test file and log output to test_results/ 
    // use stdin/stdout redirection to run execution binary
}
