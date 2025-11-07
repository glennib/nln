//! Contains function for the nln binary
//!
//! Exists so we can fuzz it with cargo-fuzz. Don't add this to your cargo dependencies.

use std::io::{BufRead, Result, Write};

/// Moves bytes from the input reader to the output writer, removing any trailing newlines.
///
/// ## Errors
///
/// This function will return an error if the reader cannot be read or the writer cannot be written to.
pub fn snickerdoodle(mut i: impl BufRead, o: &mut impl Write) -> Result<()> {
    // keep newlines that may be in between content
    let mut nlbuf = Vec::new();
    loop {
        let buf = i.fill_buf()?;
        if buf.is_empty() {
            // fill_buf is empty only when EOF is reached
            break;
        }
        let n = buf.len();

        // last char that's not a newline
        let Some(last_not_newline) = buf.iter().rposition(|&b| !is_newline(b)) else {
            // only newlines in buffer, push it all to nlbuf
            nlbuf.extend_from_slice(buf);
            i.consume(n);
            continue;
        };

        // we have actual output, push the newline buffer
        o.write_all(&nlbuf)?;
        nlbuf.clear();

        // push everything up to and including the last byte that's not a newline
        o.write_all(&buf[..=last_not_newline])?;

        // everything after that goes into the newline buffer
        nlbuf.extend_from_slice(&buf[last_not_newline + 1..]);

        i.consume(n);
    }
    o.flush()
}

fn is_newline(b: u8) -> bool {
    b == b'\r' || b == b'\n'
}

#[cfg(test)]
mod tests {
    use super::snickerdoodle;

    #[test]
    fn test_empty() {
        let mut buf = Vec::new();
        snickerdoodle(b"".as_slice(), &mut buf).unwrap();
        assert_eq!(buf, b"");
    }

    #[test]
    fn test_no_change() {
        let mut buf = Vec::new();
        snickerdoodle(b"abc".as_slice(), &mut buf).unwrap();
        assert_eq!(buf, b"abc");
    }

    #[test]
    fn test_trailing_nl() {
        let mut buf = Vec::new();
        snickerdoodle(b"\n".as_slice(), &mut buf).unwrap();
        assert_eq!(str::from_utf8(&buf).unwrap(), "");

        buf.clear();
        snickerdoodle(b"abc\n".as_slice(), &mut buf).unwrap();
        assert_eq!(str::from_utf8(&buf).unwrap(), "abc");
    }

    #[test]
    fn test_trailing_crlf() {
        let mut buf = Vec::new();
        snickerdoodle(b"abc\r\n".as_slice(), &mut buf).unwrap();
        assert_eq!(str::from_utf8(&buf).unwrap(), "abc");
    }

    #[test]
    fn test_trailing_cr() {
        let mut buf = Vec::new();
        snickerdoodle(b"abc\r".as_slice(), &mut buf).unwrap();
        assert_eq!(str::from_utf8(&buf).unwrap(), "abc");
    }

    #[test]
    fn test_trailing_multi_nl() {
        let mut buf = Vec::new();
        snickerdoodle(b"abc\n\n".as_slice(), &mut buf).unwrap();
        assert_eq!(str::from_utf8(&buf).unwrap(), "abc");

        buf.clear();
        snickerdoodle(b"abc\n\n\n".as_slice(), &mut buf).unwrap();
        assert_eq!(str::from_utf8(&buf).unwrap(), "abc");
    }

    #[test]
    fn test_trailing_multi_crlf() {
        let mut buf = Vec::new();
        snickerdoodle(b"abc\r\n\r\n".as_slice(), &mut buf).unwrap();
        assert_eq!(str::from_utf8(&buf).unwrap(), "abc");
    }

    #[test]
    fn test_only_nl() {
        let mut buf = Vec::new();
        snickerdoodle(b"\n\n\n".as_slice(), &mut buf).unwrap();
        assert_eq!(str::from_utf8(&buf).unwrap(), "");
    }

    #[test]
    fn test_only_crlf() {
        let mut buf = Vec::new();
        snickerdoodle(b"\r\n\r\n".as_slice(), &mut buf).unwrap();
        assert_eq!(str::from_utf8(&buf).unwrap(), "");
    }

    #[test]
    fn test_only_cr() {
        let mut buf = Vec::new();
        snickerdoodle(b"\r\r\r".as_slice(), &mut buf).unwrap();
        assert_eq!(str::from_utf8(&buf).unwrap(), "");
    }

    #[test]
    fn test_leading_nl() {
        let mut buf = Vec::new();
        snickerdoodle(b"\nabc".as_slice(), &mut buf).unwrap();
        assert_eq!(str::from_utf8(&buf).unwrap(), "\nabc");
    }

    #[test]
    fn test_leading_multi_nl() {
        let mut buf = Vec::new();
        snickerdoodle(b"\n\nabc".as_slice(), &mut buf).unwrap();
        assert_eq!(str::from_utf8(&buf).unwrap(), "\n\nabc");
    }

    #[test]
    fn test_leading_crlf() {
        let mut buf = Vec::new();
        snickerdoodle(b"\r\nabc".as_slice(), &mut buf).unwrap();
        assert_eq!(str::from_utf8(&buf).unwrap(), "\r\nabc");
    }

    #[test]
    fn test_leading_trailing() {
        let mut buf = Vec::new();
        snickerdoodle(b"\nabc\n".as_slice(), &mut buf).unwrap();
        assert_eq!(str::from_utf8(&buf).unwrap(), "\nabc");
    }

    #[test]
    fn test_leading_trailing_multi() {
        let mut buf = Vec::new();
        snickerdoodle(b"\n\nabc\n\n".as_slice(), &mut buf).unwrap();
        assert_eq!(str::from_utf8(&buf).unwrap(), "\n\nabc");
    }

    #[test]
    fn test_mixed_trailing() {
        let mut buf = Vec::new();
        snickerdoodle(b"abc\n\r\n".as_slice(), &mut buf).unwrap();
        assert_eq!(str::from_utf8(&buf).unwrap(), "abc");
    }

    #[test]
    fn test_mixed_trailing_types() {
        let mut buf = Vec::new();
        snickerdoodle(b"abc\r\n\n\r".as_slice(), &mut buf).unwrap();
        assert_eq!(str::from_utf8(&buf).unwrap(), "abc");
    }

    #[test]
    fn test_middle_nl() {
        let mut buf = Vec::new();
        snickerdoodle(b"ab\nc\n".as_slice(), &mut buf).unwrap();
        assert_eq!(str::from_utf8(&buf).unwrap(), "ab\nc");

        buf.clear();
        snickerdoodle(b"ab\n\nc\n".as_slice(), &mut buf).unwrap();
        assert_eq!(str::from_utf8(&buf).unwrap(), "ab\n\nc");
    }

    #[test]
    fn test_mixed_in_content() {
        let mut buf = Vec::new();
        snickerdoodle(b"a\rb\nc\r\nd\n".as_slice(), &mut buf).unwrap();
        assert_eq!(str::from_utf8(&buf).unwrap(), "a\rb\nc\r\nd");
    }

    #[test]
    fn test_large_trailing() {
        let mut buf = Vec::new();
        let mut input = "x".repeat(100000);
        input.push_str("\n\n\n");
        snickerdoodle(input.as_bytes(), &mut buf).unwrap();
        assert_eq!(str::from_utf8(&buf).unwrap(), "x".repeat(100000));
    }

    #[test]
    fn test_large_middle() {
        let mut buf = Vec::new();
        let mut input = "x".repeat(50000);
        input.push_str("\n\n");
        input.push_str(&"y".repeat(50000));
        input.push_str("\n\n\n");
        snickerdoodle(input.as_bytes(), &mut buf).unwrap();
        let expected = format!("{}\n\n{}", "x".repeat(50000), "y".repeat(50000));
        assert_eq!(str::from_utf8(&buf).unwrap(), expected);
    }
}
