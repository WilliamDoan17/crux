pub fn run(args: &[String]) {
    match args {
        [input] => {
            println!("Ready to create crux workspace {input}");
        },
        _ => {
            eprintln!("Usage: crux create <name|path>");
            std::process::exit(1);
        }
    };
}
