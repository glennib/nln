use std::io::{self, BufReader, Write, stdin, stdout};

use nln::snickerdoodle;

fn main() -> io::Result<()> {
    let stdin = BufReader::new(stdin().lock());
    let stdout = stdout();
    let mut stdout = stdout.lock();
    snickerdoodle(stdin, &mut stdout)?;
    stdout.flush()?;
    Ok(())
}
