use std::process::exit;
use crate::workspace::{check_crux_workspace};
use std::path::{PathBuf, Path};
use std::process::Command;
use std::fs;

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
    // - get the test numbers available in tests/ (get_test_numbers) as Vec<i16>
    // - run solution file with stdin and stdout redirection  
        //  - for every test number in the Vec, run run_test
    // - compare and log results 
    
    let path: PathBuf = PathBuf::from(input);

    check_crux_workspace(&path);
    
    compile_solution(&path);

}

fn compile_solution(path: &Path) {
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

fn get_test_numbers(path: &Path) -> Vec<i16> { 
    // get the test numbers available in a crux workspace, return as a Vec<i16>
    // - read the directory
    // - for every entry read:
    //  - if error: 
    //      - Log error, continue
    //  - else:
    //      - if is_file:
    //          if in the format N.in:
    //              - save in the Vec
    //          else: Log error, continue
    //      else: Log error, continue
    //  sort the test_numbers collected and return

    let test_dir_path = path.join("tests/");

    let entries = match fs::read_dir(&test_dir_path) {
        Ok(ent) => ent,
        Err(e) => {
            eprintln!("Error reading test directory at {:?}: {e}", test_dir_path);
            exit(1);
        }
    };

    let mut test_numbers: Vec<i16> = Vec::new();

    for entry in entries {
        match entry {
            Ok(dir_entry) => {
                let entry_path = dir_entry.path();

                if !entry_path.is_file() {
                    eprintln!("{:?} is not a file", entry_path);
                    continue;

                }
    
                if entry_path.extension().and_then(|e| e.to_str()) != Some("in") {
                    eprintln!("{:?} is not a test file (not in N.in) format", entry_path);
                    continue;
                }

                let test_num: i16 = match entry_path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .and_then(|s| s.parse().ok())
                {
                    Some(num) => num,
                    None => {
                        eprintln!("{:?} is not a test file (not in N.in) format", entry_path);
                        continue;
                    }
                };
                
                test_numbers.push(test_num); 
            }, 
            Err(e) => {
                eprintln!("Error reading entry in {:?}: {e}", test_dir_path);
                continue;
            }
        }
    }

    test_numbers.sort();

    test_numbers
}

fn run_test(crux_path: &Path, test_path: &Path) {
    // run_test: run execution binary for test file and log output to test_results/ 
    // use stdin/stdout redirection to run execution binary
}
