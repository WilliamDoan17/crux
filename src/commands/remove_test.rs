pub fn run(args: &[String]) {   
    match args {
        [input, test_num] => {
            println!("Ready to remove test {test_num} from {input}");
        },
        _ => {
            eprintln!("Usage: crux remove-test <name|path> <num>");
            std::process::exit(1);
        }
    };
}
