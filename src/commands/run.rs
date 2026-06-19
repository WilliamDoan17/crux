pub fn run(args: &[String]) {
    match args {
        [input] => {
            println!("Ready to run crux at {input}");
        },
        _ => {
            eprintln!("Usage: crux run <name|path>");
            std::process::exit(1);
        }
    }
}
