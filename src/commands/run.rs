use std::process::exit;
use crate::workspace::{check_crux_workspace, write_file, readlines};
use std::path::{PathBuf, Path};
use std::process::{Command, Stdio};
use std::fs; 
use std::fs::File;
use std::time::Instant;
use chrono::Local;

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
    
    let test_numbers: Vec<i16> = get_test_numbers(&path);

    let mut test_durations: Vec<Option<u128>> = Vec::new();

    for &n in &test_numbers {
        test_durations.push(run_test(&path, n));
    } 
    
    log_results(&path, &test_numbers, &test_durations);
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

fn run_test(path: &Path, n: i16) -> Option<u128> {
    // run_test: run execution binary for test file and log output to test_results/ 
    // use stdin/stdout redirection to run execution binary
    //
    // open the test_file to get ready for binding stdin
    // get output from running the binary:
    //  if error running command:
    //      log error then return
    //  if runtime error:
    //      - log:
    //      Error
    //      "error message"
    //      to N.out
    //      return
    //  else:
    //      write output to N.out and return
    //
    //  - at the same time:
    //      - count elapsed time
    //      - return elapsed time in the running hand

    let test_path = path.join(format!("tests/{n}.in"));

    let test_file = match File::open(&test_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening test file at {:?}: {e}", test_path);
            return None
        }
    }; 

    let result_path = path.join(format!("test_results/{n}.out"));

    let bin_path = path.join("bin/solution");

    let start = Instant::now();

    match Command::new(&bin_path)
        .stdin(Stdio::from(test_file))
        .output() 
    {
        Err(e) => {
            eprintln!("Error running solution binary for test {n}: {e}");
            return None
        }
        Ok(out) => {
            if !out.status.success() {
                let error = String::from_utf8_lossy(&out.stderr);
                write_file(&result_path, &format!("Error:\n{error}"));
            } else {
                let output = String::from_utf8_lossy(&out.stdout);
                write_file(&result_path, &output);
            }
        }
    }

    Some(start.elapsed().as_millis())
}

fn log_results(path: &Path, test_numbers: &[i16], test_durations: &[Option<u128>]) {
    // compare and log out output results:
    // stores log_content
    // every test will get their test duration (Vec<u128>)
    // for every test number (test_numbers[idx])
    // - n = test_numbers[idx]
    // - use test_duration = match (log Error with time n/a and continue, Some(time)) (test_durations[index])
    // - get result lines result_lines (Vec<String>)
    // - store pass_count: i16
    // - if first result line read Error:
        //  - append Error format to log_content
        //  - continue
    // - if no expected_result file:
        // - append Run format to log_content
        // - continue
    // else:
        // - get expected result lines expected_lines (Vec<String>)
        // - traverse every line of result_lines and expected_lines until both has no more line (use loop, match and .get())
        // - use diff_lines to track differing lines:
        //  - upon found diff line (different content, or either is blank):
        //      - append line index to diff_lines
        // if diff_lines.empty():
        //  - append Pass format to log_content
        //  - continue
        // else: 
        //  - write Error heading to log_content
        //  - traverse every diff_lines
        //          - line_number = {i + 1} - use it in logging out the line number
        //          - append expected_lines[i] to expected format (if expected_lines.get(i))
        //          - append result_lines[i] to expected format (if result_lines.get(i))
    // write log_content to logs/timestamp.txt

    let mut log_content: String = String::new();
    
    let test_count = test_numbers.len();

    let mut pass_count: i16 = 0;

    for i in 0..test_count {
        let n = test_numbers[i];

        let test_duration: u128 = match test_durations[i] {
            None => {
                log_content.push_str(&format!("[ERROR] test {n}  (n/a)\n"));
                continue;
            },
           Some(time) => time,
        };
        
        let result_path = path.join(format!("test_results/{n}.out"));
        let result_lines: Vec<String> = readlines(&result_path)
            .into_iter()
            .map(|line| line.trim().to_string())
            .collect();

        if !result_lines.is_empty() && result_lines[0] == "Error:" {
            log_content.push_str(&format!("[ERROR] test {n}  ({test_duration}ms)\n"));

            for line in result_lines {
                log_content.push_str(&format!("  {line}\n"));
            }

            continue;
        }

        let expected_path = path.join(format!("expected_results/{n}.out"));
        
        if !expected_path.is_file() {
            log_content.push_str(&format!("[RUN]   test {n}  ({test_duration}ms)\n"));
           
            for line in result_lines {
                log_content.push_str(&format!("{line}\n"));
            }

            continue;
        }

        let expected_lines: Vec<String> = readlines(&expected_path)
            .into_iter()
            .map(|line| line.trim().to_string())
            .collect();

        let mut diff_lines: Vec<usize> = Vec::new();
        let mut curr_line: usize = 0;
        
        loop {
            let result_line = result_lines.get(curr_line);
            let expected_line = expected_lines.get(curr_line);

            if result_line.is_none() && expected_line.is_none() {
                break;
            }

            if result_line != expected_line {
                diff_lines.push(curr_line);
            }

            curr_line += 1;
        }
        
        if diff_lines.is_empty() {
            log_content.push_str(&format!("[PASS]  test {n}  ({test_duration}ms)\n"));
            pass_count += 1;
            continue;
        }

        log_content.push_str(&format!("[FAIL]  test {n}  ({test_duration}ms)\n"));
        log_content.push_str("--- expected\n");
        log_content.push_str("+++ result\n");

        for diff_line in diff_lines {

            let line_number = diff_line + 1;

            let expected = expected_lines.get(diff_line).map(String::as_str).unwrap_or("");
            let result = result_lines.get(diff_line).map(String::as_str).unwrap_or("");

            log_content.push_str(&format!("  {line_number} -{expected}\n"));
            log_content.push_str(&format!("  {line_number} +{result}\n"));
        }
    }

    let test_count = test_numbers.len();

    log_content.push_str(&format!("\n{pass_count}/{test_count} tests passed\n"));

    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();

    let log_path = path.join("logs").join(format!("{timestamp}.log"));

    write_file(&log_path, &log_content);

    print!("{log_content}");
}
