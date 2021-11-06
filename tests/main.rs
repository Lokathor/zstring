use zstring::ZBytesRef;

#[test]
fn zbytes_basics() {
  let _ = ZBytesRef::try_from("hello\0").unwrap();
  let _ = ZBytesRef::try_from(b"hello\0").unwrap();
  let _ = ZBytesRef::try_from(b"hello\0".as_ref()).unwrap();

  assert_eq!(format!("{}", ZBytesRef::try_from(" A\0").unwrap()), "[32, 65]");
}
