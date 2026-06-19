pub fn run(args: &[String]) {
    match args {
        [input, test_num] => {
            println!("Ready to update test case {test_num} at {input}");
        },
        _ => {
            eprintln!("Usage: crux update-test <name|path> <num>");
            std::process::exit(1);
        }
    }
}
