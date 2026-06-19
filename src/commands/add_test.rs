pub fn run(args: &[String]) {
    match args {
        [input] => {
            println!("Ready to add test to crux workspace {input}");
        }
        _ => {
            eprintln!("Usage: crux add-test <name|path>");
            std::process::exit(1);
        }
    }
}
