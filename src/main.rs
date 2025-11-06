use std::io::Result;
use std::io::Write;
use std::io::stdin;
use std::io::stdout;

use nln::snickerdoodle;

fn main() -> Result<()> {
    let stdout = stdout();
    let mut stdout = stdout.lock();
    snickerdoodle(stdin().lock(), &mut stdout)?;
    stdout.flush()?;
    Ok(())
}
