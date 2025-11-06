use std::io::{Read, Result, Write};

pub fn snickerdoodle(mut i: impl Read, o: &mut impl Write) -> Result<()> {
    let mut bytes = Vec::new();
    i.read_to_end(&mut bytes)?;
    loop {
        if bytes.pop_if(|b| *b == b'\n' || *b == b'\r').is_none() {
            break;
        }
    }
    o.write_all(&bytes)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::snickerdoodle;

    #[test]
    fn test_no_modify() {
        let mut buffer = Vec::new();

        buffer.clear();
        let s = b"";
        snickerdoodle(s.as_slice(), &mut buffer).unwrap();
        assert_eq!(buffer, s);

        buffer.clear();
        let s = b"abc";
        snickerdoodle(s.as_slice(), &mut buffer).unwrap();
        assert_eq!(buffer, s);
    }

    #[test]
    fn test_no_newline() {
        let mut buffer = Vec::new();

        buffer.clear();
        let s = b"abc\n";
        snickerdoodle(s.as_slice(), &mut buffer).unwrap();
        assert_eq!(buffer, b"abc");

        buffer.clear();
        let s = b"abc\r\n";
        snickerdoodle(s.as_slice(), &mut buffer).unwrap();
        assert_eq!(buffer, b"abc");

        buffer.clear();
        let s = b"abc\r";
        snickerdoodle(s.as_slice(), &mut buffer).unwrap();
        assert_eq!(buffer, b"abc");

        buffer.clear();
        let s = b"ab\nc\n";
        snickerdoodle(s.as_slice(), &mut buffer).unwrap();
        assert_eq!(buffer, b"ab\nc");

        buffer.clear();
        let s = b"ab\nc\n";
        snickerdoodle(s.as_slice(), &mut buffer).unwrap();
        assert_eq!(buffer, b"ab\nc");
    }
}
