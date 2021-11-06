/// Makes a `ZStr<'static>` value.
#[macro_export]
macro_rules! zstr {
  ($s:literal) => {{
    const STR_WITH_NULL_THAT_SHOULD_NOT_CLASH_WITH_ANY_OUTER_NAME: &str =
      concat!($s, "\0");
    const OUT_ZSTR_THAT_SHOULD_NOT_CLASH_WITH_ANY_OUTER_NAME: $crate::ZStr<
      'static,
    > = unsafe {
      $crate::panic_if_null_byte_detected($s.as_bytes());
      $crate::ZStr::from_non_null_unchecked(
        ::core::ptr::NonNull::new_unchecked(
          STR_WITH_NULL_THAT_SHOULD_NOT_CLASH_WITH_ANY_OUTER_NAME.as_ptr()
            as *mut u8,
        ),
      )
    };
    OUT_ZSTR_THAT_SHOULD_NOT_CLASH_WITH_ANY_OUTER_NAME
  }};
}

#[doc(hidden)]
pub const fn panic_if_null_byte_detected(b: &[u8]) -> &[u8] {
  let mut i = 0;
  let end = b.len();
  while i < end {
    if b[i] == 0 {
      #[allow(unconditional_panic)]
      ["oops"][1];
    }
    i += 1;
  }
  b
}
