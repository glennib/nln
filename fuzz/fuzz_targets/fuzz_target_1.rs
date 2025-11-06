#![no_main]

use libfuzzer_sys::fuzz_target;
use nnl::snickerdoodle;

fuzz_target!(|data: &[u8]| {
    let mut bytes = Vec::new();
    snickerdoodle(data, &mut bytes).unwrap();
    if let Some(&c) = bytes.last() {
        assert_ne!(c, b'\r');
        assert_ne!(c, b'\n');
    }
});
