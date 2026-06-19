use std::env;

mod commands;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    match args.get(1).map(String::as_str) {
        Some("create") => commands::create::run(&args[2..]),
        Some("add-test") => commands::add_test::run(&args[2..]), 
        Some("remove-test") => commands::remove_test::run(&args[2..]),
        Some("update-test") => commands::update_test::run(&args[2..]),
        Some("run") => commands::run::run(&args[2..]),
        Some(unknown) => {
            eprintln!("Unknown command: {unknown}");
            std::process::exit(1);
        }
        None => {
            eprintln!("Usage: crux <command> [args]");
            std::process::exit(1);
        }
    } 
}
