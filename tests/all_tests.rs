use zstring::{zstr, CharDecoder, ZBytesRef, ZStr};

#[test]
fn test_zbytesref() {
  let _ = ZBytesRef::try_from(b"hello\0").unwrap();
  let _ = ZBytesRef::try_from(b"hello\0".as_ref()).unwrap();

  assert_eq!(format!("{}", ZBytesRef::try_from(b" A\0").unwrap()), "[32, 65]");
}

#[test]
fn test_zstr() {
  const ABC: ZStr<'static> = zstr!("abc");

  assert_eq!(format!("{}", ABC), "abc");
}

#[test]
fn test_char_decoder() {
  const EXAMPLE_TEXT: &str = "$¢ह€한𐍈";
  let i = CharDecoder::from(EXAMPLE_TEXT.as_bytes().iter().copied());
  let s: String = i.collect();
  assert_eq!(EXAMPLE_TEXT, s.as_str());
}

#[test]
#[cfg(feature = "alloc")]
fn test_zstring() {
  use zstring::ZString;

  const ABC: ZStr<'static> = zstr!("abc");

  let zstring = ZString::from(ABC);

  assert_eq!(format!("{}", zstring), "abc");
}

#[test]
#[cfg(feature = "alloc")]
fn test_zbytes() {
  use zstring::ZBytes;

  let a = ZBytes::try_from(vec![1, 2, 3, 0].into_boxed_slice()).unwrap();
  assert!(a.iter().zip(vec![1_u8, 2, 3].iter()).all(|(a, b)| a == b));
}
