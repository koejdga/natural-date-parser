use natural_date_parser::*;
use std::env;

/// CLI interface
fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return Ok(());
    }

    match args[1].as_str() {
        "--help" => print_help(),
        "--credits" => print_credits(),
        date_string => match date_parser::from_string(date_string) {
            Ok(parsed) => println!("{:#?}", parsed),
            Err(e) => eprintln!("Error: {}", e),
        },
    }

    Ok(())
}

fn print_help() {
    println!("Date Parser CLI - Usage:");
    println!(
        "  cargo run <date_string>     Parses a date written in English and displays its components."
    );
    println!("  cargo run -- --help        Displays help information.");
    println!("  cargo run -- --credits     Shows project credits.");
}

fn print_credits() {
    println!("Natural Date Parser by Sofiia Budilova");
}
