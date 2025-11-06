use std::io::{self, BufReader, Write, stdin, stdout};

use nln::snickerdoodle;

fn main() -> io::Result<()> {
    let stdin = stdin();
    let stdin = stdin.lock();
    let stdin = BufReader::new(stdin);

    let stdout = stdout();
    let mut stdout = stdout.lock();

    snickerdoodle(stdin, &mut stdout)?;
    stdout.flush()?;

    Ok(())
}
