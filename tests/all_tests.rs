use zstring::CharDecoder;

#[test]
fn bstr_example() {
  let bytes = *b"a\xF0\x9F\x87z";
  let chars: Vec<char> = CharDecoder::from(bytes.iter().copied()).collect();
  assert_eq!(vec!['a', '\u{FFFD}', 'z'], chars);
}

#[test]
#[cfg(FALSE)]
fn fuzz_found_data() {
  use bstr::ByteSlice;

  // Note(Lokathor): bstr and String::from_utf8_lossy both agree to output this
  // as two unicode replacement characters. Our decoded outputs this as one
  // unicode replacement character. I think the difference is because both of
  // those other things can look at the first byte, see that it indicates a 4
  // byte sequence, check that there's not 4 more bytes possible, and issue a
  // replacement immediately without consuming the second byte. The second byte
  // is a continuation byte, and so it also becomes a replacement character.
  //
  // Since our decoder is written for an iterator with only a single byte of
  // look-ahead we end up not seeing that we'll run out of data until it's too
  // late, and we issue only one total replacement character.
  let bytes = [0b11110101, 0b10101111];

  let s_lossy = String::from_utf8_lossy(&bytes);
  let s_bstr = bytes.chars().collect::<String>();
  assert_eq!(s_lossy, s_bstr); // passes, they agree

  let s_decoded = CharDecoder::from(bytes.iter().copied()).collect::<String>();
  assert_eq!(s_lossy, s_decoded); // fails, we eat too much per replacement.
}
