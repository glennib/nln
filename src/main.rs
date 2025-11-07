use std::env;
use std::io::Result;
use std::io::stdin;
use std::io::stdout;
use std::process;

use nln::snickerdoodle;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    // If there are arguments (beyond program name), process them
    if args.len() > 1 {
        match args[1].as_str() {
            "--help" | "-h" => {
                print_help();
                return Ok(());
            }
            "--version" | "-v" => {
                print_version();
                return Ok(());
            }
            _ => {
                eprintln!("Unknown argument: {}", args[1]);
                eprintln!("Use --help for usage information");
                process::exit(1);
            }
        }
    }

    // Normal operation: process stdin
    let stdout = stdout();
    let mut stdout = stdout.lock();
    snickerdoodle(stdin().lock(), &mut stdout)?;
    Ok(())
}

fn print_help() {
    println!("nln {}", env!("CARGO_PKG_VERSION"));
    println!("{}", env!("CARGO_PKG_DESCRIPTION"));
    println!();
    println!("USAGE:");
    println!("    nln [OPTIONS]");
    println!();
    println!("OPTIONS:");
    println!("    -h, --help       Print help information");
    println!("    -v, --version    Print version information");
    println!();
    println!(
        "Reads from stdin and writes to stdout, removing trailing newlines and carriage returns."
    );
}

fn print_version() {
    println!("{}", env!("CARGO_PKG_VERSION"));
}
