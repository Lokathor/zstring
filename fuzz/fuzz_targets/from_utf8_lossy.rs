#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let std_result = std::string::String::from_utf8_lossy(data);
    let zstring_iter = zstring::CharDecoder::from(data.iter().copied());
    assert!(zstring_iter.eq(std_result.chars()))
});
