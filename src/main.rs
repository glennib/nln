use std::env;
use std::io::Result;
use std::io::stdin;
use std::io::stdout;
use std::process;

use nln::snickerdoodle;

fn main() -> Result<()> {
    // If there are arguments (beyond program name), process them
    if let Some(arg) = env::args().nth(1) {
        match arg.as_str() {
            "--help" | "-h" => {
                print_help();
                return Ok(());
            }
            "--version" | "-v" => {
                print_version();
                return Ok(());
            }
            _ => {
                eprint_unknown_argument(&arg);
                process::exit(1);
            }
        }
    }

    let stdout = stdout();
    let mut stdout = stdout.lock();
    snickerdoodle(stdin().lock(), &mut stdout)?;
    Ok(())
}

#[cold]
fn print_help() {
    let program_name = env::args().next().unwrap_or_else(|| env!("CARGO_PKG_NAME").to_string());
    println!("{} {}", program_name, env!("CARGO_PKG_VERSION"));
    println!("{}", env!("CARGO_PKG_DESCRIPTION"));
    println!();
    println!("USAGE:");
    println!("    {} [OPTIONS]", program_name);
    println!();
    println!("OPTIONS:");
    println!("    -h, --help       Print help information");
    println!("    -v, --version    Print version information");
}

#[cold]
fn print_version() {
    println!("{}", env!("CARGO_PKG_VERSION"));
}

#[cold]
fn eprint_unknown_argument(arg: &str) {
    eprintln!("Unknown argument: {arg}");
    eprintln!("Use --help for usage information");
}
